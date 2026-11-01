<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { focus, hms, mmss } from "./lib/format";
  import { minutes, t } from "./lib/i18n.svelte";
  import { colorVar, type View } from "./lib/types";

  let { snap, askReset = $bindable(false) }: { snap: View; askReset: boolean } = $props();

  // 버튼 줄은 평소 / 초기화 확인 / 세션 종료 메모 중 하나
  let askMemo = $state(false);
  let memo = $state("");

  const onBreak = $derived(snap.breakRemainingMs !== null);
  const statusText = $derived(
    snap.kind === "work"
      ? t("status.work", { app: snap.app ?? "" })
      : snap.kind === "paused" && !snap.started
        ? t("status.ready")
        : t(`status.${snap.kind}`),
  );
  // 같은 이름은 한 줄로(예: 모든 프로그램 모드에서 쓰다가 같은 프로그램을 등록한 경우)
  const apps = $derived.by(() => {
    const m = new Map<string, View["apps"][number]>();
    for (const a of snap.apps) {
      const prev = m.get(a.label);
      m.set(a.label, prev ? { ...prev, workMs: prev.workMs + a.workMs, color: prev.color ?? a.color } : a);
    }
    return [...m.values()].sort((x, y) => y.workMs - x.workMs);
  });
  // 막대 길이 = 이번 세션 실 작업 중 비율
  const total = $derived(Math.max(1, snap.workMs));

  async function doReset() {
    await invoke("reset");
    askReset = false;
  }

  function openMemo() {
    askReset = false;
    memo = "";
    askMemo = true;
  }

  async function endSession(e?: SubmitEvent) {
    e?.preventDefault();
    await invoke("end_session", { note: memo.trim() || null });
    askMemo = false;
  }

  const nextMin = $derived(snap.nextAlert ? Math.max(1, Math.ceil(snap.nextAlert.remainingMs / 60_000)) : 0);
  // "다음 알림 {min} 후"에서 숫자만 굵게
  const nextParts = $derived(t("today.nextAlert", { min: "\u0000" }).split("\u0000"));

  function onKey(e: KeyboardEvent) {
    if (e.key !== "Escape") return;
    askReset = false;
    askMemo = false;
  }

  function autofocus(el: HTMLElement) {
    el.focus();
  }
</script>

<svelte:window onkeydown={onKey} />

<div class="today">
  <div class="status">
    <span class="dot {snap.kind}"></span>
    <span>{statusText}</span>
  </div>

  <div class="label" style="margin-top: 18px">{t("today.work")}</div>
  <div class="big num">{hms(snap.workMs)}</div>

  <div class="minor">
    <div><span class="label">{t("today.elapsed")}</span><span class="num">{hms(snap.elapsedMs)}</span></div>
    <div><span class="label">{t("today.focus")}</span><span class="num">{focus(snap.workMs, snap.elapsedMs)}</span></div>
  </div>

  {#if askReset}
    <div class="row confirm">
      <span>{t("today.resetConfirm")}</span>
      <button type="button" class="primary small" onclick={doReset}>{t("today.reset")}</button>
      <button type="button" class="text small" onclick={() => (askReset = false)}>{t("today.cancel")}</button>
    </div>
  {:else if askMemo}
    <form class="row confirm" onsubmit={endSession}>
      <input type="text" class="memo" placeholder={t("today.memoPlaceholder")} maxlength="80" bind:value={memo} use:autofocus />
      <button type="submit" class="primary small">{t("today.end")}</button>
      <button type="button" class="text small" onclick={() => (askMemo = false)}>{t("today.cancel")}</button>
    </form>
  {:else}
    <div class="row">
      <button type="button" class="primary" onclick={() => invoke("toggle_pause")}>
        {snap.kind === "paused" ? (snap.started ? t("today.resume") : t("today.start")) : onBreak ? t("today.endBreak") : t("today.pause")}
      </button>
      <button type="button" class="text" onclick={() => (askReset = true)}>{t("today.reset")}</button>
      <button type="button" class="text" onclick={openMemo}>{t("today.endSession")}</button>
      <button type="button" class="mini-btn" aria-label={t("mini.open")} title={t("mini.open")} onclick={() => invoke("open_mini")}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
          ><rect x="3" y="4" width="18" height="16" rx="2" /><rect x="11" y="11" width="8" height="6" rx="1" /></svg
        >
      </button>
    </div>
  {/if}
  <div class="note">
    {#if onBreak}
      <span class="break">{t("today.breakLeft", { time: mmss(snap.breakRemainingMs ?? 0) })}</span>
    {:else if snap.prevWorkMs !== null}
      {t("today.prevKept", { time: hms(snap.prevWorkMs) })}
    {/if}
  </div>

  {#if apps.length}
    <div class="label" style="margin-top: 18px">{t("today.byProgram")}</div>
    <div class="apps">
      {#each apps as a (a.label)}
        <div class="app">
          <div class="app-head"><span>{a.label}</span><span class="num">{hms(a.workMs)}</span></div>
          <div class="track"><div class="bar" style="width: {(a.workMs / total) * 100}%; background: {colorVar(a.color)}"></div></div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="spacer"></div>
  <div class="footer">
    {#if snap.nextAlert}
      <span>{nextParts[0]}<span class="num strong">{minutes(nextMin)}</span>{nextParts[1] ?? ""}</span>
      <span class="label"
        >{t("today.alertBasis", {
          basis: snap.nextAlert.basis === "work" ? t("alert.basisWork") : t("alert.basisElapsed"),
          every: minutes(snap.nextAlert.everyMin),
        })}{snap.nextAlert.breakMin ? t("today.alertBreak", { min: minutes(snap.nextAlert.breakMin) }) : ""}</span
      >
    {:else}
      <span class="label">{t("today.noAlert")}</span>
      <button type="button" class="enable" onclick={() => invoke("quick_enable_alert")}>{t("today.enableAlert")}</button>
    {/if}
  </div>
</div>

<style>
  .today {
    display: flex;
    flex-direction: column;
    flex: 1;
  }
  .status {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    color: var(--sub);
  }
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    box-sizing: border-box;
    background: var(--idle);
  }
  .dot.work {
    background: var(--accent);
  }
  .dot.paused {
    background: none;
    border: 1.5px solid var(--sub);
  }
  .label {
    font-size: 13px;
    color: var(--sub);
  }
  .big {
    font-size: 56px;
    font-weight: 500;
    line-height: 1.1;
    margin-top: 2px;
  }
  .minor {
    display: flex;
    gap: 28px;
    margin-top: 10px;
    align-items: baseline;
  }
  .minor > div {
    display: flex;
    align-items: baseline;
    gap: 8px;
  }
  .minor .num {
    font-size: 22px;
  }
  .row {
    display: flex;
    gap: 4px;
    margin-top: 22px;
    align-items: center;
    height: 44px;
  }
  .confirm > span {
    font-size: 15px;
    margin-right: 8px;
  }
  .memo {
    flex: 1;
    min-width: 0;
    height: 36px;
    box-sizing: border-box;
    padding: 0 10px;
    margin-right: 4px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 15px;
  }
  .memo:focus {
    outline: none;
    border-color: var(--ink);
  }
  .primary {
    height: 44px;
    padding: 0 22px;
    border-radius: 6px;
    border: none;
    background: var(--accent);
    color: var(--on-accent);
    font-size: 15px;
    font-weight: 500;
  }
  .text {
    height: 44px;
    padding: 0 12px;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 15px;
    border-radius: 6px;
  }
  .text:hover {
    color: var(--ink);
  }
  .primary.small {
    height: 36px;
    padding: 0 14px;
    font-size: 14px;
  }
  .text.small {
    height: 36px;
    padding: 0 10px;
    font-size: 14px;
  }
  .note {
    margin-top: 6px;
    font-size: 13px;
    color: var(--sub);
    min-height: 18px;
  }
  .apps {
    display: flex;
    flex-direction: column;
    gap: 14px;
    margin-top: 12px;
  }
  .app {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .app-head {
    display: flex;
    justify-content: space-between;
    font-size: 15px;
  }
  .track {
    height: 4px;
    border-radius: 2px;
    background: var(--line);
  }
  .bar {
    height: 4px;
    border-radius: 2px;
  }
  .footer {
    padding-top: 14px;
    border-top: 1px solid var(--line);
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    font-size: 15px;
  }
  .spacer {
    flex: 1;
    min-height: 18px;
  }
  .strong {
    font-weight: 600;
  }
  .mini-btn {
    margin-left: auto;
    width: 44px;
    height: 44px;
    flex-shrink: 0;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: none;
    color: var(--ink);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .break {
    color: var(--ink);
    font-variant-numeric: tabular-nums;
  }
  .enable {
    height: 32px;
    padding: 0;
    border: none;
    background: none;
    color: var(--accent);
    font-size: 14px;
  }
</style>
