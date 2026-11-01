mod alerts;
mod engine;
mod history;
mod i18n;
mod settings;
mod sound;
mod store;
mod tools;
mod tracker;

use alerts::{AlertRow, NextAlert, Scheduler};
use base64::Engine as _;
use engine::{day_of, day_of_ms, Config, Engine, Interval, Kind, Snapshot};
use serde::Serialize;
use settings::{Settings, SettingsFile};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use store::{AppRow, Store};
use tools::{AlarmClock, AlarmRow, Stopwatch, StopwatchView, Timer, TimerView};
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, RunEvent, State, WebviewUrl, WebviewWindowBuilder, Wry};
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;

const STATE_EVENT: &str = "tally://state";
const ASK_RESET_EVENT: &str = "tally://ask-reset";
const ALERT_EVENT: &str = "tally://alert";
/// 테마·작은 타이머 설정이 바뀌었을 때(두 창이 같이 따라가도록)
const SETTINGS_EVENT: &str = "tally://settings";

/// 잠금 순서: engine → store. 다른 잠금은 짧게 따로 잡는다.
struct Core {
    engine: Mutex<Engine>,
    store: Mutex<Store>,
    cfg: Mutex<Config>,
    settings: Mutex<Settings>,
    settings_file: Mutex<SettingsFile>,
    alerts: Mutex<Vec<AlertRow>>,
    scheduler: Mutex<Scheduler>,
    /// 알림에 붙은 자동 휴식(그동안 일시정지)
    brk: Mutex<Option<Break>>,
    timer: Mutex<Timer>,
    stopwatch: Mutex<Stopwatch>,
    alarms: Mutex<Vec<AlarmRow>>,
    /// 등록에 실패한 단축키(다른 프로그램이 이미 쓰는 등)
    shortcut_errors: Mutex<Vec<String>>,
    /// 트레이 '초기화'로 창을 새로 띄웠을 때 확인 화면을 보여줄지
    ask_reset: AtomicBool,
}

impl Core {
    fn record(store: &mut Store, iv: Option<Interval>) {
        if let Some(iv) = iv {
            if let Err(e) = store.push(&iv) {
                eprintln!("구간 기록 실패: {e}");
            }
        }
    }

    /// 추적 목록이 바뀌면 엔진 설정을 다시 만든다.
    fn refresh_cfg(&self) {
        let tracked = self.store.lock().unwrap().tracked().unwrap_or_default();
        let idle_ms = self.settings.lock().unwrap().idle_ms();
        *self.cfg.lock().unwrap() = Config { tracked, idle_ms };
    }

    fn refresh_alerts(&self) {
        let list = self.store.lock().unwrap().list_alerts().unwrap_or_default();
        *self.alerts.lock().unwrap() = list;
    }

    fn lang(&self) -> String {
        self.settings.lock().unwrap().language.clone()
    }

    /// UI로 보내는 상태: 엔진 스냅숏 + 다음 알림 + 휴식 남은 시간
    fn view(&self) -> View {
        let snap = self.engine.lock().unwrap().snapshot();
        let alerts = self.alerts.lock().unwrap().clone();
        let mut next_alert = self.scheduler.lock().unwrap().next_due(
            &alerts,
            snap.session_started_ms,
            snap.elapsed_ms,
            snap.work_ms,
        );
        // 휴식이 끝나면 처음부터 다시 세므로, 휴식 중에는 한 주기를 그대로 보여 준다
        if snap.kind == Kind::Break {
            if let Some(n) = next_alert.as_mut() {
                n.remaining_ms = n.every_min as u64 * 60_000;
            }
        }
        let break_remaining_ms = self
            .brk
            .lock()
            .unwrap()
            .as_ref()
            .map(|b| (b.until_ms - now_ms()).max(0) as u64);
        let now = now_ms();
        let timer = self.timer.lock().unwrap().view(now);
        let stopwatch = self.stopwatch.lock().unwrap().view();
        View { snap, next_alert, break_remaining_ms, timer, stopwatch }
    }

    fn refresh_alarms(&self) {
        let list = self.store.lock().unwrap().list_alarms().unwrap_or_default();
        *self.alarms.lock().unwrap() = list;
    }

    /// 소리 값 정리: "builtin:이름"은 확인만, 파일 경로는 sounds 폴더로 복사, null은 기본값
    fn sound_value(&self, value: Option<String>) -> Result<Option<String>, String> {
        match value {
            Some(v) if v.starts_with("builtin:") => {
                if !sound::BUILTINS.contains(&&v["builtin:".len()..]) {
                    return Err(format!("unknown built-in sound: {v}"));
                }
                Ok(Some(v))
            }
            Some(p) => sound::import(std::path::Path::new(&p)).map(Some).map_err(|e| match e {
                sound::ImportError::BadType => i18n::t(&self.lang(), i18n::K::ErrSoundType).to_string(),
                sound::ImportError::Io(msg) => msg,
            }),
            None => Ok(None),
        }
    }

    /// 지금 세션을 닫고 새로 연다(초기화·세션 종료). 기록은 통계에 남는다.
    fn restart_session(&self, note: Option<&str>) {
        let now = now_ms();
        // 휴식 중이었다면 휴식도 끝난다. 새 세션은 시작을 누를 때까지 멈춰 있다
        self.brk.lock().unwrap().take();
        let mut e = self.engine.lock().unwrap();
        let iv = e.reset(now);
        let mut st = self.store.lock().unwrap();
        Self::record(&mut st, iv);
        let res = st.end_session(now, note).and_then(|_| st.start_session(now));
        if let Err(err) = res {
            eprintln!("세션 기록 실패: {err}");
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct View {
    #[serde(flatten)]
    snap: Snapshot,
    /// 켜진 알림이 없으면 null
    next_alert: Option<NextAlert>,
    /// 휴식 중이면 남은 시간
    break_remaining_ms: Option<u64>,
    timer: TimerView,
    stopwatch: StopwatchView,
}

struct Break {
    until_ms: i64,
    sound: Option<String>,
    volume: f32,
    every_min: u32,
}

struct Tray {
    icon: TrayIcon<Wry>,
    open_item: MenuItem<Wry>,
    mini_item: MenuItem<Wry>,
    pause_item: MenuItem<Wry>,
    reset_item: MenuItem<Wry>,
    quit_item: MenuItem<Wry>,
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

fn err_str(e: impl std::fmt::Display) -> String {
    e.to_string()
}

// ---- 명령: 세션 ----

#[tauri::command]
fn get_state(core: State<Core>) -> View {
    core.view()
}

#[tauri::command]
fn toggle_pause(app: AppHandle) -> View {
    do_toggle_pause(&app)
}

#[tauri::command]
fn reset(app: AppHandle) -> View {
    app.state::<Core>().restart_session(None);
    publish_now(&app)
}

/// 세션 종료: '무슨 작업?' 메모를 남기고 새 세션으로 넘어간다.
#[tauri::command]
fn end_session(app: AppHandle, note: Option<String>) -> View {
    app.state::<Core>().restart_session(note.as_deref());
    publish_now(&app)
}

/// 창이 처음 뜰 때 한 번 묻는다. 트레이에서 '초기화'를 눌러 열린 창이면 true.
#[tauri::command]
fn take_ask_reset(core: State<Core>) -> bool {
    core.ask_reset.swap(false, Ordering::SeqCst)
}

/// 창을 닫으면 웹뷰까지 해제하고 트레이만 남긴다.
#[tauri::command]
fn close_to_tray(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.destroy();
    }
}

// ---- 명령: 추적 프로그램 ----

#[tauri::command]
fn list_apps(core: State<Core>) -> Result<Vec<AppRow>, String> {
    core.store.lock().unwrap().list_apps().map_err(err_str)
}

#[tauri::command]
fn add_app(core: State<Core>, exe: String, label: Option<String>, title_kw: Option<String>) -> Result<AppRow, String> {
    if exe.trim().is_empty() {
        return Err(i18n::t(&core.lang(), i18n::K::ErrExeEmpty).into());
    }
    let row = core
        .store
        .lock()
        .unwrap()
        .add_app(&exe, label.as_deref(), title_kw.as_deref())
        .map_err(err_str)?;
    core.refresh_cfg();
    Ok(row)
}

#[tauri::command]
fn update_app(core: State<Core>, app: AppRow) -> Result<(), String> {
    core.store.lock().unwrap().update_app(&app).map_err(err_str)?;
    core.refresh_cfg();
    Ok(())
}

#[tauri::command]
fn delete_app(core: State<Core>, id: i64) -> Result<(), String> {
    core.store.lock().unwrap().delete_app(id).map_err(err_str)?;
    core.refresh_cfg();
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunningOut {
    exe: String,
    label: String,
    title: String,
    /// data:image/png;base64,...
    icon: Option<String>,
}

/// 지금 창이 떠 있는 프로그램(아이콘 포함). 아이콘 추출이 있어 async(메인 스레드 밖)로 돈다.
#[tauri::command]
async fn running_programs() -> Vec<RunningOut> {
    tracker::running_programs()
        .into_iter()
        .map(|p| RunningOut {
            label: engine::display_name(&p.exe),
            icon: tracker::icon_png(&p.path)
                .map(|b| format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(b))),
            exe: p.exe,
            title: p.title,
        })
        .collect()
}

#[tauri::command]
fn get_onboarded(core: State<Core>) -> bool {
    core.settings.lock().unwrap().onboarded
}

/// 첫 실행 화면에서 고른 프로그램을 등록한다. 비어 있으면 모든 프로그램 = 작업.
#[tauri::command]
fn finish_onboarding(core: State<Core>, exes: Vec<String>) -> Result<(), String> {
    {
        let st = core.store.lock().unwrap();
        for exe in &exes {
            st.add_app(exe, None, None).map_err(err_str)?;
        }
    }
    save_settings(&core, |s| s.onboarded = true);
    core.refresh_cfg();
    Ok(())
}

// ---- 명령: 주기 알림 ----

#[tauri::command]
fn list_alerts(core: State<Core>) -> Vec<AlertRow> {
    core.alerts.lock().unwrap().clone()
}

#[tauri::command]
fn add_alert(app: AppHandle, every_min: u32, basis: String) -> Result<AlertRow, String> {
    let core = app.state::<Core>();
    let row = core.store.lock().unwrap().add_alert(every_min, &basis).map_err(err_str)?;
    core.refresh_alerts();
    publish_now(&app);
    Ok(row)
}

#[tauri::command]
fn update_alert(app: AppHandle, alert: AlertRow) -> Result<(), String> {
    let core = app.state::<Core>();
    core.store.lock().unwrap().update_alert(&alert).map_err(err_str)?;
    core.refresh_alerts();
    publish_now(&app);
    Ok(())
}

#[tauri::command]
fn delete_alert(app: AppHandle, id: i64) -> Result<(), String> {
    let core = app.state::<Core>();
    core.store.lock().unwrap().delete_alert(id).map_err(err_str)?;
    core.refresh_alerts();
    publish_now(&app);
    Ok(())
}

/// 오늘 화면의 "알림 없음"을 눌렀을 때: 꺼진 알림이 있으면 켜고, 없으면 50분·실 작업 기준으로 만든다.
#[tauri::command]
fn quick_enable_alert(app: AppHandle) -> Result<(), String> {
    let core = app.state::<Core>();
    let first = core.alerts.lock().unwrap().first().cloned();
    {
        let st = core.store.lock().unwrap();
        match first {
            Some(a) => st.update_alert(&AlertRow { enabled: true, ..a }).map_err(err_str)?,
            None => {
                st.add_alert(50, "work").map_err(err_str)?;
            }
        }
    }
    core.refresh_alerts();
    publish_now(&app);
    Ok(())
}

/// 알림 소리(which="alert") 또는 휴식 끝 소리(which="break")를 정한다.
/// value: "builtin:이름" | 파일 경로(sounds 폴더로 복사) | null(기본)
#[tauri::command]
fn set_alert_sound(core: State<Core>, id: i64, which: String, value: Option<String>) -> Result<AlertRow, String> {
    let name = core.sound_value(value)?;
    let mut a = core
        .alerts
        .lock()
        .unwrap()
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .ok_or_else(|| i18n::t(&core.lang(), i18n::K::ErrAlertNotFound).to_string())?;
    if which == "break" {
        a.break_sound = name;
    } else {
        a.sound = name;
    }
    core.store.lock().unwrap().update_alert(&a).map_err(err_str)?;
    core.refresh_alerts();
    Ok(a)
}

#[tauri::command]
fn preview_sound(sound: Option<String>, volume: f32) {
    sound::play(sound, volume);
}

fn media_err(lang: &str, e: sound::MediaError) -> String {
    match e {
        sound::MediaError::BadType => i18n::t(lang, i18n::K::ErrMediaType).to_string(),
        sound::MediaError::TooBig => {
            i18n::tf(lang, i18n::K::ErrMediaBig, &[("mb", &sound::MEDIA_MAX_MB.to_string())])
        }
        sound::MediaError::Io(msg) => msg,
    }
}

/// 구간 편집기: 고른 소리·영상 파일을 그대로 넘긴다(해독은 화면이 한다)
#[tauri::command]
async fn read_media(core: State<'_, Core>, path: String) -> Result<tauri::ipc::Response, String> {
    sound::read_media(std::path::Path::new(&path))
        .map(tauri::ipc::Response::new)
        .map_err(|e| media_err(&core.lang(), e))
}

/// 구간 편집기: 잘라 낸 WAV(본문)를 저장하고 전체 경로를 돌려준다. 이름은 x-name 헤더(URL 인코딩)
#[tauri::command]
async fn save_clip(core: State<'_, Core>, request: tauri::ipc::Request<'_>) -> Result<String, String> {
    let tauri::ipc::InvokeBody::Raw(wav) = request.body() else {
        return Err("expected raw body".into());
    };
    let stem = request
        .headers()
        .get("x-name")
        .and_then(|v| v.to_str().ok())
        .map(|v| percent_decode(v))
        .unwrap_or_default();
    sound::save_clip(&stem, wav)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| media_err(&core.lang(), e))
}

/// encodeURIComponent 되돌리기(헤더는 ASCII만 되므로)
fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn notify(app: &AppHandle, title: String, body: String) {
    let _ = app.emit(ALERT_EVENT, &title);
    // 소리는 직접 내므로 Windows 알림음은 끈다
    if let Err(e) = app.notification().builder().title(title).body(body).silent().show() {
        eprintln!("알림 표시 실패: {e}");
    }
}

/// 같은 순간에 여러 개가 울리면 휴식이 가장 긴 것 하나만 알린다.
fn fire_alerts(app: &AppHandle, fired: &[AlertRow], view: &View) {
    let Some(a) = fired.iter().max_by_key(|a| a.break_min) else { return };
    let lang = app.state::<Core>().lang();
    let basis = if a.basis == "elapsed" { i18n::K::Elapsed } else { i18n::K::Work };
    let basis_ms = a.basis_ms(view.snap.elapsed_ms, view.snap.work_ms);
    let title = format!("{} {}", i18n::t(&lang, basis), i18n::minutes(&lang, basis_ms / 60_000));
    let body = if a.break_min > 0 {
        i18n::tf(&lang, i18n::K::AlertBodyBreak, &[("n", &a.break_min.to_string())])
    } else {
        i18n::tf(&lang, i18n::K::AlertBody, &[("n", &a.every_min.to_string())])
    };
    notify(app, title, body);
    sound::play(a.sound.clone(), a.volume);
    if a.break_min > 0 && !matches!(view.snap.kind, Kind::Paused | Kind::Break) {
        start_break(app, a);
    }
}

/// 자동 휴식: 실 작업만 멈추고(경과는 흐름) 끝나는 시각을 기억한다. 끝나면 루프가 end_break를 부른다.
fn start_break(app: &AppHandle, a: &AlertRow) {
    let core = app.state::<Core>();
    {
        let mut e = core.engine.lock().unwrap();
        let iv = e.set_break(now_ms(), true);
        Core::record(&mut core.store.lock().unwrap(), iv);
    }
    *core.brk.lock().unwrap() = Some(Break {
        until_ms: now_ms() + a.break_min as i64 * 60_000,
        sound: a.break_sound.clone(),
        volume: a.volume,
        every_min: a.every_min,
    });
}

/// 휴식을 끝내고 실 작업을 다시 센다. 알림은 지금부터 다시 N분을 센다.
/// 시간이 다 돼서 끝나면(sound=true) 휴식 끝 소리와 알림을 낸다.
fn finish_break(app: &AppHandle) -> Option<Break> {
    let core = app.state::<Core>();
    let b = core.brk.lock().unwrap().take()?;
    let snap = {
        let mut e = core.engine.lock().unwrap();
        let iv = e.set_break(now_ms(), false);
        Core::record(&mut core.store.lock().unwrap(), iv);
        e.snapshot()
    };
    let alerts = core.alerts.lock().unwrap().clone();
    core.scheduler
        .lock()
        .unwrap()
        .rearm(&alerts, snap.session_started_ms, snap.elapsed_ms, snap.work_ms);
    Some(b)
}

fn end_break(app: &AppHandle) {
    let core = app.state::<Core>();
    let Some(b) = finish_break(app) else { return };
    let lang = core.lang();
    notify(
        app,
        i18n::t(&lang, i18n::K::BreakEndTitle).to_string(),
        i18n::tf(&lang, i18n::K::BreakEndBody, &[("n", &b.every_min.to_string())]),
    );
    sound::play(Some(b.sound.unwrap_or_else(|| "builtin:rise".into())), b.volume);
}

// ---- 명령: 도구(타이머·스톱워치·알람) ----

fn tool_now(app: &AppHandle, f: impl FnOnce(&Core, i64)) -> View {
    let core = app.state::<Core>();
    f(&core, now_ms());
    publish_now(app)
}

#[tauri::command]
fn timer_set(app: AppHandle, ms: u64) -> View {
    let ms = ms.clamp(1_000, 24 * 3_600_000);
    let core = app.state::<Core>();
    save_settings(&core, |s| s.timer.last_ms = ms);
    tool_now(&app, |c, _| c.timer.lock().unwrap().set(ms))
}

#[tauri::command]
fn timer_start(app: AppHandle) -> View {
    tool_now(&app, |c, now| c.timer.lock().unwrap().start(now))
}

#[tauri::command]
fn timer_pause(app: AppHandle) -> View {
    tool_now(&app, |c, now| c.timer.lock().unwrap().pause(now))
}

#[tauri::command]
fn timer_reset(app: AppHandle) -> View {
    tool_now(&app, |c, _| c.timer.lock().unwrap().reset())
}

#[tauri::command]
fn sw_start(app: AppHandle) -> View {
    tool_now(&app, |c, now| c.stopwatch.lock().unwrap().start(now))
}

#[tauri::command]
fn sw_stop(app: AppHandle) -> View {
    tool_now(&app, |c, now| c.stopwatch.lock().unwrap().stop(now))
}

#[tauri::command]
fn sw_lap(app: AppHandle) -> View {
    tool_now(&app, |c, now| c.stopwatch.lock().unwrap().lap(now))
}

#[tauri::command]
fn sw_reset(app: AppHandle) -> View {
    tool_now(&app, |c, _| c.stopwatch.lock().unwrap().reset())
}

/// 프리셋(분)과 볼륨
#[tauri::command]
fn set_timer_settings(core: State<Core>, presets: Vec<u32>, volume: f32) {
    let presets: Vec<u32> = presets.into_iter().map(|m| m.clamp(1, 24 * 60)).take(5).collect();
    save_settings(&core, |s| {
        if !presets.is_empty() {
            s.timer.presets = presets;
        }
        s.timer.volume = volume.clamp(0.0, 1.0);
    });
}

#[tauri::command]
fn set_timer_sound(core: State<Core>, value: Option<String>) -> Result<(), String> {
    let name = core.sound_value(value)?;
    save_settings(&core, |s| s.timer.sound = name);
    Ok(())
}

#[tauri::command]
fn list_alarms(core: State<Core>) -> Vec<AlarmRow> {
    core.alarms.lock().unwrap().clone()
}

#[tauri::command]
fn add_alarm(core: State<Core>, time_hm: String, weekdays: u8, label: Option<String>) -> Result<AlarmRow, String> {
    let hm = tools::norm_hm(&time_hm).ok_or_else(|| i18n::t(&core.lang(), i18n::K::ErrTime).to_string())?;
    let row = core.store.lock().unwrap().add_alarm(&hm, weekdays, label.as_deref()).map_err(err_str)?;
    core.refresh_alarms();
    Ok(row)
}

#[tauri::command]
fn update_alarm(core: State<Core>, alarm: AlarmRow) -> Result<(), String> {
    let time_hm = tools::norm_hm(&alarm.time_hm).ok_or_else(|| i18n::t(&core.lang(), i18n::K::ErrTime).to_string())?;
    core.store.lock().unwrap().update_alarm(&AlarmRow { time_hm, ..alarm }).map_err(err_str)?;
    core.refresh_alarms();
    Ok(())
}

#[tauri::command]
fn delete_alarm(core: State<Core>, id: i64) -> Result<(), String> {
    core.store.lock().unwrap().delete_alarm(id).map_err(err_str)?;
    core.refresh_alarms();
    Ok(())
}

#[tauri::command]
fn set_alarm_sound(core: State<Core>, id: i64, value: Option<String>) -> Result<(), String> {
    let sound = core.sound_value(value)?;
    let a = core.alarms.lock().unwrap().iter().find(|a| a.id == id).cloned();
    if let Some(a) = a {
        core.store.lock().unwrap().update_alarm(&AlarmRow { sound, ..a }).map_err(err_str)?;
    }
    core.refresh_alarms();
    Ok(())
}

/// 1초마다: 타이머가 끝났는지, 지금 울릴 알람이 있는지
fn check_tools(app: &AppHandle, clock: &mut AlarmClock) {
    let core = app.state::<Core>();
    let lang = core.lang();
    let duration = {
        let mut t = core.timer.lock().unwrap();
        t.check_done(now_ms()).then(|| t.view(0).duration_ms)
    };
    if let Some(ms) = duration {
        let ts = core.settings.lock().unwrap().timer.clone();
        let time = i18n::minutes(&lang, (ms / 60_000).max(1));
        notify(
            app,
            i18n::t(&lang, i18n::K::TimerDoneTitle).to_string(),
            i18n::tf(&lang, i18n::K::TimerDoneBody, &[("time", &time)]),
        );
        sound::play(Some(ts.sound.unwrap_or_else(|| "builtin:chime".into())), ts.volume);
    }
    let alarms = core.alarms.lock().unwrap().clone();
    let due = clock.due(&alarms, chrono::Local::now().naive_local());
    for a in &due {
        notify(
            app,
            i18n::tf(&lang, i18n::K::AlarmTitle, &[("time", &a.time_hm)]),
            a.label.clone().unwrap_or_default(),
        );
        sound::play(Some(a.sound.clone().unwrap_or_else(|| "builtin:bell".into())), 0.9);
        // 한 번만 울리는 알람은 끈다
        if a.weekdays == 0 {
            let _ = core.store.lock().unwrap().update_alarm(&AlarmRow { enabled: false, ..a.clone() });
        }
    }
    if due.iter().any(|a| a.weekdays == 0) {
        core.refresh_alarms();
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TimerSettingsOut {
    presets: Vec<u32>,
    sound: Option<String>,
    volume: f32,
}

#[tauri::command]
fn get_timer_settings(core: State<Core>) -> TimerSettingsOut {
    let t = core.settings.lock().unwrap().timer.clone();
    TimerSettingsOut { presets: t.presets, sound: t.sound, volume: t.volume }
}

// ---- 명령: 기록 ----

/// 조회 전에 진행 중인 구간을 디스크에 써서 지금까지가 보이게 한다
fn with_flushed<T>(core: &Core, f: impl FnOnce(&Store) -> rusqlite::Result<T>) -> Result<T, String> {
    let mut st = core.store.lock().unwrap();
    st.flush().map_err(err_str)?;
    f(&st).map_err(err_str)
}

#[tauri::command]
fn history_days(core: State<Core>, from: String, to: String) -> Result<Vec<history::DayTotal>, String> {
    with_flushed(&core, |st| history::day_totals(st, &from, &to))
}

#[tauri::command]
fn history_day(core: State<Core>, day: String) -> Result<history::DayDetail, String> {
    with_flushed(&core, |st| history::day_detail(st, &day))
}

#[tauri::command]
fn set_session_note(core: State<Core>, id: i64, note: Option<String>) -> Result<(), String> {
    with_flushed(&core, |st| history::set_session_note(st, id, note.as_deref()))
}

/// from..=to 기간의 구간을 CSV로 저장하고 줄 수를 돌려준다
#[tauri::command]
fn export_csv(core: State<Core>, path: String, from: String, to: String) -> Result<usize, String> {
    use chrono::TimeZone;
    let local = |ms: i64| {
        chrono::Local
            .timestamp_millis_opt(ms)
            .single()
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default()
    };
    let (csv, n) = with_flushed(&core, |st| history::export_csv(st, &from, &to, local))?;
    std::fs::write(&path, csv).map_err(err_str)?;
    Ok(n)
}

// ---- 명령: 설정(자리 비움, 단축키) ----

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SettingsOut {
    idle_minutes: f64,
    shortcuts: settings::Shortcuts,
    shortcut_errors: Vec<String>,
    language: String,
    day_start_hour: u32,
    theme: String,
    mini: settings::MiniSettings,
    autostart: bool,
}

#[tauri::command]
fn get_settings(app: AppHandle) -> SettingsOut {
    let core = app.state::<Core>();
    let s = core.settings.lock().unwrap().clone();
    let shortcut_errors = core.shortcut_errors.lock().unwrap().clone();
    SettingsOut {
        idle_minutes: s.idle_minutes,
        shortcuts: s.shortcuts,
        shortcut_errors,
        language: s.language,
        day_start_hour: s.day_start_hour,
        theme: s.theme,
        mini: s.mini,
        autostart: app.autolaunch().is_enabled().unwrap_or(false),
    }
}

// ---- 명령: 테마·시작 프로그램·작은 타이머 ----

#[tauri::command]
fn set_theme(app: AppHandle, theme: String) {
    let theme = match theme.as_str() {
        "light" | "dark" => theme,
        _ => "system".into(),
    };
    save_settings(&app.state::<Core>(), |s| s.theme = theme);
    let _ = app.emit(SETTINGS_EVENT, ());
}

/// Windows 시작 시 트레이로 실행(--tray)
#[tauri::command]
fn set_autostart(app: AppHandle, on: bool) -> Result<bool, String> {
    let al = app.autolaunch();
    if on { al.enable() } else { al.disable() }.map_err(err_str)?;
    al.is_enabled().map_err(err_str)
}

/// 작은 타이머 설정을 바꾸고 창에 바로 적용한다(위치는 save_mini_pos)
#[tauri::command]
fn set_mini(app: AppHandle, mini: settings::MiniSettings) {
    save_settings(&app.state::<Core>(), |s| {
        let (x, y) = (s.mini.x, s.mini.y);
        s.mini = settings::MiniSettings {
            opacity: mini.opacity.clamp(0.6, 1.0),
            x,
            y,
            ..mini
        };
    });
    apply_mini(&app);
    let _ = app.emit(SETTINGS_EVENT, ());
}

#[tauri::command]
fn save_mini_pos(core: State<Core>, x: i32, y: i32) {
    save_settings(&core, |s| {
        s.mini.x = Some(x);
        s.mini.y = Some(y);
    });
}

// 창을 만드는 명령은 async여야 한다(Windows에서 동기 명령 안에서 웹뷰를 만들면 초기화가 멈춤)
#[tauri::command]
async fn open_mini(app: AppHandle) {
    show_mini(&app);
}

#[tauri::command]
fn close_mini(app: AppHandle) {
    if let Some(w) = app.get_webview_window("mini") {
        let _ = w.destroy();
    }
}

/// 본창으로 돌아가기(작은 타이머 더블클릭)
#[tauri::command]
async fn back_to_main(app: AppHandle) {
    if let Some(w) = app.get_webview_window("mini") {
        let _ = w.destroy();
    }
    show_main(&app);
}

/// 본창을 닫고 작은 타이머만 남긴다(헤더의 "작은 타이머로 접기")
#[tauri::command]
async fn fold_to_mini(app: AppHandle) {
    show_mini(&app);
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.destroy();
    }
}

/// 트레이 "작은 타이머": 접혀 있으면 펼치고, 펼쳐져 있으면 닫고, 없으면 연다
fn toggle_mini(app: &AppHandle) {
    let collapsed = app.state::<Core>().settings.lock().unwrap().mini.collapsed;
    match app.get_webview_window("mini") {
        Some(_) if collapsed => show_mini(app),
        Some(w) => {
            let _ = w.destroy();
        }
        None => show_mini(app),
    }
}

/// 작은 타이머를 연다. 이미 있으면 칩을 펼치고 앞으로 가져온다
fn show_mini(app: &AppHandle) {
    let core = app.state::<Core>();
    if core.settings.lock().unwrap().mini.collapsed {
        save_settings(&core, |s| s.mini.collapsed = false);
        let _ = app.emit(SETTINGS_EVENT, ());
    }
    if let Some(w) = app.get_webview_window("mini") {
        apply_mini(app);
        let _ = w.unminimize();
        let _ = w.show();
        return;
    }
    let m = core.settings.lock().unwrap().mini.clone();
    let (w, h) = m.window_size();
    // 작업 표시줄에도 보이게(본창과 구분되는 제목)
    let title = format!("Tally · {}", i18n::t(&core.lang(), i18n::K::TrayMini));
    let built = WebviewWindowBuilder::new(app, "mini", WebviewUrl::default())
        .title(title)
        .inner_size(w, h)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .resizable(false)
        .always_on_top(m.pinned)
        .focused(false)
        .visible(false)
        .build();
    match built {
        Ok(win) => {
            // 저장된 위치가 지금 화면 안에 있을 때만 쓴다(모니터를 뺐을 수 있으니)
            let pos = match (m.x, m.y) {
                (Some(x), Some(y)) if on_screen(&win, x, y) => Some(tauri::PhysicalPosition::new(x, y)),
                _ => default_mini_pos(&win),
            };
            if let Some(p) = pos {
                let _ = win.set_position(p);
            }
            let _ = win.show();
        }
        Err(e) => eprintln!("작은 타이머 창 만들기 실패: {e}"),
    }
}

fn apply_mini(app: &AppHandle) {
    let Some(win) = app.get_webview_window("mini") else { return };
    let m = app.state::<Core>().settings.lock().unwrap().mini.clone();
    let (w, h) = m.window_size();
    let _ = win.set_size(tauri::LogicalSize::new(w, h));
    let _ = win.set_always_on_top(m.pinned);
}

fn on_screen(win: &tauri::WebviewWindow, x: i32, y: i32) -> bool {
    win.available_monitors().map_or(false, |ms| {
        ms.iter().any(|m| {
            let (p, s) = (m.position(), m.size());
            x >= p.x && y >= p.y && x < p.x + s.width as i32 - 40 && y < p.y + s.height as i32 - 20
        })
    })
}

/// 처음엔 주 모니터 오른쪽 위
fn default_mini_pos(win: &tauri::WebviewWindow) -> Option<tauri::PhysicalPosition<i32>> {
    let m = win.primary_monitor().ok()??;
    let (p, s, k) = (m.position(), m.size(), m.scale_factor());
    Some(tauri::PhysicalPosition::new(
        p.x + s.width as i32 - (220.0 * k) as i32,
        p.y + (48.0 * k) as i32,
    ))
}

/// 언어를 바꾸고 트레이 문구를 다시 쓴다.
#[tauri::command]
fn set_language(app: AppHandle, language: String) {
    let core = app.state::<Core>();
    let lang = match language.as_str() {
        "en" | "ja" | "zh" => language,
        _ => "ko".into(),
    };
    save_settings(&core, |s| s.language = lang);
    if let Some(tray) = app.try_state::<Tray>() {
        let lang = core.lang();
        let _ = tray.open_item.set_text(i18n::t(&lang, i18n::K::TrayOpen));
        let _ = tray.mini_item.set_text(i18n::t(&lang, i18n::K::TrayMini));
        let _ = tray.reset_item.set_text(i18n::t(&lang, i18n::K::TrayReset));
        let _ = tray.quit_item.set_text(i18n::t(&lang, i18n::K::TrayQuit));
        // 일시정지 문구·툴팁은 다음 갱신 때 다시 쓰도록
        app.state::<Mutex<TrayShown>>().lock().unwrap().kind = None;
    }
    publish_now(&app);
}

fn save_settings(core: &Core, f: impl FnOnce(&mut Settings)) {
    let s = {
        let mut s = core.settings.lock().unwrap();
        f(&mut s);
        s.clone()
    };
    core.settings_file.lock().unwrap().save(&s);
}

#[tauri::command]
fn set_idle_minutes(core: State<Core>, minutes: f64) {
    save_settings(&core, |s| s.idle_minutes = minutes.clamp(1.0, 60.0));
    core.refresh_cfg();
}

/// which = "pause" | "reset", accel 예: "Ctrl+Alt+P". 빈 문자열이면 끈다.
#[tauri::command]
fn set_shortcut(app: AppHandle, which: String, accel: String) -> Result<SettingsOut, String> {
    let accel = accel.trim().to_string();
    let core = app.state::<Core>();
    if !accel.is_empty() {
        accel
            .parse::<Shortcut>()
            .map_err(|e| i18n::tf(&core.lang(), i18n::K::ErrShortcut, &[("e", &e.to_string())]))?;
    }
    save_settings(&core, |s| match which.as_str() {
        "pause" => s.shortcuts.pause = accel,
        _ => s.shortcuts.reset = accel,
    });
    apply_shortcuts(&app);
    Ok(get_settings(app.clone()))
}

/// 설정의 단축키를 다시 등록한다. 실패한 것은 shortcut_errors에 남긴다.
fn apply_shortcuts(app: &AppHandle) {
    let core = app.state::<Core>();
    let sc = core.settings.lock().unwrap().shortcuts.clone();
    let gs = app.global_shortcut();
    let _ = gs.unregister_all();
    let mut errors = Vec::new();
    for (name, accel) in [("pause", &sc.pause), ("reset", &sc.reset)] {
        if accel.is_empty() {
            continue;
        }
        let res = accel
            .parse::<Shortcut>()
            .map_err(err_str)
            .and_then(|s| gs.register(s).map_err(err_str));
        if let Err(e) = res {
            eprintln!("단축키 {accel} 등록 실패: {e}");
            errors.push(name.to_string());
        }
    }
    *core.shortcut_errors.lock().unwrap() = errors;
}

fn on_shortcut(app: &AppHandle, shortcut: &Shortcut, state: ShortcutState) {
    if state != ShortcutState::Pressed {
        return;
    }
    let sc = app.state::<Core>().settings.lock().unwrap().shortcuts.clone();
    let is = |accel: &str| accel.parse::<Shortcut>().map_or(false, |s| &s == shortcut);
    if is(&sc.pause) {
        do_toggle_pause(app);
    } else if is(&sc.reset) {
        ask_reset(app);
    }
}

/// 초기화는 확인을 한 번 거친다: 창을 띄우고 확인 줄을 연다
fn ask_reset(app: &AppHandle) {
    let core = app.state::<Core>();
    core.ask_reset.store(true, Ordering::SeqCst);
    let existed = app.get_webview_window("main").is_some();
    show_main(app);
    if existed {
        core.ask_reset.store(false, Ordering::SeqCst);
        let _ = app.emit(ASK_RESET_EVENT, ());
    }
}

fn do_toggle_pause(app: &AppHandle) -> View {
    let core = app.state::<Core>();
    // 휴식 중에 누르면 휴식을 끝내고 바로 다시 센다(소리 없이)
    if finish_break(app).is_some() {
        return publish_now(app);
    }
    {
        let mut e = core.engine.lock().unwrap();
        let iv = e.toggle_pause(now_ms());
        Core::record(&mut core.store.lock().unwrap(), iv);
    }
    publish_now(app)
}

fn publish(app: &AppHandle, view: &View) {
    let _ = app.emit(STATE_EVENT, view);
    update_tray(app, view);
}

fn publish_now(app: &AppHandle) -> View {
    let view = app.state::<Core>().view();
    publish(app, &view);
    view
}

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
        return;
    }
    let built = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Tally")
        .inner_size(400.0, 600.0)
        .min_inner_size(360.0, 480.0)
        .decorations(false)
        .resizable(true)
        .center()
        .build();
    if let Err(e) = built {
        eprintln!("창 만들기 실패: {e}");
    }
}

// ---- 트레이 ----

const ICON_WORK_DARK: &[u8] = include_bytes!("../icons/tray/tray-working-dark.png");
const ICON_WORK_LIGHT: &[u8] = include_bytes!("../icons/tray/tray-working-light.png");
const ICON_IDLE_DARK: &[u8] = include_bytes!("../icons/tray/tray-idle-dark.png");
const ICON_IDLE_LIGHT: &[u8] = include_bytes!("../icons/tray/tray-idle-light.png");
const ICON_PAUSE_DARK: &[u8] = include_bytes!("../icons/tray/tray-paused-dark.png");
const ICON_PAUSE_LIGHT: &[u8] = include_bytes!("../icons/tray/tray-paused-light.png");

/// 작업 표시줄이 라이트면 먹색 아이콘(-light), 다크면 종이색 아이콘(-dark)
fn tray_icon_bytes(kind: Kind, light_taskbar: bool) -> &'static [u8] {
    match (kind, light_taskbar) {
        (Kind::Work, false) => ICON_WORK_DARK,
        (Kind::Work, true) => ICON_WORK_LIGHT,
        (Kind::Paused, false) => ICON_PAUSE_DARK,
        (Kind::Paused, true) => ICON_PAUSE_LIGHT,
        (_, false) => ICON_IDLE_DARK,
        (_, true) => ICON_IDLE_LIGHT,
    }
}

fn status_text(lang: &str, view: &View) -> String {
    use i18n::K;
    match view.snap.kind {
        Kind::Work => i18n::tf(lang, K::StatusWork, &[("app", view.snap.app.as_deref().unwrap_or(""))]),
        Kind::Other => i18n::t(lang, K::StatusOther).into(),
        Kind::Idle => i18n::t(lang, K::StatusIdle).into(),
        Kind::Break => i18n::t(lang, K::StatusBreak).into(),
        Kind::Paused if !view.snap.started => i18n::t(lang, K::StatusReady).into(),
        Kind::Paused => i18n::t(lang, K::StatusPaused).into(),
    }
}

fn fmt_hms(ms: u64) -> String {
    let s = ms / 1000;
    format!("{}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60)
}

#[derive(Default)]
struct TrayShown {
    kind: Option<Kind>,
    started: bool,
    light: bool,
    tooltip: String,
}

fn update_tray(app: &AppHandle, view: &View) {
    let Some(tray) = app.try_state::<Tray>() else { return };
    let snap = &view.snap;
    let lang = app.state::<Core>().lang();
    let shown = app.state::<Mutex<TrayShown>>();
    let mut shown = shown.lock().unwrap();
    let light = tracker::taskbar_is_light();
    if shown.kind != Some(snap.kind) || shown.started != snap.started || shown.light != light {
        if let Ok(img) = Image::from_bytes(tray_icon_bytes(snap.kind, light)) {
            let _ = tray.icon.set_icon(Some(img));
        }
        let label = match (snap.kind, snap.started) {
            (Kind::Paused, false) => i18n::K::TrayStart,
            (Kind::Paused, true) => i18n::K::TrayResume,
            _ => i18n::K::TrayPause,
        };
        let _ = tray.pause_item.set_text(i18n::t(&lang, label));
        shown.kind = Some(snap.kind);
        shown.started = snap.started;
        shown.light = light;
    }
    let tooltip = format!(
        "Tally · {}\n{} {} · {} {}",
        status_text(&lang, view),
        i18n::t(&lang, i18n::K::Work),
        fmt_hms(snap.work_ms),
        i18n::t(&lang, i18n::K::Elapsed),
        fmt_hms(snap.elapsed_ms)
    );
    if tooltip != shown.tooltip {
        let _ = tray.icon.set_tooltip(Some(&tooltip));
        shown.tooltip = tooltip;
    }
}

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    use i18n::K;
    let lang = app.state::<Core>().lang();
    let item = |id: &str, k: K| MenuItem::with_id(app, id, i18n::t(&lang, k), true, None::<&str>);
    let open = item("open", K::TrayOpen)?;
    let mini = item("mini", K::TrayMini)?;
    let pause = item("pause", K::TrayPause)?;
    let reset = item("reset", K::TrayReset)?;
    let quit = item("quit", K::TrayQuit)?;
    let menu = Menu::with_items(
        app,
        &[&open, &mini, &pause, &reset, &PredefinedMenuItem::separator(app)?, &quit],
    )?;
    let icon = TrayIconBuilder::with_id("main")
        .icon(Image::from_bytes(tray_icon_bytes(Kind::Other, tracker::taskbar_is_light()))?)
        .tooltip("Tally")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, ev| match ev.id().as_ref() {
            "open" => show_main(app),
            "mini" => toggle_mini(app),
            "pause" => {
                do_toggle_pause(app);
            }
            "reset" => ask_reset(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, ev| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = ev
            {
                show_main(tray.app_handle());
            }
        })
        .build(app)?;
    app.manage(Tray {
        icon,
        open_item: open,
        mini_item: mini,
        pause_item: pause,
        reset_item: reset,
        quit_item: quit,
    });
    Ok(())
}

// ---- 1초 폴링 루프 ----

fn spawn_loop(app: AppHandle) {
    std::thread::spawn(move || {
        let mut tracker = tracker::Tracker::default();
        let mut clock = AlarmClock::default();
        let mut next = Instant::now();
        let mut n: u64 = 0;
        loop {
            next += Duration::from_secs(1);
            n += 1;
            let core = app.state::<Core>();
            if n % 3 == 0 {
                let reloaded = core.settings_file.lock().unwrap().reload_if_changed();
                if let Some(s) = reloaded {
                    *core.settings.lock().unwrap() = s;
                    core.refresh_cfg();
                }
            }
            let break_over = core.brk.lock().unwrap().as_ref().map_or(false, |b| now_ms() >= b.until_ms);
            if break_over {
                end_break(&app);
            }
            let sample = tracker.sample();
            let hour = core.settings.lock().unwrap().day_start_hour;
            let day = day_of(chrono::Local::now().naive_local(), hour);
            {
                let cfg = core.cfg.lock().unwrap().clone();
                let mut e = core.engine.lock().unwrap();
                let now = now_ms();
                let out = e.tick(now, &day, &sample, &cfg);
                let mut st = core.store.lock().unwrap();
                Core::record(&mut st, out.interval);
                if out.new_day {
                    if let Err(err) = st.end_session(now, None).and_then(|_| st.start_session(now)) {
                        eprintln!("세션 기록 실패: {err}");
                    }
                }
                if out.changed && cfg!(debug_assertions) {
                    println!("[tally] {:?} {:?} ({:?})", e.kind(), sample.exe, sample.title);
                }
            }
            // 타이머·알람을 먼저 확인해야 이번 tick에 보내는 상태에 반영된다
            check_tools(&app, &mut clock);
            let mut view = core.view();
            let alerts = core.alerts.lock().unwrap().clone();
            let s = &view.snap;
            let fired = core.scheduler.lock().unwrap().check(&alerts, s.session_started_ms, s.elapsed_ms, s.work_ms);
            if !fired.is_empty() {
                fire_alerts(&app, &fired, &view);
                view = core.view();
            }
            publish(&app, &view);
            let now = Instant::now();
            if next > now {
                std::thread::sleep(next - now);
            } else {
                next = now; // 절전 복귀 등으로 밀렸으면 다시 맞춘다
            }
        }
    });
}

/// DB를 열고, 1단계 settings.json의 추적 목록을 옮기고, 오늘 세션이 있으면 이어 쓴다.
fn init_core() -> Core {
    let mut settings_file = SettingsFile::new();
    let mut s = settings_file.load();
    let db_path = settings::data_dir().join("tally.db");
    let mut store = Store::open(&db_path).unwrap_or_else(|e| {
        eprintln!("DB 열기 실패({}), 메모리에만 기록: {e}", db_path.display());
        Store::open(std::path::Path::new(":memory:")).expect("메모리 DB")
    });

    if !s.tracked.is_empty() && store.list_apps().map(|a| a.is_empty()).unwrap_or(false) {
        for t in &s.tracked {
            if let Ok(row) = store.add_app(&t.exe, t.label.as_deref(), t.title_kw.as_deref()) {
                if !t.enabled {
                    let _ = store.update_app(&AppRow { enabled: false, ..row });
                }
            }
        }
        s.onboarded = true;
    }
    if !s.tracked.is_empty() {
        s.tracked.clear();
        settings_file.save(&s);
    }

    let now = now_ms();
    let hour = s.day_start_hour;
    let today = day_of(chrono::Local::now().naive_local(), hour);
    let engine = match store.take_resumable(&today, |ms| day_of_ms(ms, hour)) {
        Ok(Some(r)) => Engine::resume(now, &today, r.started_ms, r.elapsed_ms, r.work_ms, r.apps),
        other => {
            if let Err(e) = other {
                eprintln!("세션 복원 실패: {e}");
            }
            let _ = store.start_session(now);
            Engine::new(now, &today)
        }
    };
    let cfg = Config { tracked: store.tracked().unwrap_or_default(), idle_ms: s.idle_ms() };
    let alerts = store.list_alerts().unwrap_or_default();
    let alarms = store.list_alarms().unwrap_or_default();
    let mut timer = Timer::default();
    timer.set(s.timer.last_ms.max(1_000));
    Core {
        engine: Mutex::new(engine),
        store: Mutex::new(store),
        cfg: Mutex::new(cfg),
        settings: Mutex::new(s),
        settings_file: Mutex::new(settings_file),
        alerts: Mutex::new(alerts),
        scheduler: Mutex::new(Scheduler::default()),
        brk: Mutex::new(None),
        timer: Mutex::new(timer),
        stopwatch: Mutex::new(Stopwatch::default()),
        alarms: Mutex::new(alarms),
        shortcut_errors: Mutex::new(Vec::new()),
        ask_reset: AtomicBool::new(false),
    }
}

pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show_main(app)))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::Builder::new().arg("--tray").build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, ev| on_shortcut(app, shortcut, ev.state()))
                .build(),
        )
        .manage(init_core())
        .manage(Mutex::new(TrayShown::default()))
        .invoke_handler(tauri::generate_handler![
            get_state,
            toggle_pause,
            reset,
            end_session,
            take_ask_reset,
            close_to_tray,
            list_apps,
            add_app,
            update_app,
            delete_app,
            running_programs,
            get_onboarded,
            finish_onboarding,
            list_alerts,
            add_alert,
            update_alert,
            delete_alert,
            quick_enable_alert,
            set_alert_sound,
            preview_sound,
            read_media,
            save_clip,
            get_settings,
            set_idle_minutes,
            set_shortcut,
            set_language,
            timer_set,
            timer_start,
            timer_pause,
            timer_reset,
            sw_start,
            sw_stop,
            sw_lap,
            sw_reset,
            set_timer_settings,
            set_timer_sound,
            get_timer_settings,
            list_alarms,
            add_alarm,
            update_alarm,
            delete_alarm,
            set_alarm_sound,
            history_days,
            history_day,
            set_session_note,
            export_csv,
            set_theme,
            set_autostart,
            set_mini,
            save_mini_pos,
            open_mini,
            close_mini,
            back_to_main,
            fold_to_mini
        ])
        .setup(|app| {
            build_tray(app.handle())?;
            apply_shortcuts(app.handle());
            spawn_loop(app.handle().clone());
            // 시작 시 창을 띄우지 않으려면 --tray 인자로 실행
            if !std::env::args().any(|a| a == "--tray") {
                show_main(app.handle());
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Tally 실행 실패");

    app.run(|app, ev| match ev {
        // 창이 모두 닫혀도 트레이에서 계속 돈다. 트레이 '종료'(exit 코드 있음)만 실제로 끝낸다.
        RunEvent::ExitRequested { api, code, .. } if code.is_none() => api.prevent_exit(),
        // 진행 중인 구간을 디스크에 쓴다. 세션은 열어 둬서 같은 날 다시 켜면 이어 센다.
        RunEvent::Exit => {
            let core = app.state::<Core>();
            let _ = core.store.lock().unwrap().flush();
        }
        _ => {}
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_decode_round_trips_korean() {
        // encodeURIComponent("노래 1%")
        assert_eq!(percent_decode("%EB%85%B8%EB%9E%98%201%25"), "노래 1%");
        assert_eq!(percent_decode("a%2"), "a%2");
        assert_eq!(percent_decode("%zz"), "%zz");
    }

    #[test]
    fn default_shortcuts_parse() {
        let sc = settings::Shortcuts::default();
        let p: Shortcut = sc.pause.parse().unwrap();
        let r: Shortcut = sc.reset.parse().unwrap();
        assert_ne!(p, r);
        assert_eq!("ctrl+alt+p".parse::<Shortcut>().unwrap(), p);
        // UI가 만드는 형식들
        for s in ["Ctrl+Shift+F5", "Alt+Space", "F9", "Ctrl+Alt+1"] {
            s.parse::<Shortcut>().unwrap_or_else(|e| panic!("{s}: {e}"));
        }
    }
}
