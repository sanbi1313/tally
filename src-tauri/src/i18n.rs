//! Rust 쪽 문구(트레이 메뉴·툴팁, Windows 알림, 오류). 화면 문구는 src/lib/i18n.svelte.ts.
//! 언어 코드: "ko" | "en" | "ja" | "zh". 그 밖의 코드는 한국어로.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K {
    TrayOpen,
    TrayMini,
    TrayPause,
    TrayResume,
    TrayStart,
    TrayReset,
    TrayQuit,
    /// "{app} 작업 중"
    StatusWork,
    StatusOther,
    StatusIdle,
    StatusPaused,
    /// 앱을 켠 뒤·초기화 뒤 시작을 누르기 전
    StatusReady,
    StatusBreak,
    Work,
    Elapsed,
    /// "{n}분마다 알림 · 잠깐 쉬어 가세요"
    AlertBody,
    /// "{n}분 휴식을 시작해요"
    AlertBodyBreak,
    BreakEndTitle,
    /// "다음 알림은 {n}분 뒤"
    BreakEndBody,
    ErrExeEmpty,
    ErrSoundType,
    ErrAlertNotFound,
    /// "단축키 형식 오류: {e}"
    ErrShortcut,
    TimerDoneTitle,
    /// "{time} 타이머가 끝났어요"
    TimerDoneBody,
    /// "알람 {time}"
    AlarmTitle,
    ErrTime,
}

fn ko(k: K) -> &'static str {
    match k {
        K::TrayOpen => "열기",
        K::TrayMini => "작은 타이머",
        K::TrayPause => "일시정지",
        K::TrayResume => "재개",
        K::TrayStart => "시작",
        K::TrayReset => "초기화",
        K::TrayQuit => "종료",
        K::StatusWork => "{app} 작업 중",
        K::StatusOther => "대기",
        K::StatusIdle => "자리 비움",
        K::StatusPaused => "일시정지됨",
        K::StatusReady => "시작 전",
        K::StatusBreak => "휴식 중",
        K::Work => "실 작업",
        K::Elapsed => "경과",
        K::AlertBody => "{n}분마다 알림 · 잠깐 쉬어 가세요",
        K::AlertBodyBreak => "{n}분 휴식을 시작해요",
        K::BreakEndTitle => "휴식 끝",
        K::BreakEndBody => "다음 알림은 {n}분 뒤",
        K::ErrExeEmpty => "exe 이름이 비어 있어요",
        K::ErrSoundType => "wav, mp3, ogg 파일만 쓸 수 있어요",
        K::ErrAlertNotFound => "알림을 찾을 수 없어요",
        K::ErrShortcut => "단축키 형식 오류: {e}",
        K::TimerDoneTitle => "타이머 끝",
        K::TimerDoneBody => "{time} 타이머가 끝났어요",
        K::AlarmTitle => "알람 {time}",
        K::ErrTime => "시각 형식이 맞지 않아요",
    }
}

fn en(k: K) -> &'static str {
    match k {
        K::TrayOpen => "Open",
        K::TrayMini => "Mini timer",
        K::TrayPause => "Pause",
        K::TrayResume => "Resume",
        K::TrayStart => "Start",
        K::TrayReset => "Reset",
        K::TrayQuit => "Quit",
        K::StatusWork => "Working in {app}",
        K::StatusOther => "Waiting",
        K::StatusIdle => "Away",
        K::StatusPaused => "Paused",
        K::StatusReady => "Not started",
        K::StatusBreak => "On break",
        K::Work => "Active",
        K::Elapsed => "Elapsed",
        K::AlertBody => "Every {n} min · time for a short rest",
        K::AlertBodyBreak => "Starting a {n}-min break",
        K::BreakEndTitle => "Break over",
        K::BreakEndBody => "Next alert in {n} min",
        K::ErrExeEmpty => "The exe name is empty",
        K::ErrSoundType => "Only wav, mp3 and ogg files are supported",
        K::ErrAlertNotFound => "Alert not found",
        K::ErrShortcut => "Invalid shortcut: {e}",
        K::TimerDoneTitle => "Timer done",
        K::TimerDoneBody => "Your {time} timer is up",
        K::AlarmTitle => "Alarm {time}",
        K::ErrTime => "Invalid time",
    }
}

fn ja(k: K) -> &'static str {
    match k {
        K::TrayOpen => "開く",
        K::TrayMini => "ミニタイマー",
        K::TrayPause => "一時停止",
        K::TrayResume => "再開",
        K::TrayStart => "開始",
        K::TrayReset => "リセット",
        K::TrayQuit => "終了",
        K::StatusWork => "{app} で作業中",
        K::StatusOther => "待機",
        K::StatusIdle => "離席",
        K::StatusPaused => "一時停止中",
        K::StatusReady => "開始前",
        K::StatusBreak => "休憩中",
        K::Work => "実作業",
        K::Elapsed => "経過",
        K::AlertBody => "{n}分ごとの通知 · ひと息つきましょう",
        K::AlertBodyBreak => "{n}分の休憩を始めます",
        K::BreakEndTitle => "休憩終了",
        K::BreakEndBody => "次の通知は{n}分後",
        K::ErrExeEmpty => "exe 名が空です",
        K::ErrSoundType => "wav・mp3・ogg ファイルのみ使えます",
        K::ErrAlertNotFound => "通知が見つかりません",
        K::ErrShortcut => "ショートカットの形式が正しくありません: {e}",
        K::TimerDoneTitle => "タイマー終了",
        K::TimerDoneBody => "{time}のタイマーが終わりました",
        K::AlarmTitle => "アラーム {time}",
        K::ErrTime => "時刻の形式が正しくありません",
    }
}

fn zh(k: K) -> &'static str {
    match k {
        K::TrayOpen => "打开",
        K::TrayMini => "迷你计时器",
        K::TrayPause => "暂停",
        K::TrayResume => "继续",
        K::TrayStart => "开始",
        K::TrayReset => "重置",
        K::TrayQuit => "退出",
        K::StatusWork => "正在用 {app} 工作",
        K::StatusOther => "等待",
        K::StatusIdle => "离开",
        K::StatusPaused => "已暂停",
        K::StatusReady => "未开始",
        K::StatusBreak => "休息中",
        K::Work => "实际工作",
        K::Elapsed => "经过",
        K::AlertBody => "每{n}分钟提醒 · 稍作休息吧",
        K::AlertBodyBreak => "开始休息 {n} 分钟",
        K::BreakEndTitle => "休息结束",
        K::BreakEndBody => "{n}分钟后再次提醒",
        K::ErrExeEmpty => "exe 名称为空",
        K::ErrSoundType => "仅支持 wav、mp3、ogg 文件",
        K::ErrAlertNotFound => "找不到该提醒",
        K::ErrShortcut => "快捷键格式无效：{e}",
        K::TimerDoneTitle => "计时结束",
        K::TimerDoneBody => "{time}的计时器已结束",
        K::AlarmTitle => "闹钟 {time}",
        K::ErrTime => "时间格式无效",
    }
}

pub fn t(lang: &str, k: K) -> &'static str {
    match lang {
        "en" => en(k),
        "ja" => ja(k),
        "zh" => zh(k),
        _ => ko(k),
    }
}

/// {app}, {n} 같은 자리를 채운다
pub fn tf(lang: &str, k: K, args: &[(&str, &str)]) -> String {
    let mut s = t(lang, k).to_string();
    for (name, v) in args {
        s = s.replace(&format!("{{{name}}}"), v);
    }
    s
}

/// 87분 → "1시간 27분" / "1 h 27 min" / "1時間27分" / "1小时27分钟"
pub fn minutes(lang: &str, min: u64) -> String {
    let (h, m) = (min / 60, min % 60);
    let (hu, mu, sep) = match lang {
        "en" => (" h", " min", " "),
        "ja" => ("時間", "分", ""),
        "zh" => ("小时", "分钟", ""),
        _ => ("시간", "분", " "),
    };
    match (h, m) {
        (0, m) => format!("{m}{mu}"),
        (h, 0) => format!("{h}{hu}"),
        (h, m) => format!("{h}{hu}{sep}{m}{mu}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_placeholders_and_falls_back_to_korean() {
        assert_eq!(tf("ko", K::StatusWork, &[("app", "Photoshop")]), "Photoshop 작업 중");
        assert_eq!(tf("en", K::StatusWork, &[("app", "Photoshop")]), "Working in Photoshop");
        assert_eq!(tf("xx", K::AlertBody, &[("n", "50")]), "50분마다 알림 · 잠깐 쉬어 가세요");
        assert_eq!(t("en", K::Work), "Active");
    }

    #[test]
    fn formats_minutes_per_language() {
        assert_eq!(minutes("ko", 50), "50분");
        assert_eq!(minutes("ko", 120), "2시간");
        assert_eq!(minutes("ko", 100), "1시간 40분");
        assert_eq!(minutes("en", 100), "1 h 40 min");
        assert_eq!(minutes("ja", 100), "1時間40分");
        assert_eq!(minutes("zh", 100), "1小时40分钟");
    }

    #[test]
    fn every_language_has_every_placeholder() {
        let keys = [
            (K::StatusWork, "{app}"),
            (K::AlertBody, "{n}"),
            (K::AlertBodyBreak, "{n}"),
            (K::BreakEndBody, "{n}"),
            (K::ErrShortcut, "{e}"),
            (K::TimerDoneBody, "{time}"),
            (K::AlarmTitle, "{time}"),
        ];
        for lang in ["ko", "en", "ja", "zh"] {
            for (k, ph) in keys {
                assert!(t(lang, k).contains(ph), "{lang} {k:?}");
            }
        }
    }
}
