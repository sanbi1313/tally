//! %APPDATA%\Tally\settings.json: 자리 비움 기준, 하루 시작 시각, 첫 실행 여부.
//! 추적 프로그램은 2단계부터 DB(apps 테이블)에 있다. 1단계 파일의 `tracked`는 첫 실행 때 한 번 옮긴다.
//! 앱이 켜져 있는 동안 파일을 고치면 몇 초 안에 다시 읽는다.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub idle_minutes: f64,
    pub day_start_hour: u32,
    /// "무엇으로 작업하세요?" 화면을 지났는지
    pub onboarded: bool,
    pub shortcuts: Shortcuts,
    /// "ko" | "en" | "ja" | "zh"
    pub language: String,
    pub timer: TimerSettings,
    /// "system" | "light" | "dark"
    pub theme: String,
    pub mini: MiniSettings,
    /// 1단계 형식. 읽기만 하고 다시 쓰지 않는다.
    #[serde(skip_serializing)]
    pub tracked: Vec<LegacyTracked>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyTracked {
    pub exe: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub title_kw: Option<String>,
    #[serde(default = "yes")]
    pub enabled: bool,
}

/// 작은 타이머(항상 위 창)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MiniSettings {
    /// "work" | "elapsed" | "both"
    pub show: String,
    /// 항상 위
    pub pinned: bool,
    /// 0.6 ~ 1.0
    pub opacity: f32,
    /// "small" | "normal"
    pub size: String,
    /// 칩으로 접혀 있음
    pub collapsed: bool,
    /// 마지막 위치(화면 픽셀)
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Default for MiniSettings {
    fn default() -> Self {
        Self {
            show: "work".into(),
            pinned: true,
            opacity: 1.0,
            size: "normal".into(),
            collapsed: false,
            x: None,
            y: None,
        }
    }
}

impl MiniSettings {
    /// 창 크기(논리 픽셀): 한 줄 160×36, 두 줄 160×60, 칩 96×28. 작게는 약 80%
    pub fn window_size(&self) -> (f64, f64) {
        let (w, h) = if self.collapsed {
            (96.0, 28.0)
        } else if self.show == "both" {
            (160.0, 60.0)
        } else {
            (160.0, 36.0)
        };
        if self.size == "small" {
            ((w * 0.8f64).round(), (h * 0.8f64).round())
        } else {
            (w, h)
        }
    }
}

/// 도구 탭 타이머
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TimerSettings {
    /// 프리셋(분)
    pub presets: Vec<u32>,
    /// 끝날 때 소리. 없으면 차임
    pub sound: Option<String>,
    pub volume: f32,
    /// 마지막으로 정한 시간(다음 실행 때 그대로)
    pub last_ms: u64,
}

impl Default for TimerSettings {
    fn default() -> Self {
        Self { presets: vec![5, 25, 50], sound: None, volume: 0.8, last_ms: 25 * 60_000 }
    }
}

/// 전역 단축키. 빈 문자열이면 끔
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Shortcuts {
    pub pause: String,
    pub reset: String,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self { pause: "Ctrl+Alt+P".into(), reset: "Ctrl+Alt+R".into() }
    }
}

fn yes() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            idle_minutes: 3.0,
            day_start_hour: 5,
            onboarded: false,
            shortcuts: Shortcuts::default(),
            language: "ko".into(),
            timer: TimerSettings::default(),
            theme: "system".into(),
            mini: MiniSettings::default(),
            tracked: Vec::new(),
        }
    }
}

impl Settings {
    pub fn idle_ms(&self) -> u64 {
        (self.idle_minutes.max(0.1) * 60_000.0) as u64
    }
}

pub fn data_dir() -> PathBuf {
    let base = std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    base.join("Tally")
}

pub struct SettingsFile {
    path: PathBuf,
    mtime: Option<SystemTime>,
}

impl SettingsFile {
    pub fn new() -> Self {
        Self { path: data_dir().join("settings.json"), mtime: None }
    }

    /// 파일이 없으면 기본값으로 만들어 둔다(손으로 고칠 수 있게).
    pub fn load(&mut self) -> Settings {
        if !self.path.exists() {
            self.save(&Settings::default());
        }
        self.mtime = self.modified();
        match std::fs::read_to_string(&self.path) {
            // 메모장이 붙이는 BOM 제거
            Ok(text) => serde_json::from_str(text.trim_start_matches('\u{feff}')).unwrap_or_else(|e| {
                eprintln!("settings.json 읽기 실패, 기본값 사용: {e}");
                Settings::default()
            }),
            Err(_) => Settings::default(),
        }
    }

    pub fn save(&mut self, s: &Settings) {
        let _ = std::fs::create_dir_all(self.path.parent().unwrap());
        let text = serde_json::to_string_pretty(s).unwrap();
        if std::fs::write(&self.path, text).is_ok() {
            self.mtime = self.modified();
        }
    }

    /// 파일이 바뀌었으면 새 설정을 돌려준다.
    pub fn reload_if_changed(&mut self) -> Option<Settings> {
        let m = self.modified();
        if m.is_some() && m != self.mtime {
            Some(self.load())
        } else {
            None
        }
    }

    fn modified(&self) -> Option<SystemTime> {
        std::fs::metadata(&self.path).and_then(|m| m.modified()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mini_window_sizes() {
        let mut m = MiniSettings::default();
        assert_eq!(m.window_size(), (160.0, 36.0));
        m.show = "both".into();
        assert_eq!(m.window_size(), (160.0, 60.0));
        m.size = "small".into();
        assert_eq!(m.window_size(), (128.0, 48.0));
        m.collapsed = true;
        assert_eq!(m.window_size(), (77.0, 22.0));
    }

    #[test]
    fn parses_step1_file_and_drops_tracked_on_save() {
        let s: Settings = serde_json::from_str(
            r#"{ "idleMinutes": 5, "tracked": [ { "exe": "CLIPStudioPaint.exe", "label": "클립 스튜디오" },
                              { "exe": "chrome.exe", "titleKw": "Pinterest" },
                              { "exe": "Discord.exe", "enabled": false } ] }"#,
        )
        .unwrap();
        assert_eq!(s.idle_ms(), 300_000);
        assert_eq!(s.day_start_hour, 5);
        assert!(!s.onboarded);
        assert_eq!(s.shortcuts.pause, "Ctrl+Alt+P");
        assert_eq!(s.timer.presets, vec![5, 25, 50]);
        assert_eq!(s.theme, "system");
        assert!(s.mini.pinned);
        assert_eq!(s.tracked.len(), 3);
        assert_eq!(s.tracked[1].title_kw.as_deref(), Some("Pinterest"));
        assert!(!s.tracked[2].enabled);
        assert!(!serde_json::to_string(&s).unwrap().contains("tracked"));
    }
}
