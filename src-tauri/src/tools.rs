//! 도구 탭: 타이머, 스톱워치, 시각 알람. 작업 추적과 독립돼 있고 창을 닫아도 돈다.
//! 시각은 모두 unix ms로 받는 순수 로직이라 테스트로 검증한다.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// ---- 타이머 ----

#[derive(Debug, Clone, Default)]
pub struct Timer {
    duration_ms: u64,
    /// 멈춰 있을 때 남은 시간
    left_ms: u64,
    /// 돌고 있으면 끝나는 시각
    ends_at_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimerView {
    pub duration_ms: u64,
    pub remaining_ms: u64,
    /// 돌고 있으면 끝나는 시각(화면이 매끄럽게 줄이도록)
    pub ends_at_ms: Option<i64>,
}

impl Timer {
    /// 시간을 정하고 멈춘 상태로 둔다
    pub fn set(&mut self, duration_ms: u64) {
        self.duration_ms = duration_ms;
        self.left_ms = duration_ms;
        self.ends_at_ms = None;
    }

    pub fn start(&mut self, now: i64) {
        if self.ends_at_ms.is_none() && self.left_ms > 0 {
            self.ends_at_ms = Some(now + self.left_ms as i64);
        }
    }

    pub fn pause(&mut self, now: i64) {
        if let Some(end) = self.ends_at_ms.take() {
            self.left_ms = (end - now).max(0) as u64;
        }
    }

    /// 처음 정한 시간으로 되돌린다
    pub fn reset(&mut self) {
        self.left_ms = self.duration_ms;
        self.ends_at_ms = None;
    }

    pub fn remaining(&self, now: i64) -> u64 {
        match self.ends_at_ms {
            Some(end) => (end - now).max(0) as u64,
            None => self.left_ms,
        }
    }

    /// 끝났으면 한 번만 true. 끝나면 처음 시간으로 돌아가 다시 시작할 수 있게 한다.
    pub fn check_done(&mut self, now: i64) -> bool {
        match self.ends_at_ms {
            Some(end) if now >= end => {
                self.reset();
                true
            }
            _ => false,
        }
    }

    pub fn view(&self, now: i64) -> TimerView {
        TimerView { duration_ms: self.duration_ms, remaining_ms: self.remaining(now), ends_at_ms: self.ends_at_ms }
    }
}

// ---- 스톱워치 ----

#[derive(Debug, Clone, Default)]
pub struct Stopwatch {
    acc_ms: u64,
    since_ms: Option<i64>,
    /// 랩 시각(시작부터 누적)
    laps: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopwatchView {
    /// 멈춘 시점까지 누적
    pub acc_ms: u64,
    /// 돌고 있으면 다시 시작한 시각. 화면은 acc + (지금 - since)로 그린다
    pub since_ms: Option<i64>,
    /// 랩마다 걸린 시간(구간), 첫 랩부터
    pub laps: Vec<u64>,
}

impl Stopwatch {
    pub fn elapsed(&self, now: i64) -> u64 {
        self.acc_ms + self.since_ms.map_or(0, |s| (now - s).max(0) as u64)
    }

    pub fn start(&mut self, now: i64) {
        if self.since_ms.is_none() {
            self.since_ms = Some(now);
        }
    }

    pub fn stop(&mut self, now: i64) {
        self.acc_ms = self.elapsed(now);
        self.since_ms = None;
    }

    pub fn lap(&mut self, now: i64) {
        if self.since_ms.is_some() {
            self.laps.push(self.elapsed(now));
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn view(&self) -> StopwatchView {
        let mut prev = 0;
        let laps = self
            .laps
            .iter()
            .map(|&t| {
                let d = t - prev;
                prev = t;
                d
            })
            .collect();
        StopwatchView { acc_ms: self.acc_ms, since_ms: self.since_ms, laps }
    }
}

// ---- 시각 알람 ----

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlarmRow {
    pub id: i64,
    /// "07:40"
    pub time_hm: String,
    /// 요일 비트: 월=1, 화=2, … 일=64. 0이면 한 번만(울리고 꺼짐)
    pub weekdays: u8,
    pub label: Option<String>,
    pub sound: Option<String>,
    pub enabled: bool,
}

/// "7:5" 같은 입력도 "07:05"로. 형식이 틀리면 None
pub fn norm_hm(s: &str) -> Option<String> {
    let (h, m) = s.trim().split_once(':')?;
    let (h, m): (u32, u32) = (h.trim().parse().ok()?, m.trim().parse().ok()?);
    (h < 24 && m < 60).then(|| format!("{h:02}:{m:02}"))
}

/// 지금(현지 시각) 울릴 알람. 같은 알람은 같은 날 같은 분에 한 번만.
#[derive(Default)]
pub struct AlarmClock {
    fired: HashSet<(i64, String)>,
}

impl AlarmClock {
    pub fn due(&mut self, alarms: &[AlarmRow], local: chrono::NaiveDateTime) -> Vec<AlarmRow> {
        use chrono::Datelike;
        let hm = local.format("%H:%M").to_string();
        let stamp = local.format("%Y-%m-%d %H:%M").to_string();
        let bit = 1u8 << local.weekday().num_days_from_monday();
        // 지난 분의 기록은 버린다
        self.fired.retain(|(_, s)| *s == stamp);
        alarms
            .iter()
            .filter(|a| a.enabled && a.time_hm == hm && (a.weekdays == 0 || a.weekdays & bit != 0))
            .filter(|a| self.fired.insert((a.id, stamp.clone())))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn timer_counts_down_pauses_and_fires_once() {
        let mut t = Timer::default();
        t.set(25 * 60_000);
        assert_eq!(t.remaining(0), 1_500_000);
        t.start(1_000);
        assert_eq!(t.remaining(61_000), 1_440_000);
        t.pause(61_000);
        assert_eq!(t.remaining(999_999), 1_440_000); // 멈추면 그대로
        t.start(100_000);
        assert!(!t.check_done(100_000 + 1_439_999));
        assert!(t.check_done(100_000 + 1_440_000));
        assert!(!t.check_done(100_000 + 1_440_500));
        // 끝나면 처음 시간으로
        assert_eq!(t.view(0), TimerView { duration_ms: 1_500_000, remaining_ms: 1_500_000, ends_at_ms: None });
    }

    #[test]
    fn timer_reset_and_zero_does_not_start() {
        let mut t = Timer::default();
        t.start(0);
        assert_eq!(t.view(0).ends_at_ms, None);
        t.set(60_000);
        t.start(0);
        t.reset();
        assert_eq!(t.remaining(30_000), 60_000);
        assert_eq!(t.view(0).ends_at_ms, None);
    }

    #[test]
    fn stopwatch_laps_are_splits() {
        let mut s = Stopwatch::default();
        s.start(0);
        s.lap(5_000);
        s.stop(8_000);
        assert_eq!(s.elapsed(100_000), 8_000);
        s.lap(9_000); // 멈춘 동안 랩은 무시
        s.start(10_000);
        s.lap(15_000);
        let v = s.view();
        assert_eq!(v.laps, vec![5_000, 8_000]);
        assert_eq!(v.acc_ms, 8_000);
        assert_eq!(v.since_ms, Some(10_000));
        s.reset();
        assert_eq!(s.elapsed(20_000), 0);
    }

    fn alarm(id: i64, hm: &str, weekdays: u8) -> AlarmRow {
        AlarmRow { id, time_hm: hm.into(), weekdays, label: None, sound: None, enabled: true }
    }

    #[test]
    fn alarm_fires_on_matching_weekday_once_per_minute() {
        // 2026-11-02 = 월요일
        let mon = |h, m, s| NaiveDate::from_ymd_opt(2026, 11, 2).unwrap().and_hms_opt(h, m, s).unwrap();
        let alarms = vec![alarm(1, "07:40", 0b0011111), alarm(2, "07:40", 0b1100000), alarm(3, "07:41", 0)];
        let mut c = AlarmClock::default();
        assert!(c.due(&alarms, mon(7, 39, 59)).is_empty());
        let ids: Vec<_> = c.due(&alarms, mon(7, 40, 0)).iter().map(|a| a.id).collect();
        assert_eq!(ids, vec![1]); // 평일 알람만, 주말 알람은 아님
        assert!(c.due(&alarms, mon(7, 40, 30)).is_empty()); // 같은 분에 두 번 안 울림
        assert_eq!(c.due(&alarms, mon(7, 41, 5)).len(), 1); // 한 번만 알람은 요일 무관
        let mut off = alarm(4, "07:42", 127);
        off.enabled = false;
        assert!(c.due(&[off], mon(7, 42, 0)).is_empty());
    }

    #[test]
    fn normalizes_time_input() {
        assert_eq!(norm_hm("7:5").as_deref(), Some("07:05"));
        assert_eq!(norm_hm("23:30").as_deref(), Some("23:30"));
        assert_eq!(norm_hm("24:00"), None);
        assert_eq!(norm_hm("abc"), None);
    }
}
