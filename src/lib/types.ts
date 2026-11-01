export type Kind = "work" | "other" | "idle" | "break" | "paused";

/** 막대 색. 테마 토큰 이름이라 라이트/다크에 맞춰 바뀐다. */
export type ColorToken = "accent" | "ink" | "sub" | "idle";
export const COLOR_TOKENS: ColorToken[] = ["accent", "ink", "sub", "idle"];

export function colorVar(c: string | null | undefined): string {
  return COLOR_TOKENS.includes(c as ColorToken) ? `var(--${c})` : "var(--sub)";
}

/** Rust engine::Snapshot */
export interface Snapshot {
  kind: Kind;
  app: string | null;
  workMs: number;
  elapsedMs: number;
  sessionStartedMs: number;
  day: string;
  apps: { key: string; label: string; color: string | null; workMs: number }[];
  prevWorkMs: number | null;
  /** 시작을 눌렀는지(앱을 켠 뒤·초기화 뒤엔 false) */
  started: boolean;
}

/** Rust store::AppRow */
export interface AppRow {
  id: number;
  exe: string;
  label: string;
  color: string | null;
  titleKw: string | null;
  enabled: boolean;
}

export interface RunningProgram {
  exe: string;
  label: string;
  title: string;
  icon: string | null;
}

/** Rust alerts::NextAlert */
export interface NextAlert {
  remainingMs: number;
  everyMin: number;
  basis: "elapsed" | "work";
  breakMin: number;
}

/** UI가 받는 상태 = 엔진 스냅숏 + 다음 알림(켜진 알림이 없으면 null) */
export interface TimerView {
  durationMs: number;
  remainingMs: number;
  /** 돌고 있으면 끝나는 시각(unix ms) */
  endsAtMs: number | null;
}

export interface StopwatchView {
  accMs: number;
  /** 돌고 있으면 다시 시작한 시각(unix ms) */
  sinceMs: number | null;
  /** 랩마다 걸린 시간, 첫 랩부터 */
  laps: number[];
}

export type View = Snapshot & {
  nextAlert: NextAlert | null;
  breakRemainingMs: number | null;
  timer: TimerView;
  stopwatch: StopwatchView;
};

/** Rust tools::AlarmRow. weekdays: 월=1 … 일=64, 0 = 한 번 */
export interface AlarmRow {
  id: number;
  timeHm: string;
  weekdays: number;
  label: string | null;
  sound: string | null;
  enabled: boolean;
}

export interface TimerSettings {
  presets: number[];
  sound: string | null;
  volume: number;
}

/** Rust alerts::AlertRow */
export interface AlertRow {
  id: number;
  everyMin: number;
  basis: "elapsed" | "work";
  sound: string | null;
  volume: number;
  enabled: boolean;
  /** 알림 뒤 자동 휴식(분). 0이면 없음 */
  breakMin: number;
  breakSound: string | null;
}

export interface SettingsOut {
  idleMinutes: number;
  shortcuts: { pause: string; reset: string };
  shortcutErrors: string[];
  language: string;
  dayStartHour: number;
  theme: string;
  mini: MiniSettings;
  autostart: boolean;
}

/** Rust settings::MiniSettings */
export interface MiniSettings {
  show: "work" | "elapsed" | "both";
  pinned: boolean;
  opacity: number;
  size: "small" | "normal";
  collapsed: boolean;
  x: number | null;
  y: number | null;
}

/** Rust history::DayTotal */
export interface DayTotal {
  day: string;
  workMs: number;
  elapsedMs: number;
}

/** Rust history::Seg */
export interface Seg {
  startMs: number;
  endMs: number;
  kind: Kind;
  label: string | null;
  color: string | null;
}

/** Rust history::SessionSummary */
export interface SessionSummary {
  id: number;
  note: string | null;
  startMs: number;
  endMs: number;
  workMs: number;
  apps: string[];
  /** 지금 기록 중인 세션 */
  current: boolean;
}

export interface DayDetail {
  segments: Seg[];
  sessions: SessionSummary[];
}
