# Tally

지정한 프로그램(클립 스튜디오, 포토샵 등)이 앞에 떠 있고 손이 움직이는 시간만 "실 작업 시간"으로 세는 Windows용 작업 타이머. 트레이에 상주하고, 작은 창 하나로 오늘의 작업량을 보여준다.

- 실 작업 / 경과 시간 분리, 자리 비움 감지, 프로그램별 시간
- 주기 알림(경과·실 작업 기준), 자동 휴식, 내장 알림음 6종·사용자 소리
- mp3·영상 파일에서 파형을 보고 구간만 잘라 알림음으로
- 타이머 · 스톱워치 · 시각 알람
- 일/주/월/연 기록, 하루 타임라인, 세션 메모, CSV 내보내기
- 항상 위에 뜨는 작은 타이머와 상태색 칩
- 한국어 · English · 日本語 · 中文(简体), 라이트/다크 테마
- Tauri 2 + Svelte 5, 실행 파일 약 7MB, 설치 파일 약 3MB

## 빌드

준비물: Rust(rustup), Node.js LTS, Visual Studio Build Tools(C++ 데스크톱), Windows 10/11

```bash
npm install
npm run tauri build
```

결과물: `src-tauri/target/release/bundle/nsis/Tally_x.y.z_x64-setup.exe`(설치형), `src-tauri/target/release/tally.exe`(포터블)

개발 모드, 데이터 위치, 테스트, 폴더 구조는 [BUILD.md](BUILD.md)에 있다.

## 문서

- [USAGE.md](USAGE.md) — 사용법(앱 안에서는 헤더의 ? 단추)
- [BUILD.md](BUILD.md) — 빌드·실행·데이터·테스트 안내, 단계별 구현 내용
- [i18n-review.md](i18n-review.md) — 4개 언어 번역표
- [assets/logo/](assets/logo/) — 로고·트레이 아이콘 원본 SVG

## 라이선스

[MIT](LICENSE). 글꼴 Pretendard는 SIL Open Font License 1.1([src/assets/fonts/LICENSE-Pretendard.txt](src/assets/fonts/LICENSE-Pretendard.txt)).
