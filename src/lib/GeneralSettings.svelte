<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { availableLangs, LANG_NAMES, setLang, t, type Lang } from "./i18n.svelte";
  import { setTheme, themeSetting, type ThemeSetting } from "./theme.svelte";
  import type { MiniSettings, SettingsOut } from "./types";

  type Which = "pause" | "reset";
  let s = $state<SettingsOut | null>(null);
  let capturing = $state<Which | null>(null);
  let error = $state("");

  onMount(async () => {
    s = await invoke<SettingsOut>("get_settings");
  });

  async function setIdle(v: string) {
    const n = Math.round(Number(v));
    if (n >= 1 && n <= 60) await invoke("set_idle_minutes", { minutes: n });
    s = await invoke<SettingsOut>("get_settings");
  }

  async function setShortcut(which: Which, accel: string) {
    error = "";
    try {
      s = await invoke<SettingsOut>("set_shortcut", { which, accel });
    } catch (e) {
      error = String(e);
    }
  }

  /** KeyboardEvent → "Ctrl+Alt+P". 수식키만 눌렀으면 null */
  function accelOf(e: KeyboardEvent): string | null {
    let key: string;
    if (/^Key[A-Z]$/.test(e.code)) key = e.code.slice(3);
    else if (/^Digit\d$/.test(e.code)) key = e.code.slice(5);
    else if (/^F\d{1,2}$/.test(e.code)) key = e.code;
    else if (["Space", "Home", "End", "PageUp", "PageDown", "Insert"].includes(e.code)) key = e.code;
    else return null;
    const mods = [e.ctrlKey && "Ctrl", e.altKey && "Alt", e.shiftKey && "Shift", e.metaKey && "Super"].filter(Boolean);
    // 수식키 없는 단축키는 평소 타이핑을 가로채므로 F키만 허용
    if (!mods.length && !key.startsWith("F")) return null;
    return [...mods, key].join("+");
  }

  function onKey(e: KeyboardEvent) {
    if (!capturing) return;
    e.preventDefault();
    e.stopPropagation();
    if (e.key === "Escape") {
      capturing = null;
      return;
    }
    if (e.key === "Backspace" || e.key === "Delete") {
      setShortcut(capturing, "");
      capturing = null;
      return;
    }
    const accel = accelOf(e);
    if (!accel) return;
    setShortcut(capturing, accel);
    capturing = null;
  }

  const rows = [
    ["pause", "general.scPause"],
    ["reset", "general.scReset"],
  ] as const;

  async function setMini(patch: Partial<MiniSettings>) {
    if (!s) return;
    const mini = { ...s.mini, ...patch };
    s.mini = mini;
    await invoke("set_mini", { mini });
  }

  async function changeTheme(v: ThemeSetting) {
    await setTheme(v);
  }

  async function setAutostart(on: boolean) {
    error = "";
    try {
      const now = await invoke<boolean>("set_autostart", { on });
      if (s) s.autostart = now;
    } catch (e) {
      error = String(e);
    }
  }

  const showOpts = [
    ["work", "general.miniWork"],
    ["elapsed", "general.miniElapsed"],
    ["both", "general.miniBoth"],
  ] as const;
  const sizeOpts = [
    ["small", "general.sizeSmall"],
    ["normal", "general.sizeNormal"],
  ] as const;
  const themeOpts = [
    ["light", "general.themeLight"],
    ["dark", "general.themeDark"],
    ["system", "general.themeSystem"],
  ] as const;

  async function changeLang(l: Lang) {
    setLang(l);
    await invoke("set_language", { language: l });
    s = await invoke<SettingsOut>("get_settings");
  }
</script>

<svelte:window onkeydown={onKey} />

{#if s}
  <div class="label">{t("general.mini")}</div>
  <div class="seg">
    {#each showOpts as [v, k]}
      <button type="button" class:on={s.mini.show === v} onclick={() => setMini({ show: v })}>{t(k)}</button>
    {/each}
  </div>
  <div class="opt-row">
    <span>{t("general.onTop")}</span>
    <button
      type="button"
      class="switch"
      class:on={s.mini.pinned}
      aria-label={t("general.onTop")}
      aria-pressed={s.mini.pinned}
      onclick={() => s && setMini({ pinned: !s.mini.pinned })}><span></span></button
    >
  </div>
  <div class="opt-row">
    <span>{t("general.opacity")}</span>
    <span class="range">
      <input
        type="range"
        min="0.6"
        max="1"
        step="0.05"
        value={s.mini.opacity}
        aria-label={t("general.opacity")}
        onchange={(e) => setMini({ opacity: Number(e.currentTarget.value) })}
      />
      <span class="num pct">{Math.round(s.mini.opacity * 100)}%</span>
    </span>
  </div>
  <div class="opt-row">
    <span>{t("general.size")}</span>
    <div class="seg small">
      {#each sizeOpts as [v, k]}
        <button type="button" class:on={s.mini.size === v} onclick={() => setMini({ size: v })}>{t(k)}</button>
      {/each}
    </div>
  </div>
  <button type="button" class="link" onclick={() => invoke("open_mini")}>{t("mini.open")}</button>

  <div class="label" style="margin-top: 18px">{t("general.idle")}</div>
  <div class="idle">
    <input
      type="number"
      class="num"
      min="1"
      max="60"
      value={s.idleMinutes}
      aria-label={t("general.idleMin")}
      onchange={(e) => setIdle(e.currentTarget.value)}
    />{t("general.idleSuffix")}
  </div>

  <div class="label" style="margin-top: 22px">{t("general.theme")}</div>
  <div class="seg">
    {#each themeOpts as [v, k]}
      <button type="button" class:on={themeSetting() === v} onclick={() => changeTheme(v)}>{t(k)}</button>
    {/each}
  </div>

  <div class="label" style="margin-top: 22px">{t("general.shortcuts")}</div>
  {#each rows as [which, label]}
    <div class="key-row">
      <span>{t(label)}</span>
      <button type="button" class="key" class:capturing={capturing === which} onclick={() => (capturing = capturing === which ? null : which)}>
        {#if capturing === which}
          {t("general.scPress")}
        {:else}
          {s.shortcuts[which] || t("general.scNone")}
        {/if}
      </button>
    </div>
    {#if s.shortcutErrors.includes(which)}
      <div class="error">{t("general.scTaken")}</div>
    {/if}
  {/each}
  {#if capturing}<div class="hint">{t("general.scHint")}</div>{/if}
  {#if error}<div class="error">{error}</div>{/if}

  <div class="label" style="margin-top: 22px">{t("general.startup")}</div>
  <div class="opt-row">
    <span>{t("general.autostart")}</span>
    <button
      type="button"
      class="switch"
      class:on={s.autostart}
      aria-label={t("general.autostart")}
      aria-pressed={s.autostart}
      onclick={() => s && setAutostart(!s.autostart)}><span></span></button
    >
  </div>

  {#if availableLangs.length > 1}
    <div class="label" style="margin-top: 22px">{t("general.language")}</div>
    <div class="seg">
      {#each availableLangs as l}
        <button type="button" class:on={s.language === l} lang={l} onclick={() => changeLang(l)}>{LANG_NAMES[l]}</button>
      {/each}
    </div>
  {/if}
{/if}

<style>
  .label {
    font-size: 13px;
    color: var(--sub);
  }
  .idle {
    margin-top: 8px;
    font-size: 15px;
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .idle input {
    width: 44px;
    height: 30px;
    box-sizing: border-box;
    padding: 0 6px;
    margin-right: 2px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-weight: 600;
    text-align: right;
  }
  .idle input:focus {
    outline: none;
    border-color: var(--ink);
  }
  .key-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid var(--line);
    font-size: 15px;
  }
  .key {
    height: 32px;
    min-width: 120px;
    padding: 0 10px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font-size: 14px;
    font-variant-numeric: tabular-nums;
  }
  .key.capturing {
    border-color: var(--accent);
    color: var(--accent);
  }
  .seg {
    display: flex;
    border: 1px solid var(--line);
    border-radius: 6px;
    padding: 3px;
    gap: 3px;
    margin-top: 8px;
  }
  .seg button {
    flex: 1;
    height: 32px;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    background: transparent;
    color: var(--sub);
  }
  .seg button.on {
    background: var(--surface);
    color: var(--ink);
    font-weight: 600;
  }
  .opt-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-top: 10px;
    font-size: 15px;
  }
  .range {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .range input {
    width: 140px;
    accent-color: var(--accent);
  }
  .pct {
    width: 40px;
    text-align: right;
    font-size: 14px;
    color: var(--sub);
  }
  .seg.small {
    margin-top: 0;
  }
  .seg.small button {
    flex: none;
    padding: 0 14px;
  }
  .switch {
    width: 44px;
    height: 26px;
    border-radius: 13px;
    border: none;
    padding: 3px;
    display: flex;
    justify-content: flex-start;
    background: var(--line);
    flex-shrink: 0;
  }
  .switch.on {
    justify-content: flex-end;
    background: var(--accent);
  }
  .switch span {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--surface);
  }
  .link {
    align-self: flex-start;
    height: 36px;
    margin-top: 4px;
    padding: 0;
    border: none;
    background: none;
    color: var(--accent);
    font-size: 15px;
  }
  .hint {
    margin-top: 6px;
    font-size: 13px;
    color: var(--sub);
  }
  .error {
    margin-top: 6px;
    font-size: 13px;
    color: var(--accent);
  }
</style>
