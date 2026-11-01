<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { t, type Key } from "./i18n.svelte";
  import SoundSelect from "./SoundSelect.svelte";
  import type { AlarmRow } from "./types";

  let alarms = $state<AlarmRow[]>([]);
  let editing = $state<number | null>(null);
  let error = $state("");

  async function load() {
    alarms = await invoke<AlarmRow[]>("list_alarms");
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

  // 화면부터 바꿔 두고 저장한다(빠르게 여러 번 눌러도 최신 값에 이어서 바뀌게)
  function save(a: AlarmRow) {
    alarms = alarms.map((x) => (x.id === a.id ? a : x));
    return run(() => invoke("update_alarm", { alarm: a }));
  }

  const current = (id: number) => alarms.find((x) => x.id === id)!;

  async function add() {
    error = "";
    try {
      const a = await invoke<AlarmRow>("add_alarm", { timeHm: "07:00", weekdays: 127, label: null });
      editing = a.id;
    } catch (e) {
      error = String(e);
    }
    await load();
  }

  const DAYS = [0, 1, 2, 3, 4, 5, 6];
  const dayName = (i: number) => t(`day.${i}` as Key);

  /** 127 → 매일, 31 → 평일, 96 → 주말, 0 → 한 번, 나머지는 요일 나열 */
  function repeatText(w: number): string {
    if (w === 127) return t("alarm.daily");
    if (w === 31) return t("alarm.weekdays");
    if (w === 96) return t("alarm.weekend");
    if (w === 0) return t("alarm.once");
    return DAYS.filter((i) => w & (1 << i))
      .map(dayName)
      .join("·");
  }

  function toggleDay(id: number, i: number) {
    const a = current(id);
    save({ ...a, weekdays: a.weekdays ^ (1 << i) });
  }
</script>

<div class="alarms">
  {#each alarms as a (a.id)}
    <div class="item">
      <div class="row">
        <button type="button" class="summary" onclick={() => (editing = editing === a.id ? null : a.id)}>
          <span class="time num" class:off={!a.enabled}>{a.timeHm}</span>
          <span class="desc">{repeatText(a.weekdays)}{a.label ? ` · ${a.label}` : ""}</span>
        </button>
        <button
          type="button"
          class="switch"
          class:on={a.enabled}
          aria-label={t("alarm.toggle", { time: a.timeHm })}
          aria-pressed={a.enabled}
          onclick={() => save({ ...a, enabled: !a.enabled })}
        >
          <span></span>
        </button>
      </div>

      {#if editing === a.id}
        <div class="edit">
          <label class="line">
            <span class="key">{t("alarm.time")}</span>
            <input type="time" class="num" value={a.timeHm} onchange={(e) => e.currentTarget.value && save({ ...a, timeHm: e.currentTarget.value })} />
          </label>
          <div class="line">
            <span class="key">{t("alarm.repeat")}</span>
            <div class="days">
              {#each DAYS as i}
                <button type="button" class="day" class:on={(a.weekdays & (1 << i)) !== 0} aria-pressed={(a.weekdays & (1 << i)) !== 0} onclick={() => toggleDay(a.id, i)}>
                  {dayName(i)}
                </button>
              {/each}
            </div>
          </div>
          <label class="line">
            <span class="key">{t("alarm.label")}</span>
            <input
              type="text"
              value={a.label ?? ""}
              placeholder={t("alarm.labelPlaceholder")}
              maxlength="40"
              onchange={(e) => save({ ...a, label: e.currentTarget.value })}
            />
          </label>
          <div class="line">
            <span class="key">{t("alarm.sound")}</span>
            <SoundSelect value={a.sound} fallback="builtin:bell" label={t("alarm.sound")} onchange={(v) => run(() => invoke("set_alarm_sound", { id: a.id, value: v }))} />
            <button type="button" class="text" onclick={() => invoke("preview_sound", { sound: a.sound ?? "builtin:bell", volume: 0.9 })}>{t("alert.preview")}</button>
          </div>
          <div class="line tail">
            <button type="button" class="text danger" onclick={() => ((editing = null), run(() => invoke("delete_alarm", { id: a.id })))}>{t("alarm.delete")}</button>
            <button type="button" class="text" onclick={() => (editing = null)}>{t("alarm.done")}</button>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="muted">{t("alarm.empty")}</div>
  {/each}
  {#if error}<div class="error">{error}</div>{/if}
  <button type="button" class="link" onclick={add}>{t("alarm.add")}</button>
</div>

<style>
  .alarms {
    display: flex;
    flex-direction: column;
    margin-top: 24px;
  }
  .item {
    border-bottom: 1px solid var(--line);
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 0;
    gap: 12px;
  }
  .summary {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 0;
    border: none;
    background: none;
    text-align: left;
    color: var(--ink);
  }
  .time {
    font-size: 26px;
    font-weight: 500;
  }
  .time.off {
    color: var(--sub);
  }
  .desc {
    font-size: 13px;
    color: var(--sub);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
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
  .edit {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 0 0 12px;
  }
  .line {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }
  .key {
    width: 56px;
    flex-shrink: 0;
    font-size: 13px;
    color: var(--sub);
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
  .edit input[type="time"] {
    flex: 0 0 auto;
    width: 120px;
  }
  .edit input:focus {
    outline: none;
    border-color: var(--ink);
  }
  .days {
    display: flex;
    gap: 4px;
    flex: 1;
  }
  .day {
    flex: 1;
    min-width: 0;
    height: 32px;
    padding: 0;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: none;
    color: var(--sub);
    font-size: 13px;
  }
  .day.on {
    border-color: var(--ink);
    color: var(--ink);
    font-weight: 600;
  }
  .tail {
    justify-content: flex-end;
    gap: 14px;
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
  .muted {
    padding: 14px 0;
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
    margin-top: 12px;
    height: 40px;
    padding: 0;
    border: none;
    background: none;
    color: var(--accent);
    font-size: 15px;
  }
</style>
