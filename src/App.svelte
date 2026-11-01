<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import Onboarding from "./Onboarding.svelte";
  import Settings from "./Settings.svelte";
  import History from "./History.svelte";
  import Today from "./Today.svelte";
  import Tools from "./Tools.svelte";
  import { setLang, t, type Key } from "./lib/i18n.svelte";
  import { initTheme, resolved, setTheme } from "./lib/theme.svelte";
  import type { SettingsOut, View } from "./lib/types";

  // 헤더 버튼: 지금 보이는 테마의 반대로 고정
  const toggleTheme = () => setTheme(resolved() === "dark" ? "light" : "dark");

  const tabs = ["today", "tools", "log", "settings"] as const;
  let tab = $state<(typeof tabs)[number]>("today");

  let snap = $state<View | null>(null);
  let askReset = $state(false);
  // 첫 실행이면 "무엇으로 작업하세요?" 화면부터
  let onboarded = $state<boolean | null>(null);
  const ready = new Set(["today", "tools", "log", "settings"]);

  onMount(() => {
    initTheme();
    invoke<View>("get_state").then((s) => (snap = s));
    invoke<boolean>("take_ask_reset").then((v) => (askReset = v));
    invoke<SettingsOut>("get_settings").then((s) => {
      setLang(s.language);
      return invoke<boolean>("get_onboarded").then((v) => (onboarded = v));
    });
    const un = [
      listen<View>("tally://state", (e) => (snap = e.payload)),
      listen("tally://ask-reset", () => {
        tab = "today";
        askReset = true;
      }),
    ];
    return () => {
      un.forEach((p) => p.then((f) => f()));
    };
  });
</script>

<div class="shell">
  <header data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <svg width="20" height="20" viewBox="0 0 48 48" aria-hidden="true"
        ><g stroke-linecap="round" stroke-width="4.5" fill="none"
          ><path d="M12 11V37" stroke="var(--ink)" /><path d="M19.5 9V38" stroke="var(--ink)" /><path
            d="M27 12V37"
            stroke="var(--ink)"
          /><path d="M34.5 10V36" stroke="var(--ink)" /><path d="M6 33L41 15" stroke="var(--accent)" /></g
        ></svg
      >
      <span data-tauri-drag-region>Tally</span>
    </div>
    <div class="actions">
      <button type="button" class="icon" aria-label={t("theme.toggle")} onclick={toggleTheme}>
        {#if resolved() === "dark"}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"
            ><circle cx="12" cy="12" r="4" /><path d="M12 3v2M12 19v2M3 12h2M19 12h2M5.6 5.6l1.4 1.4M17 17l1.4 1.4M5.6 18.4L7 17M17 7l1.4-1.4" /></svg
          >
        {:else}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
            ><path d="M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5z" /></svg
          >
        {/if}
      </button>
      <button type="button" class="icon" aria-label={t("mini.fold")} title={t("mini.fold")} onclick={() => invoke("fold_to_mini")}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"
          ><rect x="4" y="14" width="16" height="5" rx="1.5" /><path d="M12 4v6M9 7.5l3 2.5 3-2.5" /></svg
        >
      </button>
      <button type="button" class="icon" aria-label={t("window.close")} onclick={() => invoke("close_to_tray")}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"
          ><path d="M6 6l12 12M18 6L6 18" /></svg
        >
      </button>
    </div>
  </header>

  {#if onboarded === false}
    <main class="solo">
      <Onboarding ondone={() => (onboarded = true)} />
    </main>
  {:else if onboarded}
    <nav>
      {#each tabs as id}
        <button type="button" class:on={tab === id} disabled={!ready.has(id)} onclick={() => (tab = id)}>{t(`tab.${id}` as Key)}</button>
      {/each}
    </nav>

    {#key tab}
      <main>
        {#if tab === "today" && snap}
          <Today {snap} bind:askReset />
        {:else if tab === "tools" && snap}
          <Tools {snap} />
        {:else if tab === "log" && snap}
          <History today={snap.day} />
        {:else if tab === "settings"}
          <Settings />
        {/if}
      </main>
    {/key}
  {/if}
</div>

<style>
  .shell {
    height: 100%;
    display: flex;
    flex-direction: column;
  }
  header {
    height: 44px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 6px 0 16px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 15px;
    font-weight: 600;
  }
  .actions {
    display: flex;
    align-items: center;
  }
  .icon {
    width: 40px;
    height: 40px;
    border: none;
    background: none;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--sub);
    border-radius: 6px;
  }
  .icon:hover {
    color: var(--ink);
  }
  nav {
    display: flex;
    gap: 22px;
    padding: 0 16px;
    border-bottom: 1px solid var(--line);
    flex-shrink: 0;
  }
  nav button {
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    padding: 10px 0 9px;
    font-size: 15px;
    font-weight: 400;
    color: var(--sub);
  }
  nav button.on {
    border-bottom-color: var(--accent);
    color: var(--ink);
    font-weight: 600;
  }
  nav button:disabled {
    color: var(--idle);
    cursor: default;
  }
  main {
    flex: 1;
    overflow-y: auto;
    padding: 22px 20px 20px;
    display: flex;
    flex-direction: column;
    animation: fade 120ms ease-out;
  }
  main.solo {
    border-top: 1px solid var(--line);
  }
  @keyframes fade {
    from {
      opacity: 0;
    }
  }
</style>
