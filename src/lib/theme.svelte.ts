// 테마: settings.json의 "system" | "light" | "dark"를 두 창(본창·작은 타이머)이 같이 따른다.
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { SettingsOut } from "./types";

export type ThemeSetting = "system" | "light" | "dark";

const media = window.matchMedia("(prefers-color-scheme: dark)");
const state = $state<{ setting: ThemeSetting; systemDark: boolean }>({ setting: "system", systemDark: media.matches });

/** 지금 실제로 보이는 테마 */
export function resolved(): "light" | "dark" {
  return state.setting === "system" ? (state.systemDark ? "dark" : "light") : state.setting;
}

export function themeSetting(): ThemeSetting {
  return state.setting;
}

function apply() {
  document.documentElement.dataset.theme = resolved();
}

async function reload() {
  const s = await invoke<SettingsOut>("get_settings");
  state.setting = (["light", "dark"].includes(s.theme) ? s.theme : "system") as ThemeSetting;
  apply();
}

/** 앱 시작 때 한 번. 시스템 테마 변화와 다른 창에서 바꾼 설정을 따라간다 */
export function initTheme() {
  media.addEventListener("change", (e) => {
    state.systemDark = e.matches;
    apply();
  });
  listen("tally://settings", reload);
  return reload();
}

export async function setTheme(t: ThemeSetting) {
  state.setting = t;
  apply();
  await invoke("set_theme", { theme: t });
}
