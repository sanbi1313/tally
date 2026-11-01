// 기록 탭 날짜 계산. 날짜는 "YYYY-MM-DD" 문자열로 다루고(하루 경계는 Rust가 이미 적용),
// 계산할 때만 현지 정오 Date로 바꿔 서머타임 같은 경계 문제를 피한다.
import type { Lang } from "./i18n.svelte";

export type Day = string;

export function toDate(d: Day): Date {
  const [y, m, dd] = d.split("-").map(Number);
  return new Date(y, m - 1, dd, 12);
}

export function fromDate(d: Date): Day {
  const p = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`;
}

export function addDays(d: Day, n: number): Day {
  const x = toDate(d);
  x.setDate(x.getDate() + n);
  return fromDate(x);
}

/** 월요일 시작 */
export function weekStart(d: Day): Day {
  const x = toDate(d);
  return addDays(d, -((x.getDay() + 6) % 7));
}

export function monthStart(d: Day): Day {
  return d.slice(0, 8) + "01";
}

export function monthEnd(d: Day): Day {
  const x = toDate(monthStart(d));
  return fromDate(new Date(x.getFullYear(), x.getMonth() + 1, 0, 12));
}

export function addMonths(d: Day, n: number): Day {
  const x = toDate(monthStart(d));
  return fromDate(new Date(x.getFullYear(), x.getMonth() + n, 1, 12));
}

export const yearStart = (d: Day): Day => d.slice(0, 4) + "-01-01";
export const yearEnd = (d: Day): Day => d.slice(0, 4) + "-12-31";

const LOCALES: Record<Lang, string> = { ko: "ko-KR", en: "en-US", ja: "ja-JP", zh: "zh-CN" };

export function fmt(lang: Lang, d: Day | Date, opts: Intl.DateTimeFormatOptions): string {
  return new Intl.DateTimeFormat(LOCALES[lang], opts).format(typeof d === "string" ? toDate(d) : d);
}

export function fmtRange(lang: Lang, a: Day, b: Day, opts: Intl.DateTimeFormatOptions): string {
  return new Intl.DateTimeFormat(LOCALES[lang], opts).formatRange(toDate(a), toDate(b));
}

/** unix ms → "13:05" */
export function hm(ms: number): string {
  const d = new Date(ms);
  return `${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
}

/** 그날(하루 시작 시각 기준)이 시작하는 unix ms */
export function dayStartMs(d: Day, startHour: number): number {
  const [y, m, dd] = d.split("-").map(Number);
  return new Date(y, m - 1, dd, startHour).getTime();
}
