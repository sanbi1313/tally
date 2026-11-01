//! SQLite 저장소(%APPDATA%\Tally\tally.db).
//! 엔진이 내놓는 구간을 이어 붙여 segments 행으로 기록한다. 진행 중인 행은
//! 상태가 바뀔 때와 1분마다 end_ms를 갱신하므로, 강제 종료돼도 잃는 건 최대 1분.

use crate::alerts::AlertRow;
use crate::tools::AlarmRow;
use crate::engine::{display_name, AppRef, AppTime, Interval, Kind, TrackedApp};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const FLUSH_EVERY_MS: i64 = 60_000;

/// 막대 색 순서. 테마 토큰 이름으로 저장해 라이트/다크에 따라 바뀌게 한다.
pub const COLORS: [&str; 4] = ["accent", "ink", "sub", "idle"];

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS apps (
  id INTEGER PRIMARY KEY,
  exe TEXT UNIQUE NOT NULL COLLATE NOCASE,
  label TEXT NOT NULL,
  color TEXT,
  title_kw TEXT,
  enabled INTEGER DEFAULT 1
);
CREATE TABLE IF NOT EXISTS sessions (
  id INTEGER PRIMARY KEY,
  started_at INTEGER NOT NULL,
  ended_at INTEGER,
  note TEXT
);
CREATE TABLE IF NOT EXISTS segments (
  id INTEGER PRIMARY KEY,
  session_id INTEGER REFERENCES sessions(id),
  start_ms INTEGER NOT NULL,
  end_ms INTEGER NOT NULL,
  kind TEXT NOT NULL,
  app_id INTEGER REFERENCES apps(id) ON DELETE SET NULL,
  exe TEXT,
  day TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_seg_day ON segments(day);
CREATE INDEX IF NOT EXISTS idx_seg_session ON segments(session_id);
CREATE TABLE IF NOT EXISTS alerts (
  id INTEGER PRIMARY KEY,
  every_min INTEGER NOT NULL,
  basis TEXT NOT NULL,
  sound_path TEXT,
  volume REAL DEFAULT 0.8,
  enabled INTEGER DEFAULT 1
);
CREATE TABLE IF NOT EXISTS alarms (
  id INTEGER PRIMARY KEY,
  time_hm TEXT NOT NULL,
  weekdays INTEGER,
  label TEXT, sound_path TEXT, enabled INTEGER DEFAULT 1
);
";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRow {
    pub id: i64,
    pub exe: String,
    pub label: String,
    pub color: Option<String>,
    pub title_kw: Option<String>,
    pub enabled: bool,
}

impl AppRow {
    pub fn to_tracked(&self) -> TrackedApp {
        TrackedApp {
            id: Some(self.id),
            exe: self.exe.clone(),
            label: self.label.clone(),
            title_kw: self.title_kw.clone().filter(|k| !k.trim().is_empty()),
            color: self.color.clone(),
        }
    }
}

/// 이어서 셀 세션
pub struct Resume {
    pub started_ms: i64,
    pub elapsed_ms: u64,
    pub work_ms: u64,
    pub apps: Vec<AppTime>,
}

#[derive(Debug, Clone)]
struct OpenSeg {
    id: i64,
    end_ms: i64,
    flushed_end_ms: i64,
    kind: Kind,
    app_id: Option<i64>,
    exe: Option<String>,
    day: String,
}

pub struct Store {
    conn: Connection,
    session_id: Option<i64>,
    open: Option<OpenSeg>,
}

type R<T> = rusqlite::Result<T>;

impl Store {
    pub fn open(path: &Path) -> R<Self> {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        Self::init(Connection::open(path)?)
    }

    #[cfg(test)]
    pub fn memory() -> R<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> R<Self> {
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        // 3단계 추가 열: 자동 휴식
        let has_break: bool = conn
            .prepare("SELECT 1 FROM pragma_table_info('alerts') WHERE name = 'break_min'")?
            .exists([])?;
        if !has_break {
            conn.execute_batch(
                "ALTER TABLE alerts ADD COLUMN break_min INTEGER DEFAULT 0;
                 ALTER TABLE alerts ADD COLUMN break_sound TEXT;",
            )?;
        }
        Ok(Self { conn, session_id: None, open: None })
    }

    /// 기록 조회(history.rs)용
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    // ---- 추적 프로그램 ----

    pub fn list_apps(&self) -> R<Vec<AppRow>> {
        let mut st = self
            .conn
            .prepare("SELECT id, exe, label, color, title_kw, enabled FROM apps ORDER BY id")?;
        let rows = st.query_map([], |r| {
            Ok(AppRow {
                id: r.get(0)?,
                exe: r.get(1)?,
                label: r.get(2)?,
                color: r.get(3)?,
                title_kw: r.get(4)?,
                enabled: r.get::<_, i64>(5)? != 0,
            })
        })?;
        rows.collect()
    }

    pub fn tracked(&self) -> R<Vec<TrackedApp>> {
        Ok(self.list_apps()?.iter().filter(|a| a.enabled).map(AppRow::to_tracked).collect())
    }

    /// 이미 있는 exe면 다시 켜고 그 행을 돌려준다.
    pub fn add_app(&self, exe: &str, label: Option<&str>, title_kw: Option<&str>) -> R<AppRow> {
        let exe = exe.trim();
        if let Some(id) = self
            .conn
            .query_row("SELECT id FROM apps WHERE exe = ?1", [exe], |r| r.get::<_, i64>(0))
            .optional()?
        {
            self.conn.execute("UPDATE apps SET enabled = 1 WHERE id = ?1", [id])?;
            return self.get_app(id);
        }
        let n: i64 = self.conn.query_row("SELECT COUNT(*) FROM apps", [], |r| r.get(0))?;
        let color = COLORS[n as usize % COLORS.len()];
        let label = label
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| display_name(exe));
        self.conn.execute(
            "INSERT INTO apps (exe, label, color, title_kw, enabled) VALUES (?1, ?2, ?3, ?4, 1)",
            params![exe, label, color, title_kw.filter(|k| !k.trim().is_empty())],
        )?;
        self.get_app(self.conn.last_insert_rowid())
    }

    pub fn get_app(&self, id: i64) -> R<AppRow> {
        self.list_apps()?
            .into_iter()
            .find(|a| a.id == id)
            .ok_or(rusqlite::Error::QueryReturnedNoRows)
    }

    pub fn update_app(&self, a: &AppRow) -> R<()> {
        let label = if a.label.trim().is_empty() { display_name(&a.exe) } else { a.label.trim().to_string() };
        self.conn.execute(
            "UPDATE apps SET label = ?2, color = ?3, title_kw = ?4, enabled = ?5 WHERE id = ?1",
            params![
                a.id,
                label,
                a.color,
                a.title_kw.as_deref().map(str::trim).filter(|k| !k.is_empty()),
                a.enabled as i64
            ],
        )?;
        Ok(())
    }

    /// 지운 프로그램의 지난 기록은 exe 이름으로 남는다(app_id만 NULL).
    pub fn delete_app(&self, id: i64) -> R<()> {
        self.conn.execute("DELETE FROM apps WHERE id = ?1", [id])?;
        Ok(())
    }

    // ---- 주기 알림 ----

    pub fn list_alerts(&self) -> R<Vec<AlertRow>> {
        let mut st = self
            .conn
            .prepare("SELECT id, every_min, basis, sound_path, volume, enabled, break_min, break_sound FROM alerts ORDER BY id")?;
        let rows = st.query_map([], |r| {
            Ok(AlertRow {
                id: r.get(0)?,
                every_min: r.get::<_, i64>(1)?.clamp(1, 24 * 60) as u32,
                basis: r.get(2)?,
                sound: r.get(3)?,
                volume: r.get::<_, f64>(4)? as f32,
                enabled: r.get::<_, i64>(5)? != 0,
                break_min: r.get::<_, Option<i64>>(6)?.unwrap_or(0).clamp(0, 240) as u32,
                break_sound: r.get(7)?,
            })
        })?;
        rows.collect()
    }

    pub fn add_alert(&self, every_min: u32, basis: &str) -> R<AlertRow> {
        self.conn.execute(
            "INSERT INTO alerts (every_min, basis, volume, enabled) VALUES (?1, ?2, 0.8, 1)",
            params![every_min.clamp(1, 24 * 60), norm_basis(basis)],
        )?;
        let id = self.conn.last_insert_rowid();
        self.list_alerts()?.into_iter().find(|a| a.id == id).ok_or(rusqlite::Error::QueryReturnedNoRows)
    }

    pub fn update_alert(&self, a: &AlertRow) -> R<()> {
        self.conn.execute(
            "UPDATE alerts SET every_min = ?2, basis = ?3, sound_path = ?4, volume = ?5, enabled = ?6,
                 break_min = ?7, break_sound = ?8 WHERE id = ?1",
            params![
                a.id,
                a.every_min.clamp(1, 24 * 60),
                norm_basis(&a.basis),
                a.sound,
                a.volume.clamp(0.0, 1.0) as f64,
                a.enabled as i64,
                a.break_min.min(240),
                a.break_sound
            ],
        )?;
        Ok(())
    }

    pub fn delete_alert(&self, id: i64) -> R<()> {
        self.conn.execute("DELETE FROM alerts WHERE id = ?1", [id])?;
        Ok(())
    }

    // ---- 시각 알람 ----

    pub fn list_alarms(&self) -> R<Vec<AlarmRow>> {
        let mut st = self
            .conn
            .prepare("SELECT id, time_hm, weekdays, label, sound_path, enabled FROM alarms ORDER BY time_hm, id")?;
        let rows = st.query_map([], |r| {
            Ok(AlarmRow {
                id: r.get(0)?,
                time_hm: r.get(1)?,
                weekdays: r.get::<_, Option<i64>>(2)?.unwrap_or(0).clamp(0, 127) as u8,
                label: r.get(3)?,
                sound: r.get(4)?,
                enabled: r.get::<_, i64>(5)? != 0,
            })
        })?;
        rows.collect()
    }

    pub fn add_alarm(&self, time_hm: &str, weekdays: u8, label: Option<&str>) -> R<AlarmRow> {
        self.conn.execute(
            "INSERT INTO alarms (time_hm, weekdays, label, enabled) VALUES (?1, ?2, ?3, 1)",
            params![time_hm, weekdays & 127, label.map(str::trim).filter(|l| !l.is_empty())],
        )?;
        let id = self.conn.last_insert_rowid();
        self.list_alarms()?.into_iter().find(|a| a.id == id).ok_or(rusqlite::Error::QueryReturnedNoRows)
    }

    pub fn update_alarm(&self, a: &AlarmRow) -> R<()> {
        self.conn.execute(
            "UPDATE alarms SET time_hm = ?2, weekdays = ?3, label = ?4, sound_path = ?5, enabled = ?6 WHERE id = ?1",
            params![
                a.id,
                a.time_hm,
                a.weekdays & 127,
                a.label.as_deref().map(str::trim).filter(|l| !l.is_empty()),
                a.sound,
                a.enabled as i64
            ],
        )?;
        Ok(())
    }

    pub fn delete_alarm(&self, id: i64) -> R<()> {
        self.conn.execute("DELETE FROM alarms WHERE id = ?1", [id])?;
        Ok(())
    }

    // ---- 세션 ----

    /// 지금 기록 중인 세션
    pub fn current_session(&self) -> Option<i64> {
        self.session_id
    }

    pub fn start_session(&mut self, now_ms: i64) -> R<i64> {
        self.conn.execute("INSERT INTO sessions (started_at) VALUES (?1)", [now_ms])?;
        let id = self.conn.last_insert_rowid();
        self.session_id = Some(id);
        Ok(id)
    }

    /// 진행 중인 구간을 닫고 세션을 끝낸다(초기화·세션 종료·날짜 변경).
    pub fn end_session(&mut self, now_ms: i64, note: Option<&str>) -> R<()> {
        self.close_segment()?;
        if let Some(id) = self.session_id.take() {
            self.conn.execute(
                "UPDATE sessions SET ended_at = ?2, note = ?3 WHERE id = ?1",
                params![id, now_ms, note.map(str::trim).filter(|n| !n.is_empty())],
            )?;
        }
        Ok(())
    }

    /// 앱을 켤 때: 끝나지 않은 세션 중 오늘 것은 이어 쓰고, 나머지는 마지막 기록 시각으로 닫는다.
    pub fn take_resumable(&mut self, today: &str, day_of_ms: impl Fn(i64) -> String) -> R<Option<Resume>> {
        let open: Vec<(i64, i64)> = {
            let mut st = self
                .conn
                .prepare("SELECT id, started_at FROM sessions WHERE ended_at IS NULL ORDER BY id DESC")?;
            let rows = st.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
            rows.collect::<R<_>>()?
        };
        let mut found = None;
        for (i, (id, started)) in open.into_iter().enumerate() {
            let last: Option<(i64, String)> = self
                .conn
                .query_row(
                    "SELECT end_ms, day FROM segments WHERE session_id = ?1 ORDER BY end_ms DESC LIMIT 1",
                    [id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()?;
            let day = last.as_ref().map(|l| l.1.clone()).unwrap_or_else(|| day_of_ms(started));
            if i == 0 && day == today {
                found = Some((id, started));
                continue;
            }
            let end = last.map_or(started, |l| l.0);
            self.conn
                .execute("UPDATE sessions SET ended_at = ?2 WHERE id = ?1", params![id, end])?;
        }
        let Some((id, started)) = found else { return Ok(None) };
        let (elapsed, work): (i64, i64) = self.conn.query_row(
            "SELECT COALESCE(SUM(CASE WHEN kind != 'paused' THEN end_ms - start_ms END), 0),
                    COALESCE(SUM(CASE WHEN kind = 'work' THEN end_ms - start_ms END), 0)
             FROM segments WHERE session_id = ?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )?;
        let apps = {
            let mut st = self.conn.prepare(
                "SELECT s.app_id, s.exe, a.label, a.color, SUM(s.end_ms - s.start_ms)
                 FROM segments s LEFT JOIN apps a ON a.id = s.app_id
                 WHERE s.session_id = ?1 AND s.kind = 'work'
                 GROUP BY s.app_id, CASE WHEN s.app_id IS NULL THEN lower(s.exe) END",
            )?;
            let rows = st.query_map([id], |r| {
                let app_id: Option<i64> = r.get(0)?;
                let exe: Option<String> = r.get(1)?;
                let exe = exe.unwrap_or_default();
                let label: Option<String> = r.get(2)?;
                Ok(AppTime {
                    key: AppRef::key_of(app_id, &exe),
                    label: label.unwrap_or_else(|| display_name(&exe)),
                    color: r.get(3)?,
                    work_ms: r.get::<_, i64>(4)? as u64,
                })
            })?;
            rows.collect::<R<Vec<_>>>()?
        };
        self.session_id = Some(id);
        Ok(Some(Resume {
            started_ms: started,
            elapsed_ms: elapsed as u64,
            work_ms: work as u64,
            apps,
        }))
    }

    // ---- 구간 ----

    /// 엔진이 내놓은 구간을 기록한다. 직전 구간과 같은 상태로 이어지면 행을 늘리기만 한다.
    pub fn push(&mut self, iv: &Interval) -> R<()> {
        let Some(session_id) = self.session_id else { return Ok(()) };
        let app_id = iv.app.as_ref().and_then(|a| a.app_id);
        let exe = iv.app.as_ref().map(|a| a.exe.clone());
        if let Some(o) = &mut self.open {
            let same = o.kind == iv.kind
                && o.app_id == app_id
                && o.exe.as_deref().map(str::to_lowercase) == exe.as_deref().map(str::to_lowercase)
                && o.day == iv.day
                && o.end_ms == iv.start_ms;
            if same {
                o.end_ms = iv.end_ms;
                if o.end_ms - o.flushed_end_ms >= FLUSH_EVERY_MS {
                    self.flush()?;
                }
                return Ok(());
            }
        }
        self.close_segment()?;
        self.conn.execute(
            "INSERT INTO segments (session_id, start_ms, end_ms, kind, app_id, exe, day)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![session_id, iv.start_ms, iv.end_ms, iv.kind.as_str(), app_id, exe, iv.day],
        )?;
        self.open = Some(OpenSeg {
            id: self.conn.last_insert_rowid(),
            end_ms: iv.end_ms,
            flushed_end_ms: iv.end_ms,
            kind: iv.kind,
            app_id,
            exe,
            day: iv.day.clone(),
        });
        Ok(())
    }

    /// 진행 중인 행의 end_ms를 디스크에 쓴다.
    pub fn flush(&mut self) -> R<()> {
        if let Some(o) = &mut self.open {
            if o.end_ms != o.flushed_end_ms {
                self.conn
                    .execute("UPDATE segments SET end_ms = ?2 WHERE id = ?1", params![o.id, o.end_ms])?;
                o.flushed_end_ms = o.end_ms;
            }
        }
        Ok(())
    }

    fn close_segment(&mut self) -> R<()> {
        self.flush()?;
        self.open = None;
        Ok(())
    }

    #[cfg(test)]
    fn segments(&self) -> Vec<(i64, i64, String, Option<i64>, Option<String>)> {
        let mut st = self
            .conn
            .prepare("SELECT start_ms, end_ms, kind, app_id, exe FROM segments ORDER BY id")
            .unwrap();
        let rows = st
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)))
            .unwrap();
        rows.map(Result::unwrap).collect()
    }

    #[cfg(test)]
    fn has_open(&self) -> bool {
        self.open.is_some()
    }
}

fn norm_basis(b: &str) -> &'static str {
    if b == "elapsed" {
        "elapsed"
    } else {
        "work"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: &str = "2026-11-01";

    fn iv(start: i64, end: i64, kind: Kind, app: Option<(i64, &str)>) -> Interval {
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
            day: DAY.into(),
        }
    }

    #[test]
    fn merges_contiguous_intervals_and_splits_on_change() {
        let mut st = Store::memory().unwrap();
        let a = st.add_app("A.exe", None, None).unwrap();
        st.start_session(0).unwrap();
        for i in 0..5 {
            st.push(&iv(i * 1000, (i + 1) * 1000, Kind::Work, Some((a.id, "A.exe")))).unwrap();
        }
        st.push(&iv(5000, 6000, Kind::Other, None)).unwrap();
        st.push(&iv(6000, 7000, Kind::Other, None)).unwrap();
        st.end_session(7000, Some("러프")).unwrap();
        assert_eq!(
            st.segments(),
            vec![
                (0, 5000, "work".into(), Some(a.id), Some("A.exe".into())),
                (5000, 7000, "other".into(), None, None),
            ]
        );
        let note: String = st.conn.query_row("SELECT note FROM sessions", [], |r| r.get(0)).unwrap();
        assert_eq!(note, "러프");
    }

    #[test]
    fn flushes_open_segment_every_minute() {
        let mut st = Store::memory().unwrap();
        st.start_session(0).unwrap();
        for i in 0..61 {
            st.push(&iv(i * 1000, (i + 1) * 1000, Kind::Idle, None)).unwrap();
        }
        // 닫지 않았는데도 1분치가 디스크에 있다
        assert_eq!(st.segments()[0].1, 61_000);
        assert!(st.has_open());
    }

    #[test]
    fn resumes_same_day_session_and_closes_old_ones() {
        let mut st = Store::memory().unwrap();
        let a = st.add_app("A.exe", Some("에이"), None).unwrap();
        // 어제 끝나지 않은 세션
        st.start_session(0).unwrap();
        st.push(&Interval { day: "2026-10-31".into(), ..iv(0, 1000, Kind::Work, None) }).unwrap();
        st.flush().unwrap();
        // 오늘 세션
        st.start_session(10_000).unwrap();
        st.push(&iv(10_000, 13_000, Kind::Work, Some((a.id, "A.exe")))).unwrap();
        st.push(&iv(13_000, 14_000, Kind::Paused, None)).unwrap();
        st.push(&iv(14_000, 16_000, Kind::Other, None)).unwrap();
        st.flush().unwrap();

        let mut st2 = Store { conn: st.conn, session_id: None, open: None };
        let r = st2.take_resumable(DAY, |_| DAY.into()).unwrap().unwrap();
        assert_eq!(r.started_ms, 10_000);
        assert_eq!(r.work_ms, 3_000);
        assert_eq!(r.elapsed_ms, 5_000);
        assert_eq!(r.apps.len(), 1);
        assert_eq!(r.apps[0].label, "에이");
        assert_eq!(r.apps[0].key, format!("id:{}", a.id));
        let ended: Option<i64> = st2
            .conn
            .query_row("SELECT ended_at FROM sessions WHERE started_at = 0", [], |r| r.get(0))
            .unwrap();
        assert_eq!(ended, Some(1000));
    }

    #[test]
    fn alarm_crud_sorted_by_time() {
        let st = Store::memory().unwrap();
        let a = st.add_alarm("23:30", 127, Some(" 마감 확인 ")).unwrap();
        let b = st.add_alarm("07:40", 31, None).unwrap();
        assert_eq!(a.label.as_deref(), Some("마감 확인"));
        let ids: Vec<_> = st.list_alarms().unwrap().iter().map(|x| x.id).collect();
        assert_eq!(ids, vec![b.id, a.id]);
        st.update_alarm(&AlarmRow { enabled: false, sound: Some("builtin:chime".into()), ..b.clone() }).unwrap();
        let b2 = st.list_alarms().unwrap().into_iter().find(|x| x.id == b.id).unwrap();
        assert_eq!((b2.enabled, b2.sound.as_deref()), (false, Some("builtin:chime")));
        st.delete_alarm(a.id).unwrap();
        assert_eq!(st.list_alarms().unwrap().len(), 1);
    }

    #[test]
    fn migrates_step3_alerts_table() {
        // 휴식 열이 없던 alerts 테이블
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE alerts (id INTEGER PRIMARY KEY, every_min INTEGER NOT NULL, basis TEXT NOT NULL,
               sound_path TEXT, volume REAL DEFAULT 0.8, enabled INTEGER DEFAULT 1);
             INSERT INTO alerts (every_min, basis) VALUES (50, 'work');",
        )
        .unwrap();
        let st = Store::init(conn).unwrap();
        let a = &st.list_alerts().unwrap()[0];
        assert_eq!((a.every_min, a.break_min, a.break_sound.clone()), (50, 0, None));
    }

    #[test]
    fn alert_crud_clamps_values() {
        let st = Store::memory().unwrap();
        let a = st.add_alert(50, "work").unwrap();
        assert_eq!((a.every_min, a.basis.as_str(), a.enabled, a.sound.clone()), (50, "work", true, None));
        st.update_alert(&AlertRow {
            every_min: 0,
            basis: "elapsed".into(),
            volume: 3.0,
            sound: Some("bell.wav".into()),
            break_min: 5,
            break_sound: Some("builtin:rise".into()),
            ..a.clone()
        })
        .unwrap();
        let b = &st.list_alerts().unwrap()[0];
        assert_eq!((b.every_min, b.basis.as_str(), b.volume), (1, "elapsed", 1.0));
        assert_eq!((b.break_min, b.break_sound.as_deref()), (5, Some("builtin:rise")));
        assert_eq!(b.sound.as_deref(), Some("bell.wav"));
        st.delete_alert(a.id).unwrap();
        assert!(st.list_alerts().unwrap().is_empty());
    }

    #[test]
    fn app_crud_and_delete_keeps_history_exe() {
        let mut st = Store::memory().unwrap();
        let a = st.add_app("CLIPStudioPaint.exe", None, None).unwrap();
        assert_eq!(a.label, "CLIP STUDIO PAINT");
        assert_eq!(a.color.as_deref(), Some("accent"));
        let b = st.add_app("chrome.exe", None, Some("Pinterest")).unwrap();
        assert_eq!(b.color.as_deref(), Some("ink"));
        // 같은 exe(대소문자 무시)는 새로 만들지 않음
        assert_eq!(st.add_app("clipstudiopaint.exe", None, None).unwrap().id, a.id);

        st.update_app(&AppRow { label: "클스튜".into(), enabled: false, ..a.clone() }).unwrap();
        let t = st.tracked().unwrap();
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].title_kw.as_deref(), Some("Pinterest"));

        st.start_session(0).unwrap();
        st.push(&iv(0, 1000, Kind::Work, Some((a.id, "CLIPStudioPaint.exe")))).unwrap();
        st.end_session(1000, None).unwrap();
        st.delete_app(a.id).unwrap();
        assert_eq!(st.segments()[0].3, None);
        assert_eq!(st.segments()[0].4.as_deref(), Some("CLIPStudioPaint.exe"));
    }
}
