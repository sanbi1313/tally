//! 세션 엔진: 1초마다 받은 표본으로 경과·실 작업·프로그램별 시간을 계산한다.
//! 누적할 때마다 그 구간(Interval)을 돌려주고, 저장은 store가 맡는다.
//! OS에 의존하지 않는 순수 로직이라 테스트로 검증한다.

use serde::Serialize;

/// 이 간격보다 긴 틈(절전, 렉 등)은 실 작업으로 치지 않고 경과에만 넣는다.
pub const MAX_STEP_MS: i64 = 5_000;

/// Tally 창을 보는 시간은 작업으로 치지 않는다.
const SELF_EXE: &str = "tally.exe";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// 등록 프로그램이 활성 창이고 입력이 있음
    Work,
    /// 다른 프로그램 사용 중(대기)
    Other,
    /// 자리 비움
    Idle,
    Paused,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Work => "work",
            Kind::Other => "other",
            Kind::Idle => "idle",
            Kind::Paused => "paused",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackedApp {
    /// apps 테이블 id
    pub id: Option<i64>,
    pub exe: String,
    pub label: String,
    pub title_kw: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub tracked: Vec<TrackedApp>,
    pub idle_ms: u64,
}

/// 작업으로 판정된 프로그램
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppRef {
    /// 프로그램별 합산 기준. 등록 프로그램은 "id:3", 미등록(모든 프로그램 모드)은 "exe:notepad.exe"
    pub key: String,
    pub label: String,
    pub app_id: Option<i64>,
    pub exe: String,
    pub color: Option<String>,
}

impl AppRef {
    pub fn key_of(app_id: Option<i64>, exe: &str) -> String {
        match app_id {
            Some(id) => format!("id:{id}"),
            None => format!("exe:{}", exe.to_lowercase()),
        }
    }
}

/// 추적기가 1초마다 읽어 오는 값
#[derive(Debug, Clone, Default)]
pub struct Sample {
    /// 포그라운드 창의 실행 파일 이름(예: CLIPStudioPaint.exe). 없으면 바탕화면·잠금 화면 등
    pub exe: Option<String>,
    pub title: String,
    /// 마지막 마우스·키보드 입력 후 지난 시간
    pub idle_ms: u64,
}

/// 같은 상태로 흐른 시간 한 토막. store가 segments 행으로 합친다.
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub start_ms: i64,
    pub end_ms: i64,
    pub kind: Kind,
    pub app: Option<AppRef>,
    pub day: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTime {
    /// "id:3" 또는 "exe:notepad.exe". 화면 목록의 키(이름은 겹칠 수 있다)
    pub key: String,
    pub label: String,
    pub color: Option<String>,
    pub work_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub kind: Kind,
    /// 지금 추적 중인 프로그램의 표시 이름(작업 중일 때만)
    pub app: Option<String>,
    pub work_ms: u64,
    pub elapsed_ms: u64,
    pub session_started_ms: i64,
    pub day: String,
    pub apps: Vec<AppTime>,
    /// 마지막 초기화·세션 종료 직전의 실 작업 시간
    pub prev_work_ms: Option<u64>,
}

#[derive(Debug, Default)]
pub struct TickOut {
    pub interval: Option<Interval>,
    /// 상태(종류·프로그램)가 바뀜
    pub changed: bool,
    /// 하루가 바뀌어 새 세션을 열었음
    pub new_day: bool,
}

pub struct Engine {
    session_started_ms: i64,
    day: String,
    elapsed_ms: u64,
    work_ms: u64,
    apps: Vec<AppTime>,
    paused: bool,
    /// 직전 표본의 분류. 다음 tick까지의 구간에 적용된다.
    kind: Kind,
    app: Option<AppRef>,
    last_ms: Option<i64>,
    prev_work_ms: Option<u64>,
}

impl Engine {
    pub fn new(now_ms: i64, day: &str) -> Self {
        Self {
            session_started_ms: now_ms,
            day: day.to_string(),
            elapsed_ms: 0,
            work_ms: 0,
            apps: Vec::new(),
            paused: false,
            kind: Kind::Other,
            app: None,
            last_ms: None,
            prev_work_ms: None,
        }
    }

    /// 저장된 세션을 이어서 센다(앱을 다시 켰을 때 같은 날이면).
    pub fn resume(
        now_ms: i64,
        day: &str,
        session_started_ms: i64,
        elapsed_ms: u64,
        work_ms: u64,
        apps: Vec<AppTime>,
    ) -> Self {
        let mut e = Self::new(now_ms, day);
        e.session_started_ms = session_started_ms;
        e.elapsed_ms = elapsed_ms;
        e.work_ms = work_ms;
        e.apps = apps;
        e
    }

    /// 표본이 어떤 상태인지 판정. 작업이면 프로그램도 돌려준다.
    pub fn classify(sample: &Sample, cfg: &Config) -> (Kind, Option<AppRef>) {
        if sample.idle_ms >= cfg.idle_ms {
            return (Kind::Idle, None);
        }
        let Some(exe) = sample.exe.as_deref() else {
            return (Kind::Other, None);
        };
        // 등록 프로그램이 하나도 없으면 모든 프로그램 = 작업(Tally 자신은 빼고)
        if cfg.tracked.is_empty() {
            if exe.eq_ignore_ascii_case(SELF_EXE) {
                return (Kind::Other, None);
            }
            return (
                Kind::Work,
                Some(AppRef {
                    key: AppRef::key_of(None, exe),
                    label: display_name(exe),
                    app_id: None,
                    exe: exe.to_string(),
                    color: None,
                }),
            );
        }
        let title = sample.title.to_lowercase();
        let hit = cfg.tracked.iter().find(|t| {
            t.exe.eq_ignore_ascii_case(exe)
                && t.title_kw
                    .as_deref()
                    .map_or(true, |kw| kw.trim().is_empty() || title.contains(&kw.trim().to_lowercase()))
        });
        match hit {
            Some(t) => (
                Kind::Work,
                Some(AppRef {
                    key: AppRef::key_of(t.id, &t.exe),
                    label: t.label.clone(),
                    app_id: t.id,
                    exe: exe.to_string(),
                    color: t.color.clone(),
                }),
            ),
            None => (Kind::Other, None),
        }
    }

    /// 직전 tick부터 now까지를 직전 분류로 누적하고 그 구간을 돌려준다.
    fn accrue(&mut self, now_ms: i64) -> Option<Interval> {
        let Some(last) = self.last_ms else {
            self.last_ms = Some(now_ms);
            return None;
        };
        let dt = now_ms - last;
        if dt <= 0 {
            return None;
        }
        self.last_ms = Some(now_ms);
        let mut iv = Interval {
            start_ms: last,
            end_ms: now_ms,
            kind: self.kind,
            app: None,
            day: self.day.clone(),
        };
        if self.paused {
            iv.kind = Kind::Paused;
            return Some(iv);
        }
        let dt_u = dt as u64;
        self.elapsed_ms += dt_u;
        if dt > MAX_STEP_MS {
            iv.kind = Kind::Idle;
            return Some(iv);
        }
        if self.kind == Kind::Work {
            self.work_ms += dt_u;
            if let Some(app) = &self.app {
                match self.apps.iter_mut().find(|a| a.key == app.key) {
                    Some(a) => {
                        a.work_ms += dt_u;
                        a.label = app.label.clone();
                        a.color = app.color.clone();
                    }
                    None => self.apps.push(AppTime {
                        key: app.key.clone(),
                        label: app.label.clone(),
                        color: app.color.clone(),
                        work_ms: dt_u,
                    }),
                }
            }
            iv.app = self.app.clone();
        }
        Some(iv)
    }

    /// 1초마다 호출. 하루가 바뀌면(새벽 5시 기준) 새 세션을 연다.
    pub fn tick(&mut self, now_ms: i64, day: &str, sample: &Sample, cfg: &Config) -> TickOut {
        let interval = self.accrue(now_ms);
        let new_day = day != self.day;
        if new_day {
            let paused = self.paused;
            *self = Engine::new(now_ms, day);
            self.paused = paused;
            self.last_ms = Some(now_ms);
        }
        let before = (self.kind, self.app.clone());
        if self.paused {
            self.kind = Kind::Paused;
            self.app = None;
        } else {
            (self.kind, self.app) = Self::classify(sample, cfg);
        }
        let changed = before != (self.kind, self.app.clone());
        TickOut { interval, changed, new_day }
    }

    pub fn toggle_pause(&mut self, now_ms: i64) -> Option<Interval> {
        let iv = self.accrue(now_ms);
        self.paused = !self.paused;
        if self.paused {
            self.kind = Kind::Paused;
            self.app = None;
        } else {
            // 다음 tick에서 실제 상태로 다시 판정할 때까지는 대기로 둔다
            self.kind = Kind::Other;
        }
        iv
    }

    /// 현재 세션을 0으로 되돌린다(초기화·세션 종료 공통). 일시정지 상태는 유지.
    pub fn reset(&mut self, now_ms: i64) -> Option<Interval> {
        let iv = self.accrue(now_ms);
        let prev = self.work_ms;
        let (paused, kind, app, day) = (self.paused, self.kind, self.app.clone(), self.day.clone());
        *self = Engine::new(now_ms, &day);
        self.paused = paused;
        self.kind = kind;
        self.app = app;
        self.last_ms = Some(now_ms);
        self.prev_work_ms = Some(prev);
        iv
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn snapshot(&self) -> Snapshot {
        let mut apps = self.apps.clone();
        apps.sort_by(|a, b| b.work_ms.cmp(&a.work_ms));
        Snapshot {
            kind: self.kind,
            app: self.app.as_ref().map(|a| a.label.clone()),
            work_ms: self.work_ms,
            elapsed_ms: self.elapsed_ms,
            session_started_ms: self.session_started_ms,
            day: self.day.clone(),
            apps,
            prev_work_ms: self.prev_work_ms,
        }
    }
}

/// 잘 알려진 프로그램은 공식 제품 이름(모든 언어 공통), 나머지는 확장자를 뺀 exe 이름.
/// "CLIPStudioPaint.exe" → "CLIP STUDIO PAINT", "foo.exe" → "foo"
pub fn display_name(exe: &str) -> String {
    const KNOWN: [(&str, &str); 17] = [
        ("clipstudiopaint.exe", "CLIP STUDIO PAINT"),
        ("photoshop.exe", "Photoshop"),
        ("illustrator.exe", "Illustrator"),
        ("afterfx.exe", "After Effects"),
        ("adobe premiere pro.exe", "Premiere Pro"),
        ("chrome.exe", "Chrome"),
        ("msedge.exe", "Edge"),
        ("firefox.exe", "Firefox"),
        ("discord.exe", "Discord"),
        ("krita.exe", "Krita"),
        ("sai2.exe", "PaintTool SAI Ver.2"),
        ("sai.exe", "PaintTool SAI"),
        ("blender.exe", "Blender"),
        ("figma.exe", "Figma"),
        ("code.exe", "Visual Studio Code"),
        ("notepad.exe", "Notepad"),
        ("mspaint.exe", "Paint"),
    ];
    let lower = exe.to_lowercase();
    if let Some((_, name)) = KNOWN.iter().find(|(e, _)| *e == lower) {
        return name.to_string();
    }
    let n = exe.len();
    if n > 4 && exe.is_char_boundary(n - 4) && exe[n - 4..].eq_ignore_ascii_case(".exe") {
        exe[..n - 4].to_string()
    } else {
        exe.to_string()
    }
}

/// 하루 경계(새벽 5시)를 적용한 날짜 문자열. 새벽 2시는 전날로 친다.
pub fn day_of(local: chrono::NaiveDateTime, day_start_hour: u32) -> String {
    (local - chrono::Duration::hours(day_start_hour as i64))
        .date()
        .format("%Y-%m-%d")
        .to_string()
}

/// unix ms → 하루 경계 적용 날짜(현지 시각)
pub fn day_of_ms(ms: i64, day_start_hour: u32) -> String {
    use chrono::TimeZone;
    let local = chrono::Local
        .timestamp_millis_opt(ms)
        .single()
        .unwrap_or_else(chrono::Local::now);
    day_of(local.naive_local(), day_start_hour)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: &str = "2026-11-01";

    fn cfg(tracked: &[(&str, &str, Option<&str>)]) -> Config {
        Config {
            tracked: tracked
                .iter()
                .enumerate()
                .map(|(i, (exe, label, kw))| TrackedApp {
                    id: Some(i as i64 + 1),
                    exe: exe.to_string(),
                    label: label.to_string(),
                    title_kw: kw.map(str::to_string),
                    color: None,
                })
                .collect(),
            idle_ms: 180_000,
        }
    }

    fn s(exe: &str, title: &str, idle_ms: u64) -> Sample {
        Sample { exe: Some(exe.to_string()), title: title.to_string(), idle_ms }
    }

    /// start_ms부터 1초 간격으로 secs번 같은 표본을 넣는다
    fn run(e: &mut Engine, start_ms: i64, secs: i64, sample: &Sample, c: &Config) -> i64 {
        for i in 0..secs {
            e.tick(start_ms + i * 1000, DAY, sample, c);
        }
        start_ms + secs * 1000
    }

    #[test]
    fn counts_only_tracked_app_as_work() {
        let c = cfg(&[("CLIPStudioPaint.exe", "클립 스튜디오", None)]);
        let mut e = Engine::new(0, DAY);
        let t = run(&mut e, 0, 11, &s("clipstudiopaint.exe", "", 0), &c); // 10초 작업
        let t = run(&mut e, t, 5, &s("chrome.exe", "", 0), &c); // 5초 대기
        e.tick(t, DAY, &s("chrome.exe", "", 0), &c);
        let snap = e.snapshot();
        assert_eq!(snap.work_ms, 11_000); // 마지막 작업 표본 뒤 1초까지 포함
        assert_eq!(snap.elapsed_ms, 16_000);
        assert_eq!(snap.kind, Kind::Other);
        assert_eq!(snap.apps[0].label, "클립 스튜디오");
        assert_eq!(snap.apps[0].work_ms, 11_000);
    }

    #[test]
    fn empty_list_means_everything_is_work() {
        let c = cfg(&[]);
        let mut e = Engine::new(0, DAY);
        let t = run(&mut e, 0, 4, &s("notepad.exe", "", 0), &c);
        e.tick(t, DAY, &s("notepad.exe", "", 0), &c);
        let snap = e.snapshot();
        assert_eq!(snap.work_ms, 4_000);
        assert_eq!(snap.app.as_deref(), Some("Notepad"));
        assert_eq!(snap.apps[0].key, "exe:notepad.exe");
        assert_eq!(Engine::classify(&s("Tally.exe", "Tally", 0), &c).0, Kind::Other);
    }

    #[test]
    fn idle_input_stops_work_but_not_elapsed() {
        let c = cfg(&[]);
        let mut e = Engine::new(0, DAY);
        let t = run(&mut e, 0, 3, &s("a.exe", "", 0), &c);
        let t = run(&mut e, t, 5, &s("a.exe", "", 200_000), &c);
        e.tick(t, DAY, &s("a.exe", "", 0), &c);
        let snap = e.snapshot();
        assert_eq!(snap.work_ms, 3_000);
        assert_eq!(snap.elapsed_ms, 8_000);
    }

    #[test]
    fn title_keyword_narrows_match() {
        let c = cfg(&[("chrome.exe", "크롬", Some("Pinterest"))]);
        assert_eq!(Engine::classify(&s("chrome.exe", "YouTube", 0), &c).0, Kind::Other);
        let (k, app) = Engine::classify(&s("chrome.exe", "Cats - pinterest - Chrome", 0), &c);
        assert_eq!(k, Kind::Work);
        let app = app.unwrap();
        assert_eq!(app.label, "크롬");
        assert_eq!(app.key, "id:1");
    }

    #[test]
    fn no_foreground_window_is_other() {
        let c = cfg(&[]);
        let sample = Sample { exe: None, title: String::new(), idle_ms: 0 };
        assert_eq!(Engine::classify(&sample, &c).0, Kind::Other);
    }

    #[test]
    fn pause_freezes_both_clocks() {
        let c = cfg(&[]);
        let mut e = Engine::new(0, DAY);
        let t = run(&mut e, 0, 3, &s("a.exe", "", 0), &c);
        e.toggle_pause(t); // 3초 누적 후 정지
        let t = run(&mut e, t, 10, &s("a.exe", "", 0), &c);
        assert_eq!(e.snapshot().kind, Kind::Paused);
        e.toggle_pause(t);
        let t = run(&mut e, t, 2, &s("a.exe", "", 0), &c);
        e.tick(t, DAY, &s("a.exe", "", 0), &c);
        let snap = e.snapshot();
        assert_eq!(snap.elapsed_ms, 5_000);
        assert_eq!(snap.work_ms, 5_000);
    }

    #[test]
    fn long_gap_counts_as_elapsed_only() {
        let c = cfg(&[]);
        let mut e = Engine::new(0, DAY);
        e.tick(0, DAY, &s("a.exe", "", 0), &c);
        let out = e.tick(60_000, DAY, &s("a.exe", "", 0), &c); // 절전 등으로 1분 건너뜀
        let snap = e.snapshot();
        assert_eq!(snap.work_ms, 0);
        assert_eq!(snap.elapsed_ms, 60_000);
        assert_eq!(out.interval.unwrap().kind, Kind::Idle);
    }

    #[test]
    fn reset_zeroes_session_and_remembers_previous() {
        let c = cfg(&[]);
        let mut e = Engine::new(0, DAY);
        let t = run(&mut e, 0, 6, &s("a.exe", "", 0), &c);
        let iv = e.reset(t).unwrap();
        assert_eq!((iv.start_ms, iv.end_ms, iv.kind), (5_000, 6_000, Kind::Work));
        let snap = e.snapshot();
        assert_eq!(snap.work_ms, 0);
        assert_eq!(snap.elapsed_ms, 0);
        assert_eq!(snap.prev_work_ms, Some(6_000));
        assert_eq!(snap.session_started_ms, t);
        // 초기화 후에도 계속 센다
        let t = run(&mut e, t, 2, &s("a.exe", "", 0), &c);
        e.tick(t, DAY, &s("a.exe", "", 0), &c);
        assert_eq!(e.snapshot().work_ms, 2_000);
    }

    #[test]
    fn day_change_starts_new_session() {
        let c = cfg(&[]);
        let mut e = Engine::new(0, DAY);
        let t = run(&mut e, 0, 5, &s("a.exe", "", 0), &c);
        let out = e.tick(t, "2026-11-02", &s("a.exe", "", 0), &c);
        assert!(out.new_day);
        // 넘어가기 직전 구간은 전날로 기록된다
        assert_eq!(out.interval.unwrap().day, DAY);
        let snap = e.snapshot();
        assert_eq!(snap.day, "2026-11-02");
        assert_eq!(snap.work_ms, 0);
    }

    #[test]
    fn intervals_cover_time_and_carry_app() {
        let c = cfg(&[("a.exe", "A", None)]);
        let mut e = Engine::new(0, DAY);
        let mut ivs = Vec::new();
        for (i, exe) in ["a.exe", "a.exe", "b.exe", "b.exe"].iter().enumerate() {
            ivs.extend(e.tick(i as i64 * 1000, DAY, &s(exe, "", 0), &c).interval);
        }
        ivs.extend(e.toggle_pause(4_000));
        ivs.extend(e.tick(5_000, DAY, &s("a.exe", "", 0), &c).interval);
        let kinds: Vec<_> = ivs.iter().map(|i| (i.start_ms, i.kind, i.app.as_ref().map(|a| a.label.as_str()))).collect();
        assert_eq!(
            kinds,
            vec![
                (0, Kind::Work, Some("A")),
                (1_000, Kind::Work, Some("A")),
                (2_000, Kind::Other, None),
                (3_000, Kind::Other, None),
                (4_000, Kind::Paused, None),
            ]
        );
    }

    #[test]
    fn resume_continues_totals() {
        let c = cfg(&[("a.exe", "A", None)]);
        let apps = vec![AppTime { key: "id:1".into(), label: "A".into(), color: None, work_ms: 10_000 }];
        let mut e = Engine::resume(100_000, DAY, 50_000, 20_000, 10_000, apps);
        e.tick(100_000, DAY, &s("a.exe", "", 0), &c);
        e.tick(101_000, DAY, &s("a.exe", "", 0), &c);
        let snap = e.snapshot();
        assert_eq!(snap.session_started_ms, 50_000);
        assert_eq!(snap.work_ms, 11_000);
        assert_eq!(snap.elapsed_ms, 21_000);
        assert_eq!(snap.apps.len(), 1);
        assert_eq!(snap.apps[0].work_ms, 11_000);
    }

    #[test]
    fn day_boundary_is_5am() {
        use chrono::NaiveDate;
        let at = |h, m| NaiveDate::from_ymd_opt(2026, 11, 2).unwrap().and_hms_opt(h, m, 0).unwrap();
        assert_eq!(day_of(at(2, 0), 5), "2026-11-01");
        assert_eq!(day_of(at(4, 59), 5), "2026-11-01");
        assert_eq!(day_of(at(5, 0), 5), "2026-11-02");
    }
}
