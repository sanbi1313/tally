// 화면 문구. 한국어가 기준이고 다른 언어는 locales/*.ts.
// 트레이·Windows 알림 문구는 src-tauri/src/i18n.rs.
import { en } from "./locales/en";
import { ja } from "./locales/ja";
import { zh } from "./locales/zh";

const ko = {
  // 헤더·탭
  "theme.toggle": "테마 전환",
  "window.close": "트레이로 닫기",
  "tab.today": "오늘",
  "tab.tools": "도구",
  "tab.log": "기록",
  "tab.settings": "설정",

  // 오늘
  "status.work": "{app} 작업 중",
  "status.other": "대기",
  "status.idle": "자리 비움",
  "status.paused": "일시정지됨",
  "status.break": "휴식 중",
  "today.work": "실 작업",
  "today.elapsed": "경과",
  "today.focus": "집중",
  "today.pause": "일시정지",
  "today.resume": "재개",
  "today.reset": "초기화",
  "today.endSession": "세션 종료",
  "today.resetConfirm": "현재 세션을 0으로 되돌릴까요?",
  "today.cancel": "취소",
  "today.memoPlaceholder": "무슨 작업?",
  "today.end": "종료",
  "today.prevKept": "이전 기록 {time}은 오늘 통계에 남았어요",
  "today.byProgram": "프로그램별",
  "today.breakLeft": "휴식 {time} 남음",
  "today.endBreak": "휴식 끝내기",
  "today.nextAlert": "다음 알림 {min} 후",
  "today.alertBasis": "{basis} 기준 · {every}마다",
  "today.alertBreak": " · 휴식 {min}",
  "today.noAlert": "알림 없음",
  "today.enableAlert": "50분마다 알림 켜기",

  // 첫 실행
  "onboard.language": "언어를 고르세요",
  "onboard.next": "다음",
  "onboard.title": "무엇으로 작업하세요?",
  "onboard.startN": "{n}개로 시작",
  "onboard.start": "시작",
  "onboard.later": "나중에",
  "onboard.note": "고르지 않으면 모든 프로그램을 작업으로 셉니다",

  // 프로그램 목록
  "picker.loading": "불러오는 중",
  "picker.empty": "추가할 프로그램이 없어요",
  "picker.manual": "exe 직접 입력 (예: sai2.exe)",
  "picker.add": "추가",
  "picker.refresh": "새로 고침",

  // 설정: 추적 프로그램
  "settings.tracked": "추적 프로그램",
  "settings.trackToggle": "{label} 추적",
  "settings.name": "이름",
  "settings.titleKw": "창 제목",
  "settings.titleKwPlaceholder": "이 글자가 제목에 있을 때만 (예: Pinterest)",
  "settings.color": "색",
  "settings.delete": "삭제",
  "settings.noTracked": "등록한 프로그램이 없어 모든 프로그램을 작업으로 셉니다",
  "settings.running": "실행 중인 프로그램",
  "settings.close": "닫기",
  "settings.addRunning": "+ 실행 중인 프로그램에서 추가",

  // 설정: 주기 알림
  "alert.title": "주기 알림",
  "alert.toggle": "{every} 알림 켜기",
  "alert.every": "알림 간격(분)",
  "alert.everySuffix": "분마다",
  "alert.basisElapsed": "경과",
  "alert.basisWork": "실 작업",
  "alert.sound": "알림 소리",
  "alert.volume": "볼륨",
  "alert.preview": "미리듣기",
  "alert.break": "휴식",
  "alert.breakMin": "휴식 시간(분)",
  "alert.breakSuffix": "분 쉬기",
  "alert.breakNone": "0이면 휴식 없이 알림만",
  "alert.breakSound": "휴식 끝 소리",
  "alert.delete": "삭제",
  "alert.add": "+ 알림 추가",
  "sound.bell": "종소리",
  "sound.chime": "차임",
  "sound.marimba": "마림바",
  "sound.soft": "부드러운 삐",
  "sound.bowl": "싱잉볼",
  "sound.rise": "상승음",
  "sound.file": "파일에서 고르기…",
  "sound.fileFilter": "소리",

  // 설정: 일반
  "general.idle": "자리 비움",
  "general.idleMin": "자리 비움 기준(분)",
  "general.idleSuffix": "분 동안 입력이 없으면 실 작업에서 뺌",
  "general.shortcuts": "단축키",
  "general.scPause": "일시정지·재개",
  "general.scReset": "초기화",
  "general.scPress": "키를 누르세요",
  "general.scNone": "없음",
  "general.scTaken": "다른 프로그램이 이미 쓰고 있어요. 다른 조합을 골라 주세요",
  "general.scHint": "Esc 취소 · Backspace 끄기",
  "general.language": "언어",

  // 도구
  "tool.timer": "타이머",
  "tool.stopwatch": "스톱워치",
  "tool.alarm": "알람",
  "timer.custom": "직접 입력",
  "timer.customPlaceholder": "분 또는 분:초",
  "timer.set": "설정",
  "timer.start": "시작",
  "timer.pause": "일시정지",
  "timer.resume": "계속",
  "timer.reset": "초기화",
  "timer.whenDone": "끝나면",
  "timer.editPresets": "프리셋 편집",
  "timer.presetMin": "프리셋 {n}(분)",
  "timer.done": "완료",
  "sw.start": "시작",
  "sw.stop": "정지",
  "sw.lap": "랩",
  "sw.reset": "초기화",
  "sw.lapN": "랩 {n}",
  "alarm.add": "+ 알람 추가",
  "alarm.toggle": "{time} 알람 켜기",
  "alarm.time": "시각",
  "alarm.repeat": "반복",
  "alarm.label": "메모",
  "alarm.labelPlaceholder": "예: 마감 확인",
  "alarm.sound": "소리",
  "alarm.delete": "삭제",
  "alarm.done": "완료",
  "alarm.daily": "매일",
  "alarm.weekdays": "평일",
  "alarm.weekend": "주말",
  "alarm.once": "한 번",
  "alarm.empty": "알람이 없어요",
  "day.0": "월",
  "day.1": "화",
  "day.2": "수",
  "day.3": "목",
  "day.4": "금",
  "day.5": "토",
  "day.6": "일",

  // 기록
  "log.day": "일",
  "log.week": "주",
  "log.month": "월",
  "log.year": "연",
  "log.prev": "이전",
  "log.next": "다음",
  "log.pickDate": "날짜 선택",
  "log.daySub": "경과 {elapsed} · 집중 {focus}",
  "log.weekSub": "하루 평균 {avg}",
  "log.monthSub": "작업한 날 {n}일",
  "log.yearSub": "월 평균 {avg}",
  "log.sessions": "세션",
  "log.noSessions": "이날은 기록이 없어요",
  "log.noMemo": "메모 없음",
  "log.editMemo": "메모 고치기",
  "log.tapBar": "막대를 누르면 그날 기록",
  "log.heatHint": "진할수록 오래 · 날짜를 누르면 그날 기록",
  "log.bestMonth": "가장 많이 한 달 {month} · {time}",
  "log.tapMonth": "막대를 누르면 그 달",
  "log.export": "CSV 내보내기",
  "log.exported": "{n}줄을 저장했어요",

  // 작은 타이머·테마·시작
  "mini.fold": "작은 타이머로 접기",
  "mini.open": "작은 타이머 띄우기",
  "mini.work": "작업",
  "mini.pin": "항상 위에 고정",
  "mini.cycle": "표시할 시간 바꾸기",
  "mini.collapse": "칩으로 접기",
  "mini.close": "닫기",
  "mini.chipWorking": "작업 중, 눌러서 멈춤",
  "mini.chipStopped": "멈춤, 눌러서 재개",
  "mini.chipHint": "한 번: 멈춤/재개 · 오른쪽 단추·두 번·우클릭: 펼치기",
  "mini.expand": "펼치기",
  "general.mini": "작은 타이머",
  "general.miniWork": "실 작업",
  "general.miniElapsed": "경과",
  "general.miniBoth": "둘 다",
  "general.onTop": "항상 위에 표시",
  "general.opacity": "투명도",
  "general.size": "크기",
  "general.sizeSmall": "작게",
  "general.sizeNormal": "보통",
  "general.theme": "테마",
  "general.themeLight": "라이트",
  "general.themeDark": "다크",
  "general.themeSystem": "시스템",
  "general.startup": "시작 프로그램",
  "general.autostart": "Windows 시작 시 트레이로 실행",

  // 시간 단위
  "unit.min": "{n}분",
  "unit.hour": "{n}시간",
  "unit.hourMin": "{h}시간 {m}분",
};

export type Key = keyof typeof ko;
export type Lang = "ko" | "en" | "ja" | "zh";
/** 모든 키가 있어야 한다(빠진 번역은 타입 검사에서 걸림) */
export type Dict = Record<Key, string>;

const dicts: Record<Lang, Dict> = { ko, en, ja, zh };

export const LANG_NAMES: Record<Lang, string> = { ko: "한국어", en: "English", ja: "日本語", zh: "中文(简体)" };

export const availableLangs = Object.keys(dicts) as Lang[];

/** Windows 표시 언어로 첫 실행 기본값을 고른다. 한·영·일·중이 아니면 영어 */
export function detectLang(): Lang {
  const l = (navigator.language || "en").toLowerCase();
  if (l.startsWith("ko")) return "ko";
  if (l.startsWith("ja")) return "ja";
  if (l.startsWith("zh")) return "zh";
  return "en";
}

const state = $state<{ lang: Lang }>({ lang: "ko" });

export function setLang(l: string) {
  state.lang = (availableLangs as string[]).includes(l) ? (l as Lang) : "ko";
  document.documentElement.lang = state.lang === "zh" ? "zh-CN" : state.lang;
}

export function getLang(): Lang {
  return state.lang;
}

export function t(key: Key, params: Record<string, string | number> = {}): string {
  let s = dicts[state.lang][key];
  for (const [k, v] of Object.entries(params)) s = s.replaceAll(`{${k}}`, String(v));
  return s;
}

/** 분 → "1시간 40분" */
export function minutes(min: number): string {
  const h = Math.floor(min / 60);
  const m = min % 60;
  if (h === 0) return t("unit.min", { n: m });
  if (m === 0) return t("unit.hour", { n: h });
  return t("unit.hourMin", { h, m });
}
