<script lang="ts">
  import { onMount } from "svelte";
  import AlarmTool from "./lib/AlarmTool.svelte";
  import { t, type Key } from "./lib/i18n.svelte";
  import StopwatchTool from "./lib/StopwatchTool.svelte";
  import TimerTool from "./lib/TimerTool.svelte";
  import type { View } from "./lib/types";

  let { snap }: { snap: View } = $props();

  type Tool = "timer" | "stopwatch" | "alarm";
  const tools: Tool[] = ["timer", "stopwatch", "alarm"];
  function savedTool(): Tool {
    try {
      const v = localStorage.getItem("tool");
      return tools.includes(v as Tool) ? (v as Tool) : "timer";
    } catch {
      return "timer";
    }
  }
  let tool = $state<Tool>(savedTool());

  function pick(v: Tool) {
    tool = v;
    try {
      localStorage.setItem("tool", v);
    } catch {}
  }

  // 0.1초마다 갱신하는 시계(타이머·스톱워치 숫자를 매끄럽게). 계산은 Rust가 한 값을 기준으로 한다.
  let now = $state(Date.now());
  onMount(() => {
    const id = setInterval(() => (now = Date.now()), 100);
    return () => clearInterval(id);
  });
</script>

<div class="tools">
  <div class="seg">
    {#each tools as v}
      <button type="button" class:on={tool === v} onclick={() => pick(v)}>{t(`tool.${v}` as Key)}</button>
    {/each}
  </div>

  {#if tool === "timer"}
    <TimerTool timer={snap.timer} {now} />
  {:else if tool === "stopwatch"}
    <StopwatchTool sw={snap.stopwatch} {now} />
  {:else}
    <AlarmTool />
  {/if}
</div>

<style>
  .tools {
    display: flex;
    flex-direction: column;
    flex: 1;
  }
  .seg {
    display: flex;
    border: 1px solid var(--line);
    border-radius: 6px;
    padding: 3px;
    gap: 3px;
  }
  .seg button {
    flex: 1;
    height: 36px;
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
</style>
