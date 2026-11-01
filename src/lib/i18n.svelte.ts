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
  "status.ready": "시작 전",
  "today.start": "시작",
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
  "sound.fileFilter": "소리·영상",

  // 알림음 구간 편집기
  "clip.title": "알림음 만들기",
  "clip.loading": "파일 읽는 중…",
  "clip.errRead": "이 파일에서 소리를 읽을 수 없어요",
  "clip.errLong": "{min}분 이하 파일만 편집할 수 있어요",
  "clip.close": "닫기",
  "clip.hint": "끌어서 구간 선택 · 휠로 확대 · Shift+휠로 이동",
  "clip.zoomIn": "확대",
  "clip.zoomOut": "축소",
  "clip.fitSel": "구간 보기",
  "clip.fitAll": "전체 보기",
  "clip.start": "시작",
  "clip.end": "끝",
  "clip.length": "{sec}초 (최대 {max}초)",
  "clip.play": "▶ 들어보기",
  "clip.stop": "■ 멈춤",
  "clip.fade": "앞뒤 부드럽게",
  "clip.cancel": "취소",
  "clip.save": "저장",
  "clip.saving": "저장 중…",

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
  "log.delete": "삭제",
  "log.deleted": "삭제했어요",
  "log.current": "진행 중",
  "log.deleteSessionAsk": "이 세션 기록을 삭제할까요? 되돌릴 수 없어요.",
  "log.deleteCurrentAsk": "진행 중인 세션이에요. 삭제하면 지금 세션도 0으로 돌아가고 시작 전 상태가 돼요. 되돌릴 수 없어요.",
  "log.deleteDay": "이 날 기록 삭제",
  "log.deleteDayAsk": "{date} 기록을 모두 삭제할까요? 되돌릴 수 없어요.",

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

  // 사용법
  "guide.title": "사용법",
  "guide.open": "사용법",
  "guide.start.t": "시작하기",
  "guide.start.b": "오늘 탭의 \"시작\"을 눌러야 시간이 흐릅니다. 앱을 켰을 때와 초기화·세션 종료 뒤에는 멈춘 채 기다려요.\n일시정지한 뒤에는 \"재개\"로 이어서 셉니다.\n초기화는 지금 세션을 0으로 되돌려요. 그때까지의 기록은 통계에 남습니다.\n세션 종료는 '무슨 작업?' 메모를 남기고 새 세션으로 넘어갑니다.",
  "guide.numbers.t": "숫자 읽는 법",
  "guide.numbers.b": "실 작업: 등록한 프로그램이 앞에 있고, 마우스·키보드 입력이 자리 비움 기준(기본 3분) 안에 있었던 시간\n경과: 시작한 뒤 흐른 시간. 일시정지 중에만 멈추고 휴식 중에도 흐릅니다\n집중: 실 작업 ÷ 경과",
  "guide.programs.t": "추적 프로그램",
  "guide.programs.b": "설정 › 추적 프로그램에서 실행 중인 프로그램을 추가합니다. 하나도 없으면 모든 프로그램을 작업으로 셉니다.\n이름을 누르면 이름·창 제목 키워드·막대 색을 바꿀 수 있어요. 창 제목 키워드를 넣으면 제목에 그 글자가 있을 때만 셉니다(예: 크롬 + Pinterest).",
  "guide.mini.t": "작은 타이머",
  "guide.mini.b": "오늘 화면 버튼 줄 오른쪽 아이콘이나 헤더의 접기 아이콘으로 띄웁니다. 항상 위에 뜨고 작업 표시줄에도 따로 보여요.\n마우스를 올리면 아이콘이 나옵니다: 고정(항상 위) · 표시 바꾸기(실 작업/경과/둘 다) · 칩으로 접기 · 닫기\n끌어서 옮길 수 있고 위치를 기억합니다. 두 번 클릭하면 본창으로 돌아갑니다.",
  "guide.chip.t": "칩 (접은 작은 타이머)",
  "guide.chip.b": "초록 = 작업 중, 빨강 = 멈춤·대기·시작 전. 한 번 클릭하면 시작/일시정지가 바뀝니다.\n다시 펼치기: 마우스를 올리면 오른쪽에 나오는 펼치기 단추, 우클릭, 두 번 클릭, 트레이 메뉴 '작은 타이머'\n다른 창을 쓰다가 바로 두 번 클릭하면 첫 클릭이 창 활성화에 쓰여 안 펼쳐질 수 있어요. 펼치기 단추나 우클릭이 확실합니다.",
  "guide.alerts.t": "주기 알림과 휴식",
  "guide.alerts.b": "설정 › 주기 알림: N분마다(경과 또는 실 작업 기준) 소리와 Windows 알림. 오늘 화면 아래 \"50분마다 알림 켜기\"로 바로 켤 수도 있어요.\n휴식을 정하면 알림 뒤 그 시간 동안 실 작업을 세지 않고(경과는 흐름), 끝나면 휴식 끝 소리와 함께 다시 셉니다.\n다음 알림은 휴식이 끝난 때부터 다시 N분 뒤입니다. 휴식 중 \"휴식 끝내기\"로 일찍 끝낼 수 있어요.\n소리 목록의 \"파일에서 고르기…\"로 mp3나 영상 파일을 열면 파형에서 원하는 구간(최대 60초)만 잘라 알림음으로 쓸 수 있어요.",
  "guide.shortcuts.t": "단축키",
  "guide.shortcuts.b": "{pause}: 시작 · 일시정지 · 재개\n{reset}: 초기화(확인 한 번)\n설정 › 단축키에서 바꿀 수 있어요. 다른 프로그램이 쓰고 있는 조합이면 알려 줍니다.",
  "guide.tray.t": "트레이",
  "guide.tray.b": "창을 닫아도(×) 트레이에서 계속 기록하고 알림·알람도 울립니다.\n트레이 아이콘 왼쪽 클릭 = 창 열기, 오른쪽 클릭 = 메뉴(열기 · 작은 타이머 · 시작/일시정지 · 초기화 · 종료). 완전히 끄려면 메뉴의 '종료'.",
  "guide.tools.t": "도구",
  "guide.tools.b": "타이머(프리셋·직접 입력, 예: 25 또는 1:30), 스톱워치(랩), 알람(요일 반복, 요일을 안 고르면 한 번만).\n작업 시간과 따로 돌고, 창을 닫아도 울립니다.",
  "guide.log.t": "기록",
  "guide.log.b": "일/주/월/연을 바꾸고 ◀ ▶로 과거로 이동합니다. 가운데 날짜를 누르면 날짜를 직접 고를 수 있어요.\n막대나 날짜를 누르면 한 단계 아래(연 → 월 → 일)로 내려갑니다.\n하루 보기에서 세션 메모를 눌러 고칠 수 있고, 아래 \"CSV 내보내기\"로 보고 있는 기간을 저장합니다.\n하루 보기의 휴지통 단추나 \"이 날 기록 삭제\"로 잘못 남은 기록을 지울 수 있어요(되돌릴 수 없음).",

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
