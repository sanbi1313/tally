<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { availableLangs, detectLang, LANG_NAMES, setLang, t, type Lang } from "./lib/i18n.svelte";
  import ProgramPicker from "./lib/ProgramPicker.svelte";

  let { ondone }: { ondone: () => void } = $props();

  // 1) 언어 고르기 → 2) "무엇으로 작업하세요?"
  let step = $state<"lang" | "apps">("lang");
  const initial = detectLang();
  let lang = $state<Lang>(initial);
  setLang(initial);

  let selected = $state<string[]>([]);
  let busy = $state(false);

  function pickLang(l: Lang) {
    lang = l;
    setLang(l); // 고르는 즉시 화면 문구도 바뀌게
  }

  async function next() {
    await invoke("set_language", { language: lang });
    step = "apps";
  }

  async function finish(exes: string[]) {
    busy = true;
    try {
      await invoke("finish_onboarding", { exes });
      ondone();
    } finally {
      busy = false;
    }
  }
</script>

<div class="onboard">
  {#if step === "lang"}
    <h1>{t("onboard.language")}</h1>
    <div class="langs" role="radiogroup" aria-label={t("general.language")}>
      {#each availableLangs as l}
        <button type="button" role="radio" aria-checked={lang === l} class:on={lang === l} {lang} onclick={() => pickLang(l)}>
          <span class="mark"></span>{LANG_NAMES[l]}
        </button>
      {/each}
    </div>
    <div class="actions">
      <button type="button" class="primary" onclick={next}>{t("onboard.next")}</button>
    </div>
  {:else}
    <h1>{t("onboard.title")}</h1>
    <div class="list">
      <ProgramPicker bind:selected />
    </div>
    <div class="actions">
      <button type="button" class="primary" disabled={busy} onclick={() => finish(selected)}>
        {selected.length ? t("onboard.startN", { n: selected.length }) : t("onboard.start")}
      </button>
      <button type="button" class="text" disabled={busy} onclick={() => finish([])}>{t("onboard.later")}</button>
    </div>
    <div class="note">{t("onboard.note")}</div>
  {/if}
</div>

<style>
  .onboard {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
  }
  h1 {
    margin: 0;
    font-size: 22px;
    font-weight: 600;
  }
  .langs {
    display: flex;
    flex-direction: column;
    margin-top: 14px;
    flex: 1;
  }
  .langs button {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 48px;
    padding: 0;
    border: none;
    border-bottom: 1px solid var(--line);
    background: none;
    color: var(--ink);
    font-size: 15px;
    text-align: left;
  }
  .mark {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 1.5px solid var(--sub);
    box-sizing: border-box;
  }
  .langs button.on {
    font-weight: 600;
  }
  .langs button.on .mark {
    border: 5px solid var(--accent);
  }
  .list {
    margin-top: 14px;
    flex: 1;
    overflow-y: auto;
    min-height: 0;
  }
  .actions {
    display: flex;
    gap: 4px;
    margin-top: 16px;
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
  .text {
    height: 44px;
    padding: 0 12px;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 15px;
  }
  .note {
    margin-top: 6px;
    font-size: 13px;
    color: var(--sub);
  }
</style>
