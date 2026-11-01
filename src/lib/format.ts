/** 밀리초 → H:MM:SS */
export function hms(ms: number): string {
  const s = Math.floor(ms / 1000);
  const h = Math.floor(s / 3600);
  const m = Math.floor(s / 60) % 60;
  const sec = s % 60;
  return `${h}:${String(m).padStart(2, "0")}:${String(sec).padStart(2, "0")}`;
}

/** 실 작업 / 경과. 1분이 안 되면 의미가 없어 대시로 둔다. */
export function focus(workMs: number, elapsedMs: number): string {
  if (elapsedMs < 60_000) return "—";
  return `${Math.round((workMs / elapsedMs) * 100)}%`;
}

/** 남은 시간 → M:SS (초는 올림. 0:56, 4:05) */
export function mmss(ms: number): string {
  const s = Math.ceil(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}

/** 초 → "25:00" 또는 1시간 이상이면 "1:05:00" */
function clockOf(totalSec: number): string {
  const h = Math.floor(totalSec / 3600);
  const m = Math.floor(totalSec / 60) % 60;
  const sec = totalSec % 60;
  const mm = String(m).padStart(2, "0");
  const ss = String(sec).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

/** 타이머 남은 시간(초 올림): 25:00, 1:05:00 */
export function countdown(ms: number): string {
  return clockOf(Math.ceil(ms / 1000));
}

/** 스톱워치: ["12:04", ".3"] (0.1초 단위 버림) */
export function stopwatchParts(ms: number): [string, string] {
  const tenths = Math.floor(ms / 100);
  return [clockOf(Math.floor(tenths / 10)), `.${tenths % 10}`];
}
