//! 주기 알림: 경과 또는 실 작업 시간이 N분 쌓일 때마다 울린다.
//! 울린 시점부터 다시 센다. 그래서 알림에 휴식(자동 일시정지)이 붙어 있으면
//! 휴식이 끝난 뒤 다시 N분이 지나야 다음 알림이 온다. OS 독립 로직.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertRow {
    pub id: i64,
    pub every_min: u32,
    /// "elapsed" | "work"
    pub basis: String,
    /// 알림 소리. "builtin:bell" 같은 내장 소리 또는 sounds 폴더의 파일 이름. 없으면 기본 종소리
    pub sound: Option<String>,
    pub volume: f32,
    pub enabled: bool,
    /// 알림 뒤 자동 휴식(분). 0이면 휴식 없이 알림만
    pub break_min: u32,
    /// 휴식이 끝날 때 소리
    pub break_sound: Option<String>,
}

impl AlertRow {
    fn period_ms(&self) -> u64 {
        self.every_min.max(1) as u64 * 60_000
    }

    pub fn basis_ms(&self, elapsed_ms: u64, work_ms: u64) -> u64 {
        if self.basis == "elapsed" {
            elapsed_ms
        } else {
            work_ms
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextAlert {
    pub remaining_ms: u64,
    pub every_min: u32,
    pub basis: String,
    pub break_min: u32,
}

/// 알림마다 마지막으로 센 기준점(anchor)을 기억한다.
/// 세션이 바뀌거나 앱을 새로 켜면 기준 시간의 마지막 배수에서, 설정을 바꾸면 그 순간부터 다시 센다.
#[derive(Default)]
pub struct Scheduler {
    session_started_ms: Option<i64>,
    /// id → (every_min, basis, anchor_ms)
    seen: HashMap<i64, (u32, String, u64)>,
}

impl Scheduler {
    fn sync(&mut self, alerts: &[AlertRow], session_started_ms: i64, elapsed_ms: u64, work_ms: u64) {
        let fresh = self.session_started_ms != Some(session_started_ms);
        if fresh {
            self.session_started_ms = Some(session_started_ms);
            self.seen.clear();
        }
        self.seen.retain(|id, _| alerts.iter().any(|a| a.id == *id && a.enabled));
        for a in alerts.iter().filter(|a| a.enabled) {
            let basis = a.basis_ms(elapsed_ms, work_ms);
            match self.seen.get(&a.id) {
                Some((every, b, _)) if *every == a.every_min && *b == a.basis => {}
                Some(_) => {
                    // 간격·기준을 바꿈: 지금부터 다시
                    self.seen.insert(a.id, (a.every_min, a.basis.clone(), basis));
                }
                None => {
                    let anchor = basis - basis % a.period_ms();
                    self.seen.insert(a.id, (a.every_min, a.basis.clone(), anchor));
                }
            }
        }
    }

    /// 이번 tick에 울릴 알림
    pub fn check(
        &mut self,
        alerts: &[AlertRow],
        session_started_ms: i64,
        elapsed_ms: u64,
        work_ms: u64,
    ) -> Vec<AlertRow> {
        self.sync(alerts, session_started_ms, elapsed_ms, work_ms);
        let mut fired = Vec::new();
        for a in alerts.iter().filter(|a| a.enabled) {
            let basis = a.basis_ms(elapsed_ms, work_ms);
            if let Some((_, _, anchor)) = self.seen.get_mut(&a.id) {
                if basis >= *anchor + a.period_ms() {
                    *anchor = basis;
                    fired.push(a.clone());
                }
            }
        }
        fired
    }

    /// 모든 알림을 지금 시점부터 다시 센다(휴식이 끝났을 때).
    /// 휴식 중에도 경과는 흐르므로, 경과 기준 알림도 휴식이 끝난 뒤 N분이 지나야 울린다.
    pub fn rearm(&mut self, alerts: &[AlertRow], session_started_ms: i64, elapsed_ms: u64, work_ms: u64) {
        self.sync(alerts, session_started_ms, elapsed_ms, work_ms);
        for a in alerts.iter().filter(|a| a.enabled) {
            if let Some((_, _, anchor)) = self.seen.get_mut(&a.id) {
                *anchor = a.basis_ms(elapsed_ms, work_ms);
            }
        }
    }

    /// 가장 먼저 울릴 알림(기준 시간으로 남은 양)
    pub fn next_due(
        &mut self,
        alerts: &[AlertRow],
        session_started_ms: i64,
        elapsed_ms: u64,
        work_ms: u64,
    ) -> Option<NextAlert> {
        self.sync(alerts, session_started_ms, elapsed_ms, work_ms);
        alerts
            .iter()
            .filter(|a| a.enabled)
            .filter_map(|a| {
                let (_, _, anchor) = self.seen.get(&a.id)?;
                let done = a.basis_ms(elapsed_ms, work_ms).saturating_sub(*anchor);
                Some(NextAlert {
                    remaining_ms: a.period_ms().saturating_sub(done),
                    every_min: a.every_min,
                    basis: a.basis.clone(),
                    break_min: a.break_min,
                })
            })
            .min_by_key(|n| n.remaining_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: u64 = 60_000;

    fn alert(id: i64, every: u32, basis: &str) -> AlertRow {
        AlertRow {
            id,
            every_min: every,
            basis: basis.into(),
            sound: None,
            volume: 0.8,
            enabled: true,
            break_min: 0,
            break_sound: None,
        }
    }

    fn ids(v: &[AlertRow]) -> Vec<i64> {
        v.iter().map(|a| a.id).collect()
    }

    #[test]
    fn fires_once_per_period_on_its_basis() {
        let alerts = vec![alert(1, 50, "work"), alert(2, 30, "elapsed")];
        let mut s = Scheduler::default();
        assert!(s.check(&alerts, 0, 0, 0).is_empty());
        assert!(s.check(&alerts, 0, 29 * MIN, 29 * MIN).is_empty());
        assert_eq!(ids(&s.check(&alerts, 0, 30 * MIN, 20 * MIN)), vec![2]);
        assert!(s.check(&alerts, 0, 31 * MIN, 21 * MIN).is_empty());
        assert_eq!(ids(&s.check(&alerts, 0, 60 * MIN, 50 * MIN)), vec![1, 2]);
    }

    #[test]
    fn next_alert_counts_from_when_it_fired() {
        // 90분 알림이 90분에 울림 → (휴식으로 시간이 멈췄다가) 다시 90분 뒤
        let alerts = vec![alert(1, 90, "elapsed")];
        let mut s = Scheduler::default();
        s.check(&alerts, 0, 0, 0);
        assert_eq!(s.check(&alerts, 0, 90 * MIN, 0).len(), 1);
        assert_eq!(s.next_due(&alerts, 0, 90 * MIN, 0).unwrap().remaining_ms, 90 * MIN);
        assert!(s.check(&alerts, 0, 179 * MIN, 0).is_empty());
        assert_eq!(s.check(&alerts, 0, 180 * MIN, 0).len(), 1);
    }

    #[test]
    fn rearm_after_break_counts_full_period_again() {
        // 90분 경과 알림 → 5분 휴식(경과는 흐름) → 휴식 끝에서 다시 90분
        let alerts = vec![alert(1, 90, "elapsed")];
        let mut s = Scheduler::default();
        s.check(&alerts, 0, 0, 0);
        assert_eq!(s.check(&alerts, 0, 90 * MIN, 90 * MIN).len(), 1);
        s.rearm(&alerts, 0, 95 * MIN, 90 * MIN);
        assert_eq!(s.next_due(&alerts, 0, 95 * MIN, 90 * MIN).unwrap().remaining_ms, 90 * MIN);
        assert!(s.check(&alerts, 0, 184 * MIN, 179 * MIN).is_empty());
        assert_eq!(s.check(&alerts, 0, 185 * MIN, 180 * MIN).len(), 1);
    }

    #[test]
    fn does_not_fire_on_start_and_edit_restarts_count() {
        let mut s = Scheduler::default();
        // 앱을 다시 켰을 때 이미 120분 지난 세션: 마지막 배수(100분)부터
        assert!(s.check(&[alert(1, 50, "work")], 0, 0, 120 * MIN).is_empty());
        assert_eq!(s.next_due(&[alert(1, 50, "work")], 0, 0, 120 * MIN).unwrap().remaining_ms, 30 * MIN);
        // 간격을 바꾸면 그 순간부터 다시 센다
        assert!(s.check(&[alert(1, 25, "work")], 0, 0, 122 * MIN).is_empty());
        assert!(s.check(&[alert(1, 25, "work")], 0, 0, 146 * MIN).is_empty());
        assert_eq!(s.check(&[alert(1, 25, "work")], 0, 0, 147 * MIN).len(), 1);
    }

    #[test]
    fn new_session_restarts_count_and_disabled_is_silent() {
        let mut s = Scheduler::default();
        let mut a = alert(1, 10, "elapsed");
        s.check(&[a.clone()], 0, 9 * MIN, 0);
        // 초기화 → 새 세션
        assert!(s.check(&[a.clone()], 1, 0, 0).is_empty());
        assert_eq!(s.check(&[a.clone()], 1, 10 * MIN, 0).len(), 1);
        a.enabled = false;
        assert!(s.check(&[a.clone()], 1, 20 * MIN, 0).is_empty());
        assert!(s.next_due(&[a], 1, 20 * MIN, 0).is_none());
    }

    #[test]
    fn next_due_picks_soonest_enabled() {
        let mut off = alert(3, 5, "elapsed");
        off.enabled = false;
        let alerts = vec![alert(1, 50, "work"), alert(2, 30, "elapsed"), off];
        let mut s = Scheduler::default();
        let n = s.next_due(&alerts, 0, 25 * MIN, 42 * MIN).unwrap();
        assert_eq!((n.remaining_ms, n.every_min), (5 * MIN, 30));
        assert!(s.next_due(&[], 0, 0, 0).is_none());
    }
}
