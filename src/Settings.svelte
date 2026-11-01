<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import AlertSettings from "./lib/AlertSettings.svelte";
  import GeneralSettings from "./lib/GeneralSettings.svelte";
  import ProgramPicker from "./lib/ProgramPicker.svelte";
  import { t } from "./lib/i18n.svelte";
  import { COLOR_TOKENS, colorVar, type AppRow } from "./lib/types";

  let apps = $state<AppRow[]>([]);
  let editing = $state<number | null>(null);
  let adding = $state(false);
  let error = $state("");

  async function load() {
    apps = await invoke<AppRow[]>("list_apps");
  }
  onMount(load);

  async function save(a: AppRow) {
    error = "";
    try {
      await invoke("update_app", { app: a });
    } catch (e) {
      error = String(e);
    }
    await load();
  }

  async function add(exe: string) {
    error = "";
    try {
      await invoke("add_app", { exe });
    } catch (e) {
      error = String(e);
    }
    await load();
  }

  async function remove(id: number) {
    await invoke("delete_app", { id });
    editing = null;
    await load();
  }

  function onEnter(e: KeyboardEvent) {
    if (e.key === "Enter") (e.currentTarget as HTMLInputElement).blur();
  }
</script>

<div class="settings">
  <div class="label">{t("settings.tracked")}</div>
  <div class="list">
    {#each apps as a (a.id)}
      <div class="item">
        <div class="row">
          <input
            type="checkbox"
            checked={a.enabled}
            aria-label={t("settings.trackToggle", { label: a.label })}
            onchange={(e) => save({ ...a, enabled: e.currentTarget.checked })}
          />
          <span class="swatch" style="background: {colorVar(a.color)}"></span>
          <button type="button" class="name" onclick={() => (editing = editing === a.id ? null : a.id)}>
            {a.label}
          </button>
          <span class="exe">{a.exe}{a.titleKw ? ` · ${a.titleKw}` : ""}</span>
        </div>
        {#if editing === a.id}
          <div class="edit">
            <label>
              <span>{t("settings.name")}</span>
              <input type="text" value={a.label} onkeydown={onEnter} onchange={(e) => save({ ...a, label: e.currentTarget.value })} />
            </label>
            <label>
              <span>{t("settings.titleKw")}</span>
              <input
                type="text"
                value={a.titleKw ?? ""}
                placeholder={t("settings.titleKwPlaceholder")}
                onkeydown={onEnter}
                onchange={(e) => save({ ...a, titleKw: e.currentTarget.value })}
                spellcheck="false"
              />
            </label>
            <div class="colors">
              <span>{t("settings.color")}</span>
              {#each COLOR_TOKENS as c}
                <button
                  type="button"
                  class="color"
                  class:on={a.color === c}
                  style="background: {colorVar(c)}"
                  aria-label="{t('settings.color')} {c}"
                  aria-pressed={a.color === c}
                  onclick={() => save({ ...a, color: c })}
                ></button>
              {/each}
              <button type="button" class="danger" onclick={() => remove(a.id)}>{t("settings.delete")}</button>
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <div class="muted">{t("settings.noTracked")}</div>
    {/each}
  </div>
  {#if error}<div class="error">{error}</div>{/if}

  {#if adding}
    <div class="adding">
      <div class="label">{t("settings.running")}</div>
      <ProgramPicker exclude={apps.map((a) => a.exe)} onpick={add} />
      <button type="button" class="text" onclick={() => (adding = false)}>{t("settings.close")}</button>
    </div>
  {:else}
    <button type="button" class="link" onclick={() => (adding = true)}>{t("settings.addRunning")}</button>
  {/if}

  <div class="section"><AlertSettings /></div>
  <div class="section"><GeneralSettings /></div>
</div>

<style>
  .settings {
    display: flex;
    flex-direction: column;
  }
  .section {
    margin-top: 18px;
    display: flex;
    flex-direction: column;
  }
  .label {
    font-size: 13px;
    color: var(--sub);
  }
  .list {
    display: flex;
    flex-direction: column;
    margin-top: 6px;
  }
  .item {
    border-bottom: 1px solid var(--line);
  }
  .row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 0;
    font-size: 15px;
    min-width: 0;
  }
  input[type="checkbox"] {
    width: 18px;
    height: 18px;
    margin: 0;
    accent-color: var(--accent);
    flex-shrink: 0;
  }
  .swatch {
    width: 4px;
    height: 16px;
    border-radius: 2px;
    flex-shrink: 0;
    margin-left: -4px;
  }
  .name {
    flex: 1;
    min-width: 0;
    text-align: left;
    height: 32px;
    padding: 0;
    border: none;
    background: none;
    color: var(--ink);
    font-size: 15px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .exe {
    font-size: 13px;
    color: var(--sub);
    max-width: 50%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .edit {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 4px 0 12px 30px;
  }
  .edit label,
  .colors {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
    color: var(--sub);
  }
  .edit label > span,
  .colors > span {
    width: 44px;
    flex-shrink: 0;
  }
  .edit input {
    flex: 1;
    min-width: 0;
    height: 32px;
    box-sizing: border-box;
    padding: 0 10px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 14px;
  }
  .edit input:focus {
    outline: none;
    border-color: var(--ink);
  }
  .color {
    width: 22px;
    height: 22px;
    padding: 0;
    border-radius: 6px;
    border: 2px solid transparent;
    box-sizing: border-box;
  }
  .color.on {
    outline: 1.5px solid var(--ink);
    outline-offset: 2px;
  }
  .danger {
    margin-left: auto;
    height: 32px;
    padding: 0;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 14px;
  }
  .danger:hover {
    color: var(--accent);
  }
  .muted {
    padding: 10px 0;
    font-size: 13px;
    color: var(--sub);
    border-bottom: 1px solid var(--line);
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
  .adding {
    margin-top: 18px;
    display: flex;
    flex-direction: column;
  }
  .text {
    align-self: flex-start;
    margin-top: 6px;
    height: 36px;
    padding: 0;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 14px;
  }
</style>
