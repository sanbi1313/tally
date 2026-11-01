# Tally 번역표 (영어 · 일본어 · 중국어 간체)

**적용됨 (2026-11-01).** 결정 사항: 실 작업의 영어는 "Active", 기본 프로그램 이름은 모든 언어에서 공식 제품 이름, 첫 실행 때 언어 선택 화면.
실제 문구는 `src/lib/i18n.svelte.ts`(한국어), `src/lib/locales/*.ts`(영·일·중), `src-tauri/src/i18n.rs`(트레이·알림·오류)에 있습니다. 고칠 곳은 "키 이름: 새 문구"로 알려 주세요.
`{app}`, `{n}`, `{time}`, `{min}`, `{every}`, `{basis}`, `{h}`, `{m}`, `{label}`은 자리 표시자라 그대로 둬야 합니다.

## 먼저 정할 용어

| 한국어 | English | 日本語 | 中文(简体) | 메모 |
| --- | --- | --- | --- | --- |
| 실 작업 | Active | 実作業 | 实际工作 | 확정 |
| 경과 | Elapsed | 経過 | 经过 | 중국어 "总时长"도 후보 |
| 집중 | Focus | 集中 | 专注 | 실 작업 ÷ 경과 비율 |
| 자리 비움 | Away | 離席 | 离开 | |
| 대기 | Waiting | 待機 | 等待 | 다른 프로그램 사용 중 |
| 세션 | Session | セッション | 会话 | 중국어 "时段"도 후보 |
| 주기 알림 | Interval alerts | 定期通知 | 定时提醒 | |
| 휴식 | Break | 休憩 | 休息 | |

## 화면 문구 (src/lib/i18n.svelte.ts)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| theme.toggle | 테마 전환 | Toggle theme | テーマ切替 | 切换主题 |
| window.close | 트레이로 닫기 | Close to tray | トレイに閉じる | 关闭到托盘 |
| tab.today | 오늘 | Today | 今日 | 今天 |
| tab.tools | 도구 | Tools | ツール | 工具 |
| tab.log | 기록 | History | 記録 | 记录 |
| tab.settings | 설정 | Settings | 設定 | 设置 |
| status.work | {app} 작업 중 | Working in {app} | {app} で作業中 | 正在用 {app} 工作 |
| status.other | 대기 | Waiting | 待機 | 等待 |
| status.idle | 자리 비움 | Away | 離席 | 离开 |
| status.paused | 일시정지됨 | Paused | 一時停止中 | 已暂停 |
| status.break | 휴식 중 | On break | 休憩中 | 休息中 |
| today.work | 실 작업 | Active | 実作業 | 实际工作 |
| today.elapsed | 경과 | Elapsed | 経過 | 经过 |
| today.focus | 집중 | Focus | 集中 | 专注 |
| today.pause | 일시정지 | Pause | 一時停止 | 暂停 |
| today.resume | 재개 | Resume | 再開 | 继续 |
| today.reset | 초기화 | Reset | リセット | 重置 |
| today.endSession | 세션 종료 | End session | セッション終了 | 结束会话 |
| today.resetConfirm | 현재 세션을 0으로 되돌릴까요? | Reset this session to 0? | 現在のセッションを0に戻しますか？ | 将当前会话归零？ |
| today.cancel | 취소 | Cancel | キャンセル | 取消 |
| today.memoPlaceholder | 무슨 작업? | What did you work on? | 何の作業？ | 做了什么？ |
| today.end | 종료 | End | 終了 | 结束 |
| today.prevKept | 이전 기록 {time}은 오늘 통계에 남았어요 | Previous {time} is kept in today's stats | 前回の {time} は今日の統計に残っています | 之前的 {time} 已计入今日统计 |
| today.byProgram | 프로그램별 | By program | プログラム別 | 按程序 |
| today.breakLeft | 휴식 {time} 남음 | Break · {time} left | 休憩 残り {time} | 休息剩余 {time} |
| today.endBreak | 휴식 끝내기 | End break | 休憩を終える | 结束休息 |
| today.nextAlert | 다음 알림 {min} 후 | Next alert in {min} | 次の通知まで {min} | {min}后提醒 |
| today.alertBasis | {basis} 기준 · {every}마다 | {basis} · every {every} | {basis}基準 · {every}ごと | 按{basis} · 每{every} |
| today.alertBreak | · 휴식 {min} | · {min} break | · 休憩 {min} | · 休息 {min} |
| today.noAlert | 알림 없음 | No alerts | 通知なし | 无提醒 |
| today.enableAlert | 50분마다 알림 켜기 | Alert every 50 min | 50分ごとに通知 | 每50分钟提醒 |
| onboard.title | 무엇으로 작업하세요? | What do you work in? | 何で作業しますか？ | 你用什么工作？ |
| onboard.startN | {n}개로 시작 | Start with {n} | {n}個で開始 | 开始（{n}个） |
| onboard.start | 시작 | Start | 開始 | 开始 |
| onboard.later | 나중에 | Later | あとで | 稍后 |
| onboard.note | 고르지 않으면 모든 프로그램을 작업으로 셉니다 | If you pick none, every program counts as active time | 選ばない場合、すべてのプログラムを作業として数えます | 不选择则所有程序都计为工作 |
| picker.loading | 불러오는 중 | Loading | 読み込み中 | 加载中 |
| picker.empty | 추가할 프로그램이 없어요 | No programs to add | 追加できるプログラムがありません | 没有可添加的程序 |
| picker.manual | exe 직접 입력 (예: sai2.exe) | Enter exe name (e.g. sai2.exe) | exe を直接入力（例: sai2.exe） | 直接输入 exe（例：sai2.exe） |
| picker.add | 추가 | Add | 追加 | 添加 |
| picker.refresh | 새로 고침 | Refresh | 更新 | 刷新 |
| settings.tracked | 추적 프로그램 | Tracked programs | 追跡するプログラム | 跟踪的程序 |
| settings.trackToggle | {label} 추적 | Track {label} | {label} を追跡 | 跟踪 {label} |
| settings.name | 이름 | Name | 名前 | 名称 |
| settings.titleKw | 창 제목 | Window title | ウィンドウ名 | 窗口标题 |
| settings.titleKwPlaceholder | 이 글자가 제목에 있을 때만 (예: Pinterest) | Only when the title contains this (e.g. Pinterest) | タイトルにこの文字があるときだけ（例: Pinterest） | 仅当标题包含此文字时（例：Pinterest） |
| settings.color | 색 | Color | 色 | 颜色 |
| settings.delete | 삭제 | Delete | 削除 | 删除 |
| settings.noTracked | 등록한 프로그램이 없어 모든 프로그램을 작업으로 셉니다 | No programs added, so every program counts as active time | 登録したプログラムがないため、すべてのプログラムを作業として数えます | 未添加程序，所有程序都计为工作 |
| settings.running | 실행 중인 프로그램 | Running programs | 実行中のプログラム | 正在运行的程序 |
| settings.close | 닫기 | Close | 閉じる | 关闭 |
| settings.addRunning | + 실행 중인 프로그램에서 추가 | + Add from running programs | + 実行中のプログラムから追加 | + 从正在运行的程序添加 |
| alert.title | 주기 알림 | Interval alerts | 定期通知 | 定时提醒 |
| alert.toggle | {every} 알림 켜기 | Turn on {every} alert | {every}の通知をオン | 开启{every}提醒 |
| alert.every | 알림 간격(분) | Alert interval (min) | 通知間隔（分） | 提醒间隔（分钟） |
| alert.everySuffix | 분마다 | min interval | 分ごと | 分钟一次 |
| alert.basisElapsed | 경과 | Elapsed | 経過 | 经过 |
| alert.basisWork | 실 작업 | Active | 実作業 | 实际工作 |
| alert.sound | 알림 소리 | Alert sound | 通知音 | 提醒音 |
| alert.volume | 볼륨 | Volume | 音量 | 音量 |
| alert.preview | 미리듣기 | Preview | 試聴 | 试听 |
| alert.break | 휴식 | Break | 休憩 | 休息 |
| alert.breakMin | 휴식 시간(분) | Break length (min) | 休憩時間（分） | 休息时长（分钟） |
| alert.breakSuffix | 분 쉬기 | min break | 分休む | 分钟休息 |
| alert.breakNone | 0이면 휴식 없이 알림만 | 0 = alert only, no break | 0なら休憩なしで通知のみ | 0 表示只提醒不休息 |
| alert.breakSound | 휴식 끝 소리 | Break-end sound | 休憩終了の音 | 休息结束提示音 |
| alert.delete | 삭제 | Delete | 削除 | 删除 |
| alert.add | + 알림 추가 | + Add alert | + 通知を追加 | + 添加提醒 |
| sound.bell | 종소리 | Bell | ベル | 铃声 |
| sound.chime | 차임 | Chime | チャイム | 叮咚 |
| sound.marimba | 마림바 | Marimba | マリンバ | 马林巴 |
| sound.soft | 부드러운 삐 | Soft beep | やさしいビープ | 柔和提示音 |
| sound.bowl | 싱잉볼 | Singing bowl | シンギングボウル | 颂钵 |
| sound.rise | 상승음 | Rising tones | 上昇音 | 上升音 |
| sound.file | 파일에서 고르기… | Choose a file… | ファイルから選ぶ… | 从文件选择… |
| sound.fileFilter | 소리 | Sound | サウンド | 声音 |
| general.idle | 자리 비움 | Away | 離席 | 离开 |
| general.idleMin | 자리 비움 기준(분) | Away after (min) | 離席判定（分） | 离开判定（分钟） |
| general.idleSuffix | 분 동안 입력이 없으면 실 작업에서 뺌 | min without input → not counted as active | 分間入力がなければ実作業から除外 | 分钟无输入则不计入实际工作 |
| general.shortcuts | 단축키 | Shortcuts | ショートカット | 快捷键 |
| general.scPause | 일시정지·재개 | Pause / resume | 一時停止・再開 | 暂停 / 继续 |
| general.scReset | 초기화 | Reset | リセット | 重置 |
| general.scPress | 키를 누르세요 | Press keys | キーを押してください | 请按键 |
| general.scNone | 없음 | None | なし | 无 |
| general.scTaken | 다른 프로그램이 이미 쓰고 있어요. 다른 조합을 골라 주세요 | Another program already uses this. Pick a different combination | 他のプログラムが使用中です。別の組み合わせを選んでください | 已被其他程序占用，请换一个组合 |
| general.scHint | Esc 취소 · Backspace 끄기 | Esc cancel · Backspace clear | Esc キャンセル · Backspace 解除 | Esc 取消 · Backspace 清除 |
| general.language | 언어 | Language | 言語 | 语言 |
| unit.min | {n}분 | {n} min | {n}分 | {n}分钟 |
| unit.hour | {n}시간 | {n} h | {n}時間 | {n}小时 |
| unit.hourMin | {h}시간 {m}분 | {h} h {m} min | {h}時間{m}分 | {h}小时{m}分钟 |

## 트레이 · Windows 알림 (src-tauri/src/i18n.rs)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| TrayOpen | 열기 | Open | 開く | 打开 |
| TrayPause | 일시정지 | Pause | 一時停止 | 暂停 |
| TrayResume | 재개 | Resume | 再開 | 继续 |
| TrayReset | 초기화 | Reset | リセット | 重置 |
| TrayQuit | 종료 | Quit | 終了 | 退出 |
| AlertBody | {n}분마다 알림 · 잠깐 쉬어 가세요 | Every {n} min · time for a short rest | {n}分ごとの通知 · ひと息つきましょう | 每{n}分钟提醒 · 稍作休息吧 |
| AlertBodyBreak | {n}분 휴식을 시작해요 | Starting a {n}-min break | {n}分の休憩を始めます | 开始休息 {n} 分钟 |
| BreakEndTitle | 휴식 끝 | Break over | 休憩終了 | 休息结束 |
| BreakEndBody | 다음 알림은 {n}분 뒤 | Next alert in {n} min | 次の通知は{n}分後 | {n}分钟后再次提醒 |

알림 제목은 "기준 + 시간" 조합입니다. 예: `실 작업 1시간 40분` / `Active 1 h 40 min` / `実作業 1時間40分` / `实际工作 1小时40分钟`. 상태·툴팁 문구는 위 화면 문구(status.*, today.work, today.elapsed)와 같은 번역을 씁니다.

## 첫 실행 언어 선택 (새로 추가한 문구)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| onboard.language | 언어를 고르세요 | Choose a language | 言語を選んでください | 选择语言 |
| onboard.next | 다음 | Next | 次へ | 下一步 |

기본 선택은 Windows 표시 언어(한·영·일·중이 아니면 English).

## 오류 메시지

| 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- |
| exe 이름이 비어 있어요 | The exe name is empty | exe 名が空です | exe 名称为空 |
| wav, mp3, ogg 파일만 쓸 수 있어요 | Only wav, mp3 and ogg files are supported | wav・mp3・ogg ファイルのみ使えます | 仅支持 wav、mp3、ogg 文件 |
| 알림을 찾을 수 없어요 | Alert not found | 通知が見つかりません | 找不到该提醒 |
| 단축키 형식 오류 | Invalid shortcut | ショートカットの形式が正しくありません | 快捷键格式无效 |

## 기본 프로그램 이름 (새로 등록할 때 붙는 이름)

모든 언어(한국어 포함)에서 공식 제품 이름을 씁니다. "모든 프로그램 = 작업" 모드의 프로그램별 이름도 같습니다. 표에 없는 프로그램은 exe 이름에서 .exe를 뺀 이름. 이미 등록한 프로그램의 이름은 바뀌지 않습니다(설정에서 직접 변경).

| exe | 이름 |
| --- | --- |
| CLIPStudioPaint.exe | CLIP STUDIO PAINT |
| Photoshop.exe | Photoshop |
| Illustrator.exe | Illustrator |
| AfterFX.exe | After Effects |
| Adobe Premiere Pro.exe | Premiere Pro |
| chrome.exe | Chrome |
| msedge.exe | Edge |
| firefox.exe | Firefox |
| Discord.exe | Discord |
| krita.exe | Krita |
| sai2.exe / sai.exe | PaintTool SAI Ver.2 / PaintTool SAI |
| blender.exe | Blender |
| Figma.exe | Figma |
| Code.exe | Visual Studio Code |
| notepad.exe | Notepad |
| mspaint.exe | Paint |

## 적용 방식

- 첫 실행: 언어 선택 → "무엇으로 작업하세요?". 언어를 누르는 즉시 화면 문구가 바뀝니다.
- 이후: 설정 탭 맨 아래 "언어". 화면·트레이 메뉴·Windows 알림·오류 문구가 바로 바뀝니다.
- 글꼴: 한국어·영어는 Pretendard. 번들한 Pretendard에 일본어·중국어 글자가 없어서 일본어는 Yu Gothic UI, 중국어는 Microsoft YaHei UI(둘 다 Windows 기본 글꼴)를 쓰고, 숫자는 모든 언어에서 Pretendard로 맞춥니다.

## 4단계: 도구 탭 (2026-11-01 추가·적용)

확정한 용어 규칙(Active, 공식 제품명)에 맞춰 바로 적용했습니다. 고칠 곳은 "키 이름: 새 문구"로 알려 주세요.

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| tool.timer | 타이머 | Timer | タイマー | 计时器 |
| tool.stopwatch | 스톱워치 | Stopwatch | ストップウォッチ | 秒表 |
| tool.alarm | 알람 | Alarm | アラーム | 闹钟 |
| timer.custom | 직접 입력 | Custom | 直接入力 | 自定义 |
| timer.customPlaceholder | 분 또는 분:초 | min or min:sec | 分 または 分:秒 | 分钟 或 分:秒 |
| timer.set | 설정 | Set | 設定 | 设置 |
| timer.start | 시작 | Start | スタート | 开始 |
| timer.pause | 일시정지 | Pause | 一時停止 | 暂停 |
| timer.resume | 계속 | Resume | 再開 | 继续 |
| timer.reset | 초기화 | Reset | リセット | 重置 |
| timer.whenDone | 끝나면 | When done | 終了時 | 结束时 |
| timer.editPresets | 프리셋 편집 | Edit presets | プリセット編集 | 编辑预设 |
| timer.presetMin | 프리셋 {n}(분) | Preset {n} (min) | プリセット{n}（分） | 预设 {n}（分钟） |
| timer.done | 완료 | Done | 完了 | 完成 |
| sw.start | 시작 | Start | スタート | 开始 |
| sw.stop | 정지 | Stop | ストップ | 停止 |
| sw.lap | 랩 | Lap | ラップ | 计圈 |
| sw.reset | 초기화 | Reset | リセット | 重置 |
| sw.lapN | 랩 {n} | Lap {n} | ラップ {n} | 第{n}圈 |
| alarm.add | + 알람 추가 | + Add alarm | + アラームを追加 | + 添加闹钟 |
| alarm.toggle | {time} 알람 켜기 | Turn on {time} alarm | {time}のアラームをオン | 开启{time}闹钟 |
| alarm.time | 시각 | Time | 時刻 | 时间 |
| alarm.repeat | 반복 | Repeat | 繰り返し | 重复 |
| alarm.label | 메모 | Label | メモ | 备注 |
| alarm.labelPlaceholder | 예: 마감 확인 | e.g. Check deadline | 例: 締切確認 | 例：检查截止日期 |
| alarm.sound | 소리 | Sound | サウンド | 声音 |
| alarm.delete | 삭제 | Delete | 削除 | 删除 |
| alarm.done | 완료 | Done | 完了 | 完成 |
| alarm.daily | 매일 | Every day | 毎日 | 每天 |
| alarm.weekdays | 평일 | Weekdays | 平日 | 工作日 |
| alarm.weekend | 주말 | Weekends | 週末 | 周末 |
| alarm.once | 한 번 | Once | 1回のみ | 仅一次 |
| alarm.empty | 알람이 없어요 | No alarms | アラームはありません | 没有闹钟 |
| day.0 | 월 | Mon | 月 | 一 |
| day.1 | 화 | Tue | 火 | 二 |
| day.2 | 수 | Wed | 水 | 三 |
| day.3 | 목 | Thu | 木 | 四 |
| day.4 | 금 | Fri | 金 | 五 |
| day.5 | 토 | Sat | 土 | 六 |
| day.6 | 일 | Sun | 日 | 日 |

트레이·알림(Rust):

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| TimerDoneTitle | 타이머 끝 | Timer done | タイマー終了 | 计时结束 |
| TimerDoneBody | {time} 타이머가 끝났어요 | Your {time} timer is up | {time}のタイマーが終わりました | {time}的计时器已结束 |
| AlarmTitle | 알람 {time} | Alarm {time} | アラーム {time} | 闹钟 {time} |
| ErrTime | 시각 형식이 맞지 않아요 | Invalid time | 時刻の形式が正しくありません | 时间格式无效 |

## 5단계: 기록 탭 (2026-11-01 추가·적용)

날짜·기간 표기(예: 2026년 11월 1일 (일), 10월 26일~11월 1일)는 운영체제의 언어별 날짜 형식(Intl)을 써서 따로 번역하지 않았습니다.

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| log.day | 일 | Day | 日 | 日 |
| log.week | 주 | Week | 週 | 周 |
| log.month | 월 | Month | 月 | 月 |
| log.year | 연 | Year | 年 | 年 |
| log.prev | 이전 | Previous | 前へ | 上一个 |
| log.next | 다음 | Next | 次へ | 下一个 |
| log.pickDate | 날짜 선택 | Pick a date | 日付を選択 | 选择日期 |
| log.daySub | 경과 {elapsed} · 집중 {focus} | Elapsed {elapsed} · Focus {focus} | 経過 {elapsed} · 集中 {focus} | 经过 {elapsed} · 专注 {focus} |
| log.weekSub | 하루 평균 {avg} | {avg} a day on average | 1日平均 {avg} | 日均 {avg} |
| log.monthSub | 작업한 날 {n}일 | {n} days worked | 作業した日 {n}日 | 工作 {n} 天 |
| log.yearSub | 월 평균 {avg} | {avg} a month on average | 月平均 {avg} | 月均 {avg} |
| log.sessions | 세션 | Sessions | セッション | 会话 |
| log.noSessions | 이날은 기록이 없어요 | Nothing recorded this day | この日の記録はありません | 这天没有记录 |
| log.noMemo | 메모 없음 | No note | メモなし | 无备注 |
| log.editMemo | 메모 고치기 | Edit note | メモを編集 | 编辑备注 |
| log.tapBar | 막대를 누르면 그날 기록 | Tap a bar to see that day | 棒を押すとその日の記録 | 点击柱条查看当天记录 |
| log.heatHint | 진할수록 오래 · 날짜를 누르면 그날 기록 | Darker = longer · tap a date to see that day | 濃いほど長い · 日付を押すとその日の記録 | 颜色越深时间越长 · 点击日期查看当天记录 |
| log.bestMonth | 가장 많이 한 달 {month} · {time} | Busiest month: {month} · {time} | 最も多かった月 {month} · {time} | 最多的月份 {month} · {time} |
| log.tapMonth | 막대를 누르면 그 달 | Tap a bar to see that month | 棒を押すとその月 | 点击柱条查看该月 |
| log.export | CSV 내보내기 | Export CSV | CSV 書き出し | 导出 CSV |
| log.exported | {n}줄을 저장했어요 | Saved {n} rows | {n}行を保存しました | 已保存 {n} 行 |

## 6단계: 작은 타이머 · 테마 · 시작 프로그램 (2026-11-01 추가·적용)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| mini.fold | 작은 타이머로 접기 | Fold into mini timer | ミニタイマーにする | 收起为迷你计时器 |
| mini.open | 작은 타이머 띄우기 | Open mini timer | ミニタイマーを表示 | 打开迷你计时器 |
| mini.work | 작업 | Active | 作業 | 工作 |
| mini.pin | 항상 위에 고정 | Keep on top | 常に手前に表示 | 置顶 |
| mini.cycle | 표시할 시간 바꾸기 | Switch display | 表示を切り替え | 切换显示 |
| mini.collapse | 칩으로 접기 | Collapse to chip | チップにたたむ | 收起为小条 |
| mini.close | 닫기 | Close | 閉じる | 关闭 |
| mini.chipWorking | 작업 중, 눌러서 멈춤 | Working — click to pause | 作業中、クリックで停止 | 工作中，点击暂停 |
| mini.chipStopped | 멈춤, 눌러서 재개 | Stopped — click to resume | 停止中、クリックで再開 | 已停止，点击继续 |
| general.mini | 작은 타이머 | Mini timer | ミニタイマー | 迷你计时器 |
| general.miniWork | 실 작업 | Active | 実作業 | 实际工作 |
| general.miniElapsed | 경과 | Elapsed | 経過 | 经过 |
| general.miniBoth | 둘 다 | Both | 両方 | 两者 |
| general.onTop | 항상 위에 표시 | Always on top | 常に手前に表示 | 始终置顶 |
| general.opacity | 투명도 | Opacity | 不透明度 | 不透明度 |
| general.size | 크기 | Size | サイズ | 大小 |
| general.sizeSmall | 작게 | Small | 小 | 小 |
| general.sizeNormal | 보통 | Normal | 標準 | 标准 |
| general.theme | 테마 | Theme | テーマ | 主题 |
| general.themeLight | 라이트 | Light | ライト | 浅色 |
| general.themeDark | 다크 | Dark | ダーク | 深色 |
| general.themeSystem | 시스템 | System | システム | 跟随系统 |
| general.startup | 시작 프로그램 | Startup | スタートアップ | 开机启动 |
| general.autostart | Windows 시작 시 트레이로 실행 | Start in the tray with Windows | Windows 起動時にトレイで実行 | 随 Windows 启动到托盘 |
| TrayMini (트레이) | 작은 타이머 | Mini timer | ミニタイマー | 迷你计时器 |

## 사용법 화면 · 시작 전 상태 (2026-11-01 추가·적용)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| status.ready | 시작 전 | Not started | 開始前 | 未开始 |
| today.start | 시작 | Start | 開始 | 开始 |
| TrayStart (트레이) | 시작 | Start | 開始 | 开始 |
| mini.expand | 펼치기 | Expand | 展開 | 展开 |
| guide.title | 사용법 | How to use | 使い方 | 使用方法 |
| guide.open | 사용법 | How to use | 使い方 | 使用方法 |
| guide.start.t | 시작하기 | Getting started | はじめに | 开始使用 |
| guide.start.b | 오늘 탭의 "시작"을 눌러야 시간이 흐릅니다. 앱을 켰을 때와 초기화·세션 종료 뒤에는 멈춘 채 기다려요.<br>일시정지한 뒤에는 "재개"로 이어서 셉니다.<br>초기화는 지금 세션을 0으로 되돌려요. 그때까지의 기록은 통계에 남습니다.<br>세션 종료는 '무슨 작업?' 메모를 남기고 새 세션으로 넘어갑니다. | Time only runs after you press "Start" on the Today tab. When the app opens, and after a reset or ending a session, it waits stopped.<br>After pausing, press "Resume" to continue.<br>Reset sets the current session back to 0. What you did so far stays in your stats.<br>End session saves a short note about what you worked on and starts a new session. | 今日タブの「開始」を押すと時間が進みます。起動時やリセット・セッション終了の後は止まったまま待ちます。<br>一時停止した後は「再開」で続きから数えます。<br>リセットは今のセッションを0に戻します。それまでの記録は統計に残ります。<br>セッション終了は「何の作業？」のメモを残して新しいセッションに移ります。 | 在“今天”标签按下“开始”后才会计时。启动时以及重置、结束会话后都会停住等待。<br>暂停后按“继续”接着计时。<br>重置会把当前会话归零，之前的记录仍保留在统计中。<br>结束会话会留下“做了什么？”备注并开始新的会话。 |
| guide.numbers.t | 숫자 읽는 법 | What the numbers mean | 数字の見方 | 数字的含义 |
| guide.numbers.b | 실 작업: 등록한 프로그램이 앞에 있고, 마우스·키보드 입력이 자리 비움 기준(기본 3분) 안에 있었던 시간<br>경과: 시작한 뒤 흐른 시간. 일시정지 중에만 멈추고 휴식 중에도 흐릅니다<br>집중: 실 작업 ÷ 경과 | Active: time when a tracked program was in front and you used the mouse or keyboard within the away limit (3 min by default)<br>Elapsed: time since you started. It only stops while paused, and keeps running during breaks<br>Focus: Active ÷ Elapsed | 実作業: 登録したプログラムが前面にあり、離席判定（標準3分）以内にマウス・キーボード入力があった時間<br>経過: 開始してからの時間。一時停止中だけ止まり、休憩中も進みます<br>集中: 実作業 ÷ 経過 | 实际工作：已登记的程序在前台，且在离开判定时间（默认3分钟）内有鼠标或键盘输入的时间<br>经过：开始后经过的时间。只有暂停时停止，休息时也会继续<br>专注：实际工作 ÷ 经过 |
| guide.programs.t | 추적 프로그램 | Tracked programs | 追跡するプログラム | 跟踪的程序 |
| guide.programs.b | 설정 › 추적 프로그램에서 실행 중인 프로그램을 추가합니다. 하나도 없으면 모든 프로그램을 작업으로 셉니다.<br>이름을 누르면 이름·창 제목 키워드·막대 색을 바꿀 수 있어요. 창 제목 키워드를 넣으면 제목에 그 글자가 있을 때만 셉니다(예: 크롬 + Pinterest). | Add programs in Settings › Tracked programs. If none are added, every program counts as active time.<br>Click a name to change its name, window-title keyword or bar color. With a keyword, it only counts while the window title contains it (e.g. Chrome + Pinterest). | 設定 › 追跡するプログラムで実行中のプログラムを追加します。1つもなければすべてのプログラムを作業として数えます。<br>名前を押すと名前・ウィンドウ名のキーワード・棒の色を変えられます。キーワードを入れるとタイトルにその文字があるときだけ数えます（例: Chrome + Pinterest）。 | 在 设置 › 跟踪的程序 中添加正在运行的程序。如果一个都没有，所有程序都计为工作。<br>点击名称可以修改名称、窗口标题关键词和柱条颜色。填写关键词后，只有窗口标题包含该文字时才计时（例：Chrome + Pinterest）。 |
| guide.mini.t | 작은 타이머 | Mini timer | ミニタイマー | 迷你计时器 |
| guide.mini.b | 오늘 화면 버튼 줄 오른쪽 아이콘이나 헤더의 접기 아이콘으로 띄웁니다. 항상 위에 뜨고 작업 표시줄에도 따로 보여요.<br>마우스를 올리면 아이콘이 나옵니다: 고정(항상 위) · 표시 바꾸기(실 작업/경과/둘 다) · 칩으로 접기 · 닫기<br>끌어서 옮길 수 있고 위치를 기억합니다. 두 번 클릭하면 본창으로 돌아갑니다. | Open it with the icon at the right of the Today buttons, or the fold icon in the header. It stays on top and has its own taskbar button.<br>Hover to see the icons: Keep on top · Switch display (Active/Elapsed/Both) · Collapse to chip · Close<br>Drag to move it; it remembers where it was. Double-click to go back to the main window. | 今日画面のボタン列の右のアイコン、またはヘッダーのたたむアイコンで表示します。常に手前に表示され、タスクバーにも別に表示されます。<br>マウスを乗せるとアイコンが出ます: 固定（常に手前）· 表示切り替え（実作業/経過/両方）· チップにたたむ · 閉じる<br>ドラッグで移動でき、位置を覚えます。ダブルクリックでメイン画面に戻ります。 | 用“今天”页面按钮行右侧的图标，或标题栏的收起图标打开。始终置顶，并在任务栏单独显示。<br>鼠标移上去会出现图标：置顶 · 切换显示（实际工作/经过/两者）· 收起为小条 · 关闭<br>可以拖动，并会记住位置。双击返回主窗口。 |
| guide.chip.t | 칩 (접은 작은 타이머) | Chip (collapsed mini timer) | チップ（たたんだミニタイマー） | 小条（收起的迷你计时器） |
| guide.chip.b | 초록 = 작업 중, 빨강 = 멈춤·대기·시작 전. 한 번 클릭하면 시작/일시정지가 바뀝니다.<br>다시 펼치기: 마우스를 올리면 오른쪽에 나오는 펼치기 단추, 우클릭, 두 번 클릭, 트레이 메뉴 '작은 타이머'<br>다른 창을 쓰다가 바로 두 번 클릭하면 첫 클릭이 창 활성화에 쓰여 안 펼쳐질 수 있어요. 펼치기 단추나 우클릭이 확실합니다. | Green = working, red = stopped, waiting or not started. Click once to start/pause.<br>To expand again: the expand button that appears on the right when you hover, right-click, double-click, or the tray menu "Mini timer"<br>If another window was in front, the first click of a double-click may only activate the window. The expand button or right-click always works. | 緑 = 作業中、赤 = 停止・待機・開始前。1回クリックで開始/一時停止が切り替わります。<br>もう一度広げるには: マウスを乗せると右に出る展開ボタン、右クリック、ダブルクリック、トレイメニュー「ミニタイマー」<br>他のウィンドウを使っていた直後のダブルクリックは、最初のクリックがウィンドウの切り替えに使われて広がらないことがあります。展開ボタンか右クリックが確実です。 | 绿色 = 工作中，红色 = 已停止、等待或未开始。单击一次切换开始/暂停。<br>重新展开：鼠标移上去后右侧出现的展开按钮、右键、双击，或托盘菜单“迷你计时器”<br>刚在其他窗口操作后直接双击，第一次点击可能只用于激活窗口而无法展开。展开按钮或右键最可靠。 |
| guide.alerts.t | 주기 알림과 휴식 | Interval alerts and breaks | 定期通知と休憩 | 定时提醒与休息 |
| guide.alerts.b | 설정 › 주기 알림: N분마다(경과 또는 실 작업 기준) 소리와 Windows 알림. 오늘 화면 아래 "50분마다 알림 켜기"로 바로 켤 수도 있어요.<br>휴식을 정하면 알림 뒤 그 시간 동안 실 작업을 세지 않고(경과는 흐름), 끝나면 휴식 끝 소리와 함께 다시 셉니다.<br>다음 알림은 휴식이 끝난 때부터 다시 N분 뒤입니다. 휴식 중 "휴식 끝내기"로 일찍 끝낼 수 있어요. | Settings › Interval alerts: a sound and a Windows notification every N minutes (by Elapsed or Active). You can also turn one on with "Alert every 50 min" at the bottom of Today.<br>With a break set, Active time stops counting for that long after the alert (Elapsed keeps running), then resumes with the break-end sound.<br>The next alert comes N minutes after the break ends. Press "End break" to finish early. | 設定 › 定期通知: N分ごと（経過または実作業基準）に音とWindows通知。今日画面の下の「50分ごとに通知」からすぐ有効にもできます。<br>休憩を設定すると、通知の後その時間は実作業を数えず（経過は進む）、終わると休憩終了の音とともに再び数えます。<br>次の通知は休憩が終わってからN分後です。「休憩を終える」で早めに終われます。 | 设置 › 定时提醒：每N分钟（按经过或实际工作）播放声音并显示 Windows 通知。也可以在“今天”页面底部点“每50分钟提醒”直接开启。<br>设置休息后，提醒后的这段时间不计实际工作（经过继续），结束时伴随休息结束提示音重新计时。<br>下次提醒在休息结束后N分钟。休息中可点“结束休息”提前结束。 |
| guide.shortcuts.t | 단축키 | Shortcuts | ショートカット | 快捷键 |
| guide.shortcuts.b | {pause}: 시작 · 일시정지 · 재개<br>{reset}: 초기화(확인 한 번)<br>설정 › 단축키에서 바꿀 수 있어요. 다른 프로그램이 쓰고 있는 조합이면 알려 줍니다. | {pause}: Start · Pause · Resume<br>{reset}: Reset (asks once)<br>Change them in Settings › Shortcuts. You'll be told if another program already uses a combination. | {pause}: 開始 · 一時停止 · 再開<br>{reset}: リセット（1回確認）<br>設定 › ショートカットで変更できます。他のプログラムが使っている組み合わせなら知らせます。 | {pause}：开始 · 暂停 · 继续<br>{reset}：重置（确认一次）<br>可在 设置 › 快捷键 中修改。如果组合已被其他程序占用会提示。 |
| guide.tray.t | 트레이 | Tray | トレイ | 托盘 |
| guide.tray.b | 창을 닫아도(×) 트레이에서 계속 기록하고 알림·알람도 울립니다.<br>트레이 아이콘 왼쪽 클릭 = 창 열기, 오른쪽 클릭 = 메뉴(열기 · 작은 타이머 · 시작/일시정지 · 초기화 · 종료). 완전히 끄려면 메뉴의 '종료'. | Closing the window (×) keeps recording in the tray, and alerts and alarms still ring.<br>Left-click the tray icon to open the window, right-click for the menu (Open · Mini timer · Start/Pause · Reset · Quit). Use "Quit" to exit completely. | ウィンドウを閉じても（×）トレイで記録を続け、通知やアラームも鳴ります。<br>トレイアイコンの左クリック = ウィンドウを開く、右クリック = メニュー（開く · ミニタイマー · 開始/一時停止 · リセット · 終了）。完全に終了するにはメニューの「終了」。 | 关闭窗口（×）后仍在托盘中继续记录，提醒和闹钟也照常响起。<br>左键点击托盘图标 = 打开窗口，右键 = 菜单（打开 · 迷你计时器 · 开始/暂停 · 重置 · 退出）。要彻底关闭请点菜单中的“退出”。 |
| guide.tools.t | 도구 | Tools | ツール | 工具 |
| guide.tools.b | 타이머(프리셋·직접 입력, 예: 25 또는 1:30), 스톱워치(랩), 알람(요일 반복, 요일을 안 고르면 한 번만).<br>작업 시간과 따로 돌고, 창을 닫아도 울립니다. | Timer (presets or custom, e.g. 25 or 1:30), stopwatch (laps), alarm (repeat by weekday; no weekday = once).<br>They run separately from work tracking and still ring when the window is closed. | タイマー（プリセット・直接入力、例: 25 または 1:30）、ストップウォッチ（ラップ）、アラーム（曜日の繰り返し、曜日を選ばなければ1回のみ）。<br>作業時間とは別に動き、ウィンドウを閉じても鳴ります。 | 计时器（预设或自定义，例：25 或 1:30）、秒表（计圈）、闹钟（按星期重复，不选星期则仅一次）。<br>与工作计时分开运行，关闭窗口也会响。 |
| guide.log.t | 기록 | History | 記録 | 记录 |
| guide.log.b | 일/주/월/연을 바꾸고 ◀ ▶로 과거로 이동합니다. 가운데 날짜를 누르면 날짜를 직접 고를 수 있어요.<br>막대나 날짜를 누르면 한 단계 아래(연 → 월 → 일)로 내려갑니다.<br>하루 보기에서 세션 메모를 눌러 고칠 수 있고, 아래 "CSV 내보내기"로 보고 있는 기간을 저장합니다. | Switch Day/Week/Month/Year and use ◀ ▶ to go back in time. Click the date in the middle to pick one.<br>Click a bar or a date to go one level down (Year → Month → Day).<br>In the day view, click a session note to edit it. "Export CSV" saves the period you are viewing. | 日/週/月/年を切り替え、◀ ▶ で過去に移動します。中央の日付を押すと日付を直接選べます。<br>棒や日付を押すと1段階下（年 → 月 → 日）に移ります。<br>日の表示ではセッションのメモを押して修正でき、「CSV 書き出し」で表示中の期間を保存します。 | 切换 日/周/月/年，用 ◀ ▶ 查看过去。点击中间的日期可以直接选择日期。<br>点击柱条或日期会进入下一级（年 → 月 → 日）。<br>在日视图中点击会话备注即可修改，“导出 CSV”会保存当前查看的时间段。 |

## 알림음 구간 편집 (2026-11-01 추가·적용)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| sound.fileFilter (바꿈) | 소리·영상 | Audio & video | 音声・動画 | 音频和视频 |
| clip.title | 알림음 만들기 | Make an alert sound | 通知音を作る | 制作提示音 |
| clip.loading | 파일 읽는 중… | Reading the file… | ファイルを読み込み中… | 正在读取文件… |
| clip.errRead | 이 파일에서 소리를 읽을 수 없어요 | Couldn't read audio from this file | このファイルから音声を読み込めません | 无法读取此文件中的音频 |
| clip.errLong | {min}분 이하 파일만 편집할 수 있어요 | Only files up to {min} min can be edited | 編集できるのは{min}分以内のファイルです | 只能编辑 {min} 分钟以内的文件 |
| clip.close | 닫기 | Close | 閉じる | 关闭 |
| clip.hint | 끌어서 구간 선택 · 휠로 확대 · Shift+휠로 이동 | Drag to select · Wheel to zoom · Shift+wheel to scroll | ドラッグで範囲選択・ホイールで拡大・Shift+ホイールで移動 | 拖动选择片段 · 滚轮缩放 · Shift+滚轮平移 |
| clip.zoomIn | 확대 | Zoom in | 拡大 | 放大 |
| clip.zoomOut | 축소 | Zoom out | 縮小 | 缩小 |
| clip.fitSel | 구간 보기 | Fit selection | 範囲を表示 | 显示选区 |
| clip.fitAll | 전체 보기 | Show all | 全体を表示 | 显示全部 |
| clip.start | 시작 | Start | 開始 | 开始 |
| clip.end | 끝 | End | 終了 | 结束 |
| clip.length | {sec}초 (최대 {max}초) | {sec} s (max {max} s) | {sec}秒（最大{max}秒） | {sec} 秒（最长 {max} 秒） |
| clip.play | ▶ 들어보기 | ▶ Preview | ▶ 試聴 | ▶ 试听 |
| clip.stop | ■ 멈춤 | ■ Stop | ■ 停止 | ■ 停止 |
| clip.fade | 앞뒤 부드럽게 | Fade in/out | フェードイン・アウト | 淡入淡出 |
| clip.cancel | 취소 | Cancel | キャンセル | 取消 |
| clip.save | 저장 | Save | 保存 | 保存 |
| clip.saving | 저장 중… | Saving… | 保存中… | 正在保存… |
| ErrMediaType (Rust) | 소리나 영상 파일만 열 수 있어요 | Only audio or video files can be opened | 音声または動画ファイルのみ開けます | 只能打开音频或视频文件 |
| ErrMediaBig (Rust) | {mb}MB 이하 파일만 열 수 있어요 | Only files up to {mb} MB can be opened | {mb}MB 以下のファイルのみ開けます | 只能打开 {mb}MB 以下的文件 |
| guide.alerts.b (덧붙임) | 소리 목록의 "파일에서 고르기…"로 mp3나 영상 파일을 열면 파형에서 원하는 구간(최대 60초)만 잘라 알림음으로 쓸 수 있어요. | Pick "Choose a file…" in a sound list to open an mp3 or video file and cut the part you want (up to 60 s) from its waveform as an alert sound. | 音のリストで「ファイルから選ぶ…」からmp3や動画ファイルを開くと、波形から好きな範囲（最大60秒）だけ切り取って通知音にできます。 | 在声音列表中点“从文件选择…”打开 mp3 或视频文件，可在波形上截取想要的片段（最长 60 秒）作为提示音。 |

## 기록 삭제 (2026-11-01 추가·적용)

| 키 | 한국어 | English | 日本語 | 中文(简体) |
| --- | --- | --- | --- | --- |
| log.delete | 삭제 | Delete | 削除 | 删除 |
| log.deleted | 삭제했어요 | Deleted | 削除しました | 已删除 |
| log.current | 진행 중 | In progress | 進行中 | 进行中 |
| log.deleteSessionAsk | 이 세션 기록을 삭제할까요? 되돌릴 수 없어요. | Delete this session's record? This can't be undone. | このセッションの記録を削除しますか？元に戻せません。 | 删除此会话的记录？此操作无法撤销。 |
| log.deleteCurrentAsk | 진행 중인 세션이에요. 삭제하면 지금 세션도 0으로 돌아가고 시작 전 상태가 돼요. 되돌릴 수 없어요. | This session is in progress. Deleting it also resets the current session to 0 and stops it. This can't be undone. | 進行中のセッションです。削除すると今のセッションも0に戻り、開始前の状態になります。元に戻せません。 | 这是正在进行的会话。删除后当前会话也会归零并回到未开始状态。此操作无法撤销。 |
| log.deleteDay | 이 날 기록 삭제 | Delete this day | この日の記録を削除 | 删除当天记录 |
| log.deleteDayAsk | {date} 기록을 모두 삭제할까요? 되돌릴 수 없어요. | Delete all records for {date}? This can't be undone. | {date} の記録をすべて削除しますか？元に戻せません。 | 删除 {date} 的全部记录？此操作无法撤销。 |
| guide.log.b (덧붙임) | 하루 보기의 휴지통 단추나 "이 날 기록 삭제"로 잘못 남은 기록을 지울 수 있어요(되돌릴 수 없음). | Use the trash button in the day view or "Delete this day" to remove records you didn't mean to keep (can't be undone). | 日の表示のゴミ箱ボタンや「この日の記録を削除」で、間違えて残った記録を消せます（元に戻せません）。 | 可用日视图中的垃圾桶按钮或“删除当天记录”删除误留的记录（无法撤销）。 |
