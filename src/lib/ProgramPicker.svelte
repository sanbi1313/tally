<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { t } from "./i18n.svelte";
  import type { RunningProgram } from "./types";

  // 실행 중인 프로그램을 아이콘과 함께 나열한다. 체크(multi) 또는 누르면 바로 추가(pick).
  let {
    exclude = [],
    selected = $bindable([]),
    onpick,
  }: {
    exclude?: string[];
    selected?: string[];
    onpick?: (exe: string) => void;
  } = $props();

  let list = $state<RunningProgram[] | null>(null);
  let manual = $state("");
  const lower = (s: string) => s.toLowerCase();
  const shown = $derived((list ?? []).filter((p) => !exclude.some((e) => lower(e) === lower(p.exe))));

  async function load() {
    list = null;
    list = await invoke<RunningProgram[]>("running_programs");
  }
  onMount(load);

  function toggle(exe: string) {
    if (onpick) return onpick(exe);
    selected = selected.includes(exe) ? selected.filter((e) => e !== exe) : [...selected, exe];
  }

  function addManual(e: SubmitEvent) {
    e.preventDefault();
    let exe = manual.trim();
    if (!exe) return;
    if (!lower(exe).endsWith(".exe")) exe += ".exe";
    manual = "";
    if (onpick) return onpick(exe);
    if (!list?.some((p) => lower(p.exe) === lower(exe))) {
      list = [...(list ?? []), { exe, label: exe.replace(/\.exe$/i, ""), title: "", icon: null }];
    }
    if (!selected.includes(exe)) selected = [...selected, exe];
  }
</script>

<div class="picker">
  {#if list === null}
    <div class="muted">{t("picker.loading")}</div>
  {:else}
    {#each shown as p (p.exe)}
      <label class="row" title={p.title}>
        {#if !onpick}
          <input type="checkbox" checked={selected.includes(p.exe)} onchange={() => toggle(p.exe)} />
        {/if}
        {#if p.icon}<img src={p.icon} alt="" width="20" height="20" />{:else}<span class="noicon"></span>{/if}
        <span class="name">{p.label}</span>
        <span class="exe">{p.exe}</span>
        {#if onpick}<button type="button" class="add" onclick={() => toggle(p.exe)}>{t("picker.add")}</button>{/if}
      </label>
    {:else}
      <div class="muted">{t("picker.empty")}</div>
    {/each}
  {/if}
  <form class="manual" onsubmit={addManual}>
    <input type="text" placeholder={t("picker.manual")} bind:value={manual} spellcheck="false" />
    <button type="submit" class="add" disabled={!manual.trim()}>{t("picker.add")}</button>
    <button type="button" class="add" onclick={load} >{t("picker.refresh")}</button>
  </form>
</div>

<style>
  .picker {
    display: flex;
    flex-direction: column;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 0;
    border-bottom: 1px solid var(--line);
    font-size: 15px;
    cursor: pointer;
    min-width: 0;
  }
  input[type="checkbox"] {
    width: 18px;
    height: 18px;
    margin: 0;
    accent-color: var(--accent);
    flex-shrink: 0;
  }
  img,
  .noicon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
  }
  .noicon {
    border: 1px solid var(--line);
    border-radius: 4px;
    box-sizing: border-box;
  }
  .name {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .exe {
    font-size: 13px;
    color: var(--sub);
    max-width: 45%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .muted {
    padding: 10px 0;
    font-size: 13px;
    color: var(--sub);
  }
  .manual {
    display: flex;
    gap: 4px;
    margin-top: 10px;
    align-items: center;
  }
  .manual input {
    flex: 1;
    min-width: 0;
    height: 34px;
    box-sizing: border-box;
    padding: 0 10px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 14px;
  }
  .manual input:focus {
    outline: none;
    border-color: var(--ink);
  }
  .add {
    height: 34px;
    padding: 0 8px;
    border: none;
    background: none;
    color: var(--accent);
    font-size: 14px;
    flex-shrink: 0;
    border-radius: 6px;
  }
  .add:disabled {
    color: var(--idle);
    cursor: default;
  }
  .manual .add:last-child {
    color: var(--sub);
  }
</style>
