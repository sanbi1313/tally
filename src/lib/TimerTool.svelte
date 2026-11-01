<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { countdown } from "./format";
  import { minutes, t } from "./i18n.svelte";
  import SoundSelect from "./SoundSelect.svelte";
  import type { TimerSettings, TimerView } from "./types";

  let { timer, now }: { timer: TimerView; now: number } = $props();

  let settings = $state<TimerSettings | null>(null);
  let custom = $state<string | null>(null); // 직접 입력 칸(열려 있을 때만 문자열)
  let editing = $state(false);
  let error = $state("");

  onMount(load);
  async function load() {
    settings = await invoke<TimerSettings>("get_timer_settings");
  }

  const running = $derived(timer.endsAtMs !== null);
  const remaining = $derived(running ? Math.max(0, (timer.endsAtMs ?? 0) - now) : timer.remainingMs);
  const touched = $derived(running || timer.remainingMs !== timer.durationMs);

  const setMs = (ms: number) => invoke("timer_set", { ms });

  /** "25" → 25분, "1:30" → 1분 30초 */
  function parseCustom(v: string): number | null {
    const m = v.trim().match(/^(\d{1,4})(?::(\d{1,2}))?$/);
    if (!m) return null;
    const ms = (Number(m[1]) * 60 + Number(m[2] ?? 0)) * 1000;
    return ms >= 1000 && ms <= 24 * 3_600_000 ? ms : null;
  }

  async function applyCustom(e: SubmitEvent) {
    e.preventDefault();
    const ms = parseCustom(custom ?? "");
    if (ms === null) return;
    await setMs(ms);
    custom = null;
  }

  async function savePresets(i: number, v: string) {
    if (!settings) return;
    const n = Math.round(Number(v));
    if (!(n >= 1 && n <= 1440)) return load();
    const presets = settings.presets.map((p, j) => (j === i ? n : p));
    await invoke("set_timer_settings", { presets, volume: settings.volume });
    await load();
  }

  async function setSound(value: string) {
    error = "";
    try {
      await invoke("set_timer_sound", { value });
    } catch (e) {
      error = String(e);
    }
    await load();
  }
</script>

<div class="timer">
  <div class="label">{t("tool.timer")}</div>
  <div class="big num">{countdown(remaining)}</div>

  {#if settings}
    {#if editing}
      <div class="presets">
        {#each settings.presets as p, i}
          <input
            type="number"
            class="num"
            min="1"
            max="1440"
            value={p}
            aria-label={t("timer.presetMin", { n: i + 1 })}
            onchange={(e) => savePresets(i, e.currentTarget.value)}
          />
        {/each}
        <button type="button" class="text" onclick={() => (editing = false)}>{t("timer.done")}</button>
      </div>
    {:else if custom !== null}
      <form class="presets" onsubmit={applyCustom}>
        <input
          type="text"
          class="num wide"
          placeholder={t("timer.customPlaceholder")}
          bind:value={custom}
          aria-label={t("timer.custom")}
        />
        <button type="submit" class="chip" disabled={parseCustom(custom) === null}>{t("timer.set")}</button>
        <button type="button" class="text" onclick={() => (custom = null)}>{t("today.cancel")}</button>
      </form>
    {:else}
      <div class="presets">
        {#each settings.presets as p}
          <button type="button" class="chip" class:on={timer.durationMs === p * 60_000} onclick={() => setMs(p * 60_000)}>
            {minutes(p)}
          </button>
        {/each}
        <button type="button" class="text" onclick={() => (custom = "")}>{t("timer.custom")}</button>
      </div>
    {/if}
  {/if}

  <div class="row">
    {#if running}
      <button type="button" class="primary" onclick={() => invoke("timer_pause")}>{t("timer.pause")}</button>
    {:else}
      <button type="button" class="primary" onclick={() => invoke("timer_start")}>
        {touched ? t("timer.resume") : t("timer.start")}
      </button>
    {/if}
    {#if touched}
      <button type="button" class="text" onclick={() => invoke("timer_reset")}>{t("timer.reset")}</button>
    {/if}
  </div>

  {#if settings}
    <div class="sound">
      <span class="label">{t("timer.whenDone")}</span>
      <SoundSelect value={settings.sound} fallback="builtin:chime" label={t("timer.whenDone")} onchange={setSound} />
      <button type="button" class="text" onclick={() => invoke("preview_sound", { sound: settings?.sound ?? "builtin:chime", volume: settings?.volume ?? 0.8 })}
        >{t("alert.preview")}</button
      >
    </div>
    {#if !editing}
      <button type="button" class="text edit" onclick={() => ((editing = true), (custom = null))}>{t("timer.editPresets")}</button>
    {/if}
  {/if}
  {#if error}<div class="error">{error}</div>{/if}
</div>

<style>
  .timer {
    display: flex;
    flex-direction: column;
    margin-top: 40px;
  }
  .label {
    font-size: 13px;
    color: var(--sub);
  }
  .big {
    font-size: 56px;
    font-weight: 500;
    line-height: 1.1;
  }
  .presets {
    display: flex;
    gap: 8px;
    margin-top: 20px;
    align-items: center;
    flex-wrap: wrap;
  }
  .chip {
    height: 36px;
    padding: 0 14px;
    border-radius: 6px;
    border: 1px solid var(--line);
    background: none;
    color: var(--ink);
    font-size: 14px;
  }
  .chip.on {
    border-color: var(--ink);
    font-weight: 600;
  }
  .chip:disabled {
    color: var(--idle);
    cursor: default;
  }
  .presets input {
    width: 64px;
    height: 36px;
    box-sizing: border-box;
    padding: 0 8px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font-size: 14px;
    text-align: right;
  }
  .presets input.wide {
    width: 150px;
    text-align: left;
  }
  .presets input:focus {
    outline: none;
    border-color: var(--ink);
  }
  .row {
    display: flex;
    gap: 4px;
    margin-top: 28px;
    align-items: center;
  }
  .primary {
    height: 44px;
    padding: 0 26px;
    border-radius: 6px;
    border: none;
    background: var(--accent);
    color: var(--on-accent);
    font-size: 15px;
    font-weight: 500;
  }
  .text {
    height: 36px;
    padding: 0 10px;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 14px;
    border-radius: 6px;
  }
  .row .text {
    height: 44px;
    font-size: 15px;
    padding: 0 12px;
  }
  .text:hover {
    color: var(--ink);
  }
  .sound {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 28px;
  }
  .edit {
    align-self: flex-start;
    margin-top: 6px;
    padding: 0;
  }
  .error {
    margin-top: 6px;
    font-size: 13px;
    color: var(--accent);
  }
</style>
