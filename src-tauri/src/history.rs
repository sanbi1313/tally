//! 기록 탭: segments를 날짜·세션·프로그램별로 모은다. CSV 내보내기도 여기서.
//! 날짜(day)는 기록할 때 이미 하루 경계(새벽 5시)를 적용해 둔 값이다.

use crate::engine::display_name;
use crate::store::Store;
use rusqlite::params;
use serde::Serialize;

type R<T> = rusqlite::Result<T>;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DayTotal {
    pub day: String,
    pub work_ms: u64,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Seg {
    pub start_ms: i64,
    pub end_ms: i64,
    /// work | other | idle | paused
    pub kind: String,
    pub label: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    pub id: i64,
    pub note: Option<String>,
    /// 그날 안에서 이 세션의 첫·마지막 시각
    pub start_ms: i64,
    pub end_ms: i64,
    pub work_ms: u64,
    /// 작업한 프로그램(많이 한 순)
    pub apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DayDetail {
    pub segments: Vec<Seg>,
    pub sessions: Vec<SessionSummary>,
}

/// from..=to 사이 기록이 있는 날의 합계
pub fn day_totals(st: &Store, from: &str, to: &str) -> R<Vec<DayTotal>> {
    let mut q = st.conn().prepare(
        "SELECT day,
                COALESCE(SUM(CASE WHEN kind = 'work' THEN end_ms - start_ms END), 0),
                COALESCE(SUM(CASE WHEN kind != 'paused' THEN end_ms - start_ms END), 0)
         FROM segments WHERE day BETWEEN ?1 AND ?2 GROUP BY day ORDER BY day",
    )?;
    let rows = q.query_map(params![from, to], |r| {
        Ok(DayTotal { day: r.get(0)?, work_ms: r.get::<_, i64>(1)? as u64, elapsed_ms: r.get::<_, i64>(2)? as u64 })
    })?;
    rows.collect()
}

pub fn day_detail(st: &Store, day: &str) -> R<DayDetail> {
    let segments: Vec<(Seg, i64, Option<i64>, Option<String>)> = {
        let mut q = st.conn().prepare(
            "SELECT s.start_ms, s.end_ms, s.kind, a.label, a.color, s.session_id, s.app_id, s.exe
             FROM segments s LEFT JOIN apps a ON a.id = s.app_id
             WHERE s.day = ?1 ORDER BY s.start_ms",
        )?;
        let rows = q.query_map([day], |r| {
            let kind: String = r.get(2)?;
            let exe: Option<String> = r.get(7)?;
            let label: Option<String> = r.get(3)?;
            let label = if kind == "work" { label.or_else(|| exe.as_deref().map(display_name)) } else { None };
            Ok((
                Seg { start_ms: r.get(0)?, end_ms: r.get(1)?, kind, label, color: r.get(4)? },
                r.get::<_, Option<i64>>(5)?.unwrap_or(0),
                r.get(6)?,
                exe,
            ))
        })?;
        rows.collect::<R<_>>()?
    };

    // 세션별로 모은다(세션 순서는 그날 처음 나온 순)
    let mut sessions: Vec<SessionSummary> = Vec::new();
    let mut per_app: Vec<Vec<(String, u64)>> = Vec::new();
    for (seg, sid, _, _) in &segments {
        let idx = match sessions.iter().position(|s| s.id == *sid) {
            Some(i) => i,
            None => {
                sessions.push(SessionSummary {
                    id: *sid,
                    note: None,
                    start_ms: seg.start_ms,
                    end_ms: seg.end_ms,
                    work_ms: 0,
                    apps: Vec::new(),
                });
                per_app.push(Vec::new());
                sessions.len() - 1
            }
        };
        let s = &mut sessions[idx];
        s.start_ms = s.start_ms.min(seg.start_ms);
        s.end_ms = s.end_ms.max(seg.end_ms);
        if seg.kind == "work" {
            let d = (seg.end_ms - seg.start_ms).max(0) as u64;
            s.work_ms += d;
            let label = seg.label.clone().unwrap_or_default();
            match per_app[idx].iter_mut().find(|(l, _)| *l == label) {
                Some((_, t)) => *t += d,
                None => per_app[idx].push((label, d)),
            }
        }
    }
    for (s, mut apps) in sessions.iter_mut().zip(per_app) {
        apps.sort_by(|a, b| b.1.cmp(&a.1));
        s.apps = apps.into_iter().map(|(l, _)| l).collect();
        s.note = st
            .conn()
            .query_row("SELECT note FROM sessions WHERE id = ?1", [s.id], |r| r.get(0))
            .unwrap_or(None);
    }
    // 실 작업이 없는 세션(대기만 있던 짧은 세션)은 목록에서 뺀다
    sessions.retain(|s| s.work_ms > 0);
    Ok(DayDetail { segments: segments.into_iter().map(|(s, ..)| s).collect(), sessions })
}

pub fn set_session_note(st: &Store, id: i64, note: Option<&str>) -> R<()> {
    st.conn().execute(
        "UPDATE sessions SET note = ?2 WHERE id = ?1",
        params![id, note.map(str::trim).filter(|n| !n.is_empty())],
    )?;
    Ok(())
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// 구간 하나가 한 줄. 엑셀에서 한글이 깨지지 않게 UTF-8 BOM을 붙인다.
/// local_time: unix ms → 현지 시각 문자열
pub fn export_csv(st: &Store, from: &str, to: &str, local_time: impl Fn(i64) -> String) -> R<(String, usize)> {
    let mut q = st.conn().prepare(
        "SELECT s.day, s.session_id, ss.note, s.start_ms, s.end_ms, s.kind, a.label, s.exe
         FROM segments s
         LEFT JOIN apps a ON a.id = s.app_id
         LEFT JOIN sessions ss ON ss.id = s.session_id
         WHERE s.day BETWEEN ?1 AND ?2 ORDER BY s.start_ms",
    )?;
    let mut out = String::from("\u{feff}day,session,note,start,end,kind,program,exe,seconds\r\n");
    let mut n = 0;
    let mut rows = q.query(params![from, to])?;
    while let Some(r) = rows.next()? {
        let (start, end): (i64, i64) = (r.get(3)?, r.get(4)?);
        let exe: Option<String> = r.get(7)?;
        let label: Option<String> = r.get(6)?;
        let program = label.or_else(|| exe.as_deref().map(display_name)).unwrap_or_default();
        let fields = [
            r.get::<_, String>(0)?,
            r.get::<_, Option<i64>>(1)?.map(|v| v.to_string()).unwrap_or_default(),
            r.get::<_, Option<String>>(2)?.unwrap_or_default(),
            local_time(start),
            local_time(end),
            r.get::<_, String>(5)?,
            program,
            exe.unwrap_or_default(),
            format!("{:.0}", (end - start).max(0) as f64 / 1000.0),
        ];
        out += &fields.iter().map(|f| csv_field(f)).collect::<Vec<_>>().join(",");
        out += "\r\n";
        n += 1;
    }
    Ok((out, n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{AppRef, Interval, Kind};

    fn iv(day: &str, start: i64, end: i64, kind: Kind, app: Option<(i64, &str)>) -> Interval {
        Interval {
            start_ms: start,
            end_ms: end,
            kind,
            app: app.map(|(id, exe)| AppRef {
                key: AppRef::key_of(Some(id), exe),
                label: exe.into(),
                app_id: Some(id),
                exe: exe.into(),
                color: None,
            }),
            day: day.into(),
        }
    }

    /// 두 날, 세 세션
    fn seeded() -> (Store, i64, i64) {
        let mut st = Store::memory().unwrap();
        let a = st.add_app("CLIPStudioPaint.exe", None, None).unwrap().id;
        let b = st.add_app("Photoshop.exe", None, None).unwrap().id;
        let d1 = "2026-10-31";
        let d2 = "2026-11-01";
        st.start_session(0).unwrap();
        st.push(&iv(d1, 0, 60_000, Kind::Work, Some((a, "CLIPStudioPaint.exe")))).unwrap();
        st.push(&iv(d1, 60_000, 90_000, Kind::Idle, None)).unwrap();
        st.push(&iv(d1, 90_000, 100_000, Kind::Paused, None)).unwrap();
        st.end_session(100_000, Some("러프, \"표지\"")).unwrap();
        st.start_session(1_000_000).unwrap();
        st.push(&iv(d2, 1_000_000, 1_030_000, Kind::Work, Some((b, "Photoshop.exe")))).unwrap();
        st.push(&iv(d2, 1_030_000, 1_040_000, Kind::Other, None)).unwrap();
        st.push(&iv(d2, 1_040_000, 1_100_000, Kind::Work, Some((a, "CLIPStudioPaint.exe")))).unwrap();
        st.end_session(1_100_000, None).unwrap();
        // 대기만 있는 세션
        st.start_session(2_000_000).unwrap();
        st.push(&iv(d2, 2_000_000, 2_010_000, Kind::Other, None)).unwrap();
        st.end_session(2_010_000, None).unwrap();
        (st, a, b)
    }

    #[test]
    fn totals_per_day_exclude_paused_from_elapsed() {
        let (st, ..) = seeded();
        let t = day_totals(&st, "2026-10-01", "2026-11-30").unwrap();
        assert_eq!(
            t,
            vec![
                DayTotal { day: "2026-10-31".into(), work_ms: 60_000, elapsed_ms: 90_000 },
                DayTotal { day: "2026-11-01".into(), work_ms: 90_000, elapsed_ms: 110_000 },
            ]
        );
        assert!(day_totals(&st, "2026-12-01", "2026-12-31").unwrap().is_empty());
    }

    #[test]
    fn day_detail_groups_sessions_and_programs() {
        let (st, ..) = seeded();
        let d = day_detail(&st, "2026-11-01").unwrap();
        assert_eq!(d.segments.len(), 4);
        assert_eq!(d.segments[0].label.as_deref(), Some("Photoshop"));
        assert_eq!(d.segments[1].label, None); // 대기
        assert_eq!(d.sessions.len(), 1); // 대기만 있던 세션은 빠짐
        let s = &d.sessions[0];
        assert_eq!((s.start_ms, s.end_ms, s.work_ms), (1_000_000, 1_100_000, 90_000));
        assert_eq!(s.apps, vec!["CLIP STUDIO PAINT", "Photoshop"]);

        set_session_note(&st, s.id, Some("  채색 ")).unwrap();
        assert_eq!(day_detail(&st, "2026-11-01").unwrap().sessions[0].note.as_deref(), Some("채색"));
    }

    #[test]
    fn csv_has_bom_header_and_escapes() {
        let (st, ..) = seeded();
        let (csv, n) = export_csv(&st, "2026-10-31", "2026-10-31", |ms| format!("t{ms}")).unwrap();
        assert_eq!(n, 3);
        let lines: Vec<_> = csv.split("\r\n").collect();
        assert_eq!(lines[0], "\u{feff}day,session,note,start,end,kind,program,exe,seconds");
        assert_eq!(
            lines[1],
            "2026-10-31,1,\"러프, \"\"표지\"\"\",t0,t60000,work,CLIP STUDIO PAINT,CLIPStudioPaint.exe,60"
        );
        assert!(lines[3].contains(",paused,"));
    }
}
