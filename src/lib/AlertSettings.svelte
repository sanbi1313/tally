<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { minutes, t } from "./i18n.svelte";
  import SoundSelect from "./SoundSelect.svelte";
  import type { AlertRow } from "./types";

  let alerts = $state<AlertRow[]>([]);
  let error = $state("");

  async function load() {
    alerts = await invoke<AlertRow[]>("list_alerts");
  }
  onMount(load);

  async function run(f: () => Promise<unknown>) {
    error = "";
    try {
      await f();
    } catch (e) {
      error = String(e);
    }
    await load();
  }

  const save = (a: AlertRow) => run(() => invoke("update_alert", { alert: a }));
  const setSound = (a: AlertRow, which: "alert" | "break", value: string) =>
    run(() => invoke("set_alert_sound", { id: a.id, which, value }));

  /** 숫자 칸: 범위를 벗어나면 원래 값으로 */
  function setNum(a: AlertRow, field: "everyMin" | "breakMin", v: string, min: number, max: number) {
    const n = Math.round(Number(v));
    if (v.trim() !== "" && n >= min && n <= max) save({ ...a, [field]: n });
    else load();
  }
</script>

<div class="label">{t("alert.title")}</div>
{#each alerts as a (a.id)}
  <div class="alert">
    <div class="line">
      <button
        type="button"
        class="switch"
        class:on={a.enabled}
        aria-label={t("alert.toggle", { every: minutes(a.everyMin) })}
        aria-pressed={a.enabled}
        onclick={() => save({ ...a, enabled: !a.enabled })}
      >
        <span></span>
      </button>
      <span class="numfield" class:off={!a.enabled}>
        <input
          type="number"
          class="num"
          min="1"
          max="1440"
          value={a.everyMin}
          aria-label={t("alert.every")}
          onchange={(e) => setNum(a, "everyMin", e.currentTarget.value, 1, 1440)}
        />{t("alert.everySuffix")}
      </span>
      <div class="seg">
        <button type="button" class:on={a.basis === "elapsed"} onclick={() => save({ ...a, basis: "elapsed" })}>{t("alert.basisElapsed")}</button>
        <button type="button" class:on={a.basis === "work"} onclick={() => save({ ...a, basis: "work" })}>{t("alert.basisWork")}</button>
      </div>
    </div>

    <div class="line">
      <span class="key">{t("alert.sound")}</span>
      <SoundSelect value={a.sound} fallback="builtin:bell" label={t("alert.sound")} onchange={(v) => setSound(a, "alert", v)} />
      <button type="button" class="text" onclick={() => invoke("preview_sound", { sound: a.sound, volume: a.volume })}>{t("alert.preview")}</button>
    </div>
    <div class="line">
      <span class="key">{t("alert.volume")}</span>
      <input
        type="range"
        min="0"
        max="1"
        step="0.05"
        value={a.volume}
        aria-label={t("alert.volume")}
        onchange={(e) => save({ ...a, volume: Number(e.currentTarget.value) })}
      />
    </div>

    <div class="line">
      <span class="key">{t("alert.break")}</span>
      <span class="numfield">
        <input
          type="number"
          class="num"
          min="0"
          max="240"
          value={a.breakMin}
          aria-label={t("alert.breakMin")}
          title={t("alert.breakNone")}
          onchange={(e) => setNum(a, "breakMin", e.currentTarget.value, 0, 240)}
        />{t("alert.breakSuffix")}
      </span>
    </div>
    {#if a.breakMin > 0}
      <div class="line">
        <span class="key">{t("alert.breakSound")}</span>
        <SoundSelect
          value={a.breakSound}
          fallback="builtin:rise"
          label={t("alert.breakSound")}
          onchange={(v) => setSound(a, "break", v)}
        />
        <button type="button" class="text" onclick={() => invoke("preview_sound", { sound: a.breakSound ?? "builtin:rise", volume: a.volume })}
          >{t("alert.preview")}</button
        >
      </div>
    {/if}

    <div class="line tail">
      <button type="button" class="text danger" onclick={() => run(() => invoke("delete_alert", { id: a.id }))}>{t("alert.delete")}</button>
    </div>
  </div>
{/each}
{#if error}<div class="error">{error}</div>{/if}
<button type="button" class="link" onclick={() => run(() => invoke("add_alert", { everyMin: 50, basis: "work" }))}>{t("alert.add")}</button>

<style>
  .label {
    font-size: 13px;
    color: var(--sub);
  }
  .alert {
    padding: 10px 0 2px;
    border-bottom: 1px solid var(--line);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .line {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 15px;
    min-width: 0;
  }
  .key {
    width: 78px;
    flex-shrink: 0;
    font-size: 13px;
    color: var(--sub);
  }
  .numfield {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .numfield.off {
    color: var(--sub);
  }
  .numfield input {
    width: 52px;
    height: 30px;
    box-sizing: border-box;
    padding: 0 6px;
    margin-right: 2px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: inherit;
    font: inherit;
    font-weight: 600;
    text-align: right;
  }
  .numfield input:focus {
    outline: none;
    border-color: var(--ink);
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
  .seg {
    display: flex;
    border: 1px solid var(--line);
    border-radius: 6px;
    padding: 3px;
    gap: 3px;
  }
  .seg button {
    height: 28px;
    padding: 0 10px;
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
  input[type="range"] {
    flex: 1;
    accent-color: var(--accent);
  }
  .tail {
    justify-content: flex-end;
  }
  .text {
    height: 30px;
    padding: 0;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 14px;
    flex-shrink: 0;
  }
  .text:hover {
    color: var(--ink);
  }
  .text.danger:hover {
    color: var(--accent);
  }
  .error {
    margin-top: 6px;
    font-size: 13px;
    color: var(--accent);
  }
  .link {
    align-self: flex-start;
    height: 40px;
    padding: 0;
    border: none;
    background: none;
    color: var(--accent);
    font-size: 15px;
  }
</style>
