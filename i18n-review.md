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
