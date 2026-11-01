<script lang="ts">
  // 작은 타이머 창(항상 위). 시간 숫자만 보이고, 마우스를 올리면 아이콘이 나온다.
  // 접으면 상태색 칩 하나: 누르면 일시정지/재개, 두 번 누르면 다시 펼침.
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { hms } from "./lib/format";
  import { setLang, t } from "./lib/i18n.svelte";
  import { initTheme } from "./lib/theme.svelte";
  import type { MiniSettings, SettingsOut, View } from "./lib/types";

  let snap = $state<View | null>(null);
  let mini = $state<MiniSettings | null>(null);
  let hover = $state(false);

  document.documentElement.classList.add("mini");

  async function loadSettings() {
    const s = await invoke<SettingsOut>("get_settings");
    setLang(s.language);
    mini = s.mini;
  }

  onMount(() => {
    initTheme();
    loadSettings();
    invoke<View>("get_state").then((v) => (snap = v));
    const win = getCurrentWindow();
    let timer: ReturnType<typeof setTimeout> | undefined;
    const un = [
      listen<View>("tally://state", (e) => (snap = e.payload)),
      listen("tally://settings", loadSettings),
      // 옮긴 위치를 기억(끌기가 끝나고 0.5초 뒤 한 번)
      win.onMoved(({ payload }) => {
        clearTimeout(timer);
        timer = setTimeout(() => invoke("save_mini_pos", { x: payload.x, y: payload.y }), 500);
      }),
    ];
    return () => un.forEach((p) => p.then((f) => f()));
  });

  const update = (patch: Partial<MiniSettings>) => mini && invoke("set_mini", { mini: { ...mini, ...patch } });
  const order: MiniSettings["show"][] = ["work", "elapsed", "both"];
  const cycle = () => mini && update({ show: order[(order.indexOf(mini.show) + 1) % 3] });

  // 끌어서 옮기기: 6px 넘게 움직이면 창 끌기 시작(버튼 위에서는 안 함).
  // 너무 작으면 두 번 클릭하는 사이 손이 조금만 떨려도 끌기가 시작돼 클릭이 사라진다
  let down: { x: number; y: number } | null = null;
  let dragged = false;
  function onDown(e: PointerEvent) {
    dragged = false;
    if (e.button !== 0 || (e.target as HTMLElement).closest("button.icon")) return;
    down = { x: e.screenX, y: e.screenY };
  }
  function onMove(e: PointerEvent) {
    if (down && Math.hypot(e.screenX - down.x, e.screenY - down.y) > 6) {
      down = null;
      dragged = true;
      getCurrentWindow().startDragging();
    }
  }

  // 클릭 수를 직접 센다. 브라우저의 dblclick은 창 끌기·창 활성화 때문에 빠질 수 있어서 쓰지 않는다
  const DOUBLE_MS = 350;
  let lastClick = 0;
  let clickTimer: ReturnType<typeof setTimeout> | undefined;
  /** single: 한 번 눌렀을 때(두 번째 클릭이 안 오면), double: 두 번 눌렀을 때 */
  function clicks(single: (() => void) | null, double: () => void) {
    if (dragged) return; // 끌기가 끝난 뒤 따라오는 클릭은 무시
    const now = Date.now();
    clearTimeout(clickTimer);
    if (now - lastClick < DOUBLE_MS) {
      lastClick = 0;
      double();
      return;
    }
    lastClick = now;
    if (single) clickTimer = setTimeout(() => ((lastClick = 0), single()), DOUBLE_MS);
  }

  const expand = () => update({ collapsed: false });
  // 칩: 한 번 = 일시정지/재개, 두 번 = 펼치기
  const chipClick = () => clicks(() => invoke("toggle_pause"), expand);
  // 펼친 상태: 두 번 = 본창으로
  function boxClick(e: MouseEvent) {
    if ((e.target as HTMLElement).closest("button")) return;
    clicks(null, () => invoke("back_to_main"));
  }

  const working = $derived(snap?.kind === "work");
  const main = $derived(snap ? (mini?.show === "elapsed" ? snap.elapsedMs : snap.workMs) : 0);
</script>

{#if snap && mini}
  {#if mini.collapsed}
    <div
      class="chip-wrap"
      style="opacity: {mini.opacity}"
      role="presentation"
      onpointerenter={() => (hover = true)}
      onpointerleave={() => ((hover = false), (down = null))}
    >
      <button
        type="button"
        class="chip num"
        class:small={mini.size === "small"}
        class:working
        aria-label={working ? t("mini.chipWorking") : t("mini.chipStopped")}
        title={t("mini.chipHint")}
        onpointerdown={onDown}
        onpointermove={onMove}
        onpointerup={() => (down = null)}
        onclick={chipClick}
        oncontextmenu={(e) => (e.preventDefault(), expand())}>{hms(main)}</button
      >
      <!-- 두 번 클릭은 다른 창이 앞에 있을 때 첫 클릭이 창 활성화에 쓰여 빠질 수 있어서, 한 번에 펼치는 단추를 둔다 -->
      {#if hover}
        <button type="button" class="icon expand" aria-label={t("mini.expand")} title={t("mini.expand")} onclick={expand}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
            ><path d="M14 4h6v6M10 20H4v-6M20 4l-7 7M4 20l7-7" /></svg
          >
        </button>
      {/if}
    </div>
  {:else}
    <div
      class="box"
      class:small={mini.size === "small"}
      class:both={mini.show === "both"}
      style="opacity: {mini.opacity}"
      role="presentation"
      onpointerenter={() => (hover = true)}
      onpointerleave={() => ((hover = false), (down = null))}
      onpointerdown={onDown}
      onpointermove={onMove}
      onpointerup={() => (down = null)}
      onclick={boxClick}
    >
      {#if mini.show === "both"}
        <div class="line"><span class="lbl">{t("mini.work")}</span><span class="num t">{hms(snap.workMs)}</span></div>
        <div class="line"><span class="lbl">{t("today.elapsed")}</span><span class="num t sub">{hms(snap.elapsedMs)}</span></div>
      {:else}
        <span class="num single">{hms(main)}</span>
      {/if}

      {#if hover}
        <div class="icons">
          <button type="button" class="icon" class:on={mini.pinned} aria-label={t("mini.pin")} aria-pressed={mini.pinned} title={t("mini.pin")} onclick={() => update({ pinned: !mini?.pinned })}>
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
              ><path d="M9 4h6l-1 6 3 3H7l3-3z" /><path d="M12 13v7" /></svg
            >
          </button>
          <button type="button" class="icon" aria-label={t("mini.cycle")} title={t("mini.cycle")} onclick={cycle}>
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
              ><path d="M4 8h13l-3-3M20 16H7l3 3" /></svg
            >
          </button>
          <button type="button" class="icon" aria-label={t("mini.collapse")} title={t("mini.collapse")} onclick={() => update({ collapsed: true })}>
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"><path d="M6 15h12" /></svg>
          </button>
          <button type="button" class="icon" aria-label={t("mini.close")} title={t("mini.close")} onclick={() => invoke("close_mini")}>
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" aria-hidden="true"
              ><path d="M6 6l12 12M18 6L6 18" /></svg
            >
          </button>
        </div>
      {/if}
    </div>
  {/if}
{/if}

<style>
  :global(html.mini),
  :global(html.mini body) {
    background: transparent;
    overflow: hidden;
  }
  .box {
    position: relative;
    height: 100vh;
    box-sizing: border-box;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    padding: 0 12px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    cursor: default;
    overflow: hidden;
  }
  .single {
    font-size: 20px;
    font-weight: 500;
  }
  .line {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }
  .lbl {
    font-size: 13px;
    color: var(--sub);
  }
  .t {
    font-size: 18px;
    font-weight: 500;
  }
  .t.sub {
    color: var(--sub);
    font-weight: 400;
  }
  .box.small {
    padding: 0 9px;
  }
  .box.small .single {
    font-size: 16px;
  }
  .box.small .t {
    font-size: 14px;
  }
  .box.small .lbl {
    font-size: 11px;
  }
  .icons {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    padding: 0 2px 0 4px;
    background: var(--surface);
  }
  .icon {
    width: 24px;
    height: 24px;
    padding: 0;
    border: none;
    border-radius: 4px;
    background: none;
    color: var(--sub);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .icon:hover {
    color: var(--ink);
  }
  .icon.on {
    color: var(--accent);
  }
  .chip-wrap {
    position: relative;
    width: 100vw;
    height: 100vh;
  }
  .expand {
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    width: 24px;
    border-radius: 0 4px 4px 0;
    color: #ffffff;
    background: rgba(0, 0, 0, 0.22);
  }
  .expand:hover {
    color: #ffffff;
    background: rgba(0, 0, 0, 0.35);
  }
  /* 상태 전용 색(다른 화면에는 쓰지 않음) */
  .chip {
    width: 100vw;
    height: 100vh;
    border: none;
    border-radius: 4px;
    background: #b3392b;
    color: #ffffff;
    font-size: 16px;
    font-weight: 600;
    padding: 0;
    cursor: default;
  }
  .chip.working {
    background: #2f7a47;
  }
  .chip.small {
    font-size: 13px;
  }
</style>
