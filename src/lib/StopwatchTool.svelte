<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { stopwatchParts } from "./format";
  import { t } from "./i18n.svelte";
  import type { StopwatchView } from "./types";

  let { sw, now }: { sw: StopwatchView; now: number } = $props();

  const running = $derived(sw.sinceMs !== null);
  const elapsed = $derived(sw.accMs + (sw.sinceMs !== null ? Math.max(0, now - sw.sinceMs) : 0));
  const parts = $derived(stopwatchParts(elapsed));
  // 최근 랩이 위로
  const laps = $derived(sw.laps.map((ms, i) => ({ n: i + 1, ms })).reverse());
</script>

<div class="sw">
  <div class="label">{t("tool.stopwatch")}</div>
  <div class="big num">{parts[0]}<span class="tenth">{parts[1]}</span></div>

  <div class="row">
    {#if running}
      <button type="button" class="primary" onclick={() => invoke("sw_stop")}>{t("sw.stop")}</button>
      <button type="button" class="outline" onclick={() => invoke("sw_lap")}>{t("sw.lap")}</button>
    {:else}
      <button type="button" class="primary" onclick={() => invoke("sw_start")}>{t("sw.start")}</button>
      {#if elapsed > 0}
        <button type="button" class="text" onclick={() => invoke("sw_reset")}>{t("sw.reset")}</button>
      {/if}
    {/if}
  </div>

  {#if laps.length}
    <div class="laps">
      {#each laps as l (l.n)}
        {@const p = stopwatchParts(l.ms)}
        <div class="lap"><span class="lap-n">{t("sw.lapN", { n: l.n })}</span><span class="num">{p[0]}{p[1]}</span></div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .sw {
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
  .tenth {
    font-size: 32px;
    color: var(--sub);
  }
  .row {
    display: flex;
    gap: 8px;
    margin-top: 20px;
    align-items: center;
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
  .outline {
    height: 44px;
    padding: 0 18px;
    border-radius: 6px;
    border: 1px solid var(--line);
    background: none;
    color: var(--ink);
    font-size: 15px;
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
  .laps {
    margin-top: 28px;
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid var(--line);
  }
  .lap {
    display: flex;
    justify-content: space-between;
    padding: 10px 0;
    border-top: 1px solid var(--line);
    font-size: 15px;
  }
  .lap-n {
    color: var(--sub);
  }
</style>
