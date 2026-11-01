<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { save } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import * as D from "./lib/dates";
  import { focus, hms } from "./lib/format";
  import { getLang, minutes, t, type Key } from "./lib/i18n.svelte";
  import { colorVar, type DayDetail, type DayTotal, type SettingsOut } from "./lib/types";

  /** today: 엔진이 정한 오늘(새벽 5시 경계 적용) */
  let { today }: { today: string } = $props();

  type ViewKind = "day" | "week" | "month" | "year";
  const VIEWS: ViewKind[] = ["day", "week", "month", "year"];

  function saved(): ViewKind {
    try {
      const v = localStorage.getItem("logView");
      return VIEWS.includes(v as ViewKind) ? (v as ViewKind) : "week";
    } catch {
      return "week";
    }
  }
  let view = $state<ViewKind>(saved());
  // 초기값만 쓰고 이후엔 사용자가 옮긴다
  // svelte-ignore state_referenced_locally
  let ref = $state<string>(today);
  let startHour = $state(5);

  let totals = $state<DayTotal[]>([]);
  let detail = $state<DayDetail | null>(null);
  let message = $state("");
  let editingNote = $state<number | null>(null);
  // 삭제 확인: 세션 id 또는 "day"
  let confirmDel = $state<number | "day" | null>(null);
  let dateInput = $state<HTMLInputElement | null>(null);

  function setView(v: ViewKind, r = ref) {
    view = v;
    ref = r;
    try {
      localStorage.setItem("logView", v);
    } catch {}
  }

  // 지금 보는 기간
  const range = $derived.by((): [string, string] => {
    switch (view) {
      case "day":
        return [ref, ref];
      case "week":
        return [D.weekStart(ref), D.addDays(D.weekStart(ref), 6)];
      case "month":
        return [D.monthStart(ref), D.monthEnd(ref)];
      case "year":
        return [D.yearStart(ref), D.yearEnd(ref)];
    }
  });

  async function load() {
    const [from, to] = range;
    const v = view;
    const [t1, d1] = await Promise.all([
      invoke<DayTotal[]>("history_days", { from, to }),
      v === "day" ? invoke<DayDetail>("history_day", { day: from }) : Promise.resolve(null),
    ]);
    totals = t1;
    detail = d1;
  }

  $effect(() => {
    void range;
    load();
  });

  onMount(() => {
    invoke<SettingsOut>("get_settings").then((s) => (startHour = s.dayStartHour));
    // 오늘이 들어간 기간을 보고 있으면 숫자가 계속 늘어나니 30초마다 다시 읽는다
    const id = setInterval(() => {
      if (range[1] >= today && range[0] <= today) load();
    }, 30_000);
    return () => clearInterval(id);
  });

  function shift(dir: -1 | 1) {
    if (view === "day") ref = D.addDays(ref, dir);
    else if (view === "week") ref = D.addDays(ref, dir * 7);
    else if (view === "month") ref = D.addMonths(ref, dir);
    else ref = D.addMonths(ref, dir * 12);
    if (ref > today) ref = today;
  }
  const canNext = $derived(range[1] < today);

  const lang = $derived(getLang());
  const byDay = $derived(new Map(totals.map((d) => [d.day, d])));
  const sumWork = $derived(totals.reduce((a, d) => a + d.workMs, 0));
  const sumElapsed = $derived(totals.reduce((a, d) => a + d.elapsedMs, 0));
  const dur = (ms: number) => minutes(Math.round(ms / 60_000));
  const hours = (ms: number) => (ms / 3_600_000).toFixed(1);

  const label = $derived.by(() => {
    const [a, b] = range;
    if (view === "day") return D.fmt(lang, a, { year: "numeric", month: "long", day: "numeric", weekday: "short" });
    if (view === "week") return D.fmtRange(lang, a, b, { month: "long", day: "numeric" });
    if (view === "month") return D.fmt(lang, a, { year: "numeric", month: "long" });
    return D.fmt(lang, a, { year: "numeric" });
  });

  /** 기간 안에서 오늘까지 지난 날 수(평균용) */
  const daysSoFar = $derived.by(() => {
    const [a, b] = range;
    const end = b < today ? b : today;
    return Math.max(1, Math.round((D.toDate(end).getTime() - D.toDate(a).getTime()) / 864e5) + 1);
  });

  // ---- 주 ----
  const week = $derived.by(() => {
    if (view !== "week") return [];
    const start = range[0];
    const days = Array.from({ length: 7 }, (_, i) => D.addDays(start, i));
    const max = Math.max(3_600_000, ...days.map((d) => byDay.get(d)?.workMs ?? 0));
    return days.map((d) => {
      const ms = byDay.get(d)?.workMs ?? 0;
      return { day: d, ms, pct: (ms / max) * 100, name: D.fmt(lang, d, { weekday: "short" }), isToday: d === today, future: d > today };
    });
  });

  // ---- 월(달력) ----
  const month = $derived.by(() => {
    if (view !== "month") return { cells: [], worked: 0, heads: [] as string[] };
    const first = range[0];
    const lead = (D.toDate(first).getDay() + 6) % 7;
    const n = Number(range[1].slice(8));
    const days = Array.from({ length: n }, (_, i) => D.addDays(first, i));
    // 진하기 기준: 그달 최대치(적어도 4시간). 몇 초만 한 날이 가장 진하게 보이지 않게
    const max = Math.max(4 * 3_600_000, ...days.map((d) => byDay.get(d)?.workMs ?? 0));
    const cells = [
      ...Array.from({ length: lead }, () => null),
      ...days.map((d) => {
        const ms = byDay.get(d)?.workMs ?? 0;
        const r = ms / max;
        const level = ms === 0 ? 0 : r <= 0.25 ? 1 : r <= 0.5 ? 2 : r <= 0.75 ? 3 : 4;
        return { day: d, n: Number(d.slice(8)), ms, level, future: d > today, isToday: d === today };
      }),
    ];
    const monday = D.weekStart(first);
    const heads = Array.from({ length: 7 }, (_, i) => D.fmt(lang, D.addDays(monday, i), { weekday: "narrow" }));
    return { cells, worked: days.filter((d) => (byDay.get(d)?.workMs ?? 0) > 0).length, heads };
  });

  // ---- 연 ----
  const year = $derived.by(() => {
    if (view !== "year") return [];
    const y = range[0].slice(0, 4);
    const months = Array.from({ length: 12 }, (_, i) => {
      const key = `${y}-${String(i + 1).padStart(2, "0")}`;
      const ms = totals.filter((d) => d.day.startsWith(key)).reduce((a, d) => a + d.workMs, 0);
      return { first: `${key}-01`, n: i + 1, ms, future: `${key}-01` > today };
    });
    const max = Math.max(3_600_000, ...months.map((m) => m.ms));
    return months.map((m) => ({ ...m, pct: (m.ms / max) * 100, best: m.ms > 0 && m.ms === max }));
  });
  const best = $derived(year.find((m) => m.best));
  const monthsSoFar = $derived(Math.max(1, year.filter((m) => !m.future).length));

  // ---- 일(타임라인) ----
  const timeline = $derived.by(() => {
    if (view !== "day" || !detail) return [];
    const start = D.dayStartMs(ref, startHour);
    const span = 24 * 3_600_000;
    const color = (s: { kind: string; color: string | null }) =>
      s.kind === "work" ? (s.color ? colorVar(s.color) : "var(--accent)") : s.kind === "idle" ? "var(--idle)" : "var(--idle-soft)";
    // 같은 색이 이어지면 하나로 합쳐 그린다
    const out: { left: number; width: number; bg: string; title: string }[] = [];
    let last: { end: number; bg: string } | null = null;
    for (const s of detail.segments) {
      if (s.kind === "paused") continue;
      const bg = color(s);
      const title = s.kind === "work" ? (s.label ?? "") : t(s.kind === "idle" ? "status.idle" : s.kind === "break" ? "status.break" : "status.other");
      if (last && last.bg === bg && s.startMs - last.end < 2000) {
        out[out.length - 1].width = ((s.endMs - start) / span) * 100 - out[out.length - 1].left;
      } else {
        out.push({ left: ((s.startMs - start) / span) * 100, width: ((s.endMs - s.startMs) / span) * 100, bg, title });
      }
      last = { end: s.endMs, bg };
    }
    return out;
  });
  const axis = $derived([0, 6, 12, 18, 24].map((h) => String((startHour + h) % 24).padStart(2, "0")));

  async function remove(target: number | "day") {
    try {
      if (target === "day") await invoke("delete_day", { day: ref });
      else await invoke("delete_session", { id: target });
      message = t("log.deleted");
    } catch (e) {
      message = String(e);
    }
    confirmDel = null;
    load();
  }

  async function saveNote(id: number, value: string) {
    await invoke("set_session_note", { id, note: value });
    editingNote = null;
    load();
  }

  async function exportCsv() {
    const [from, to] = range;
    message = "";
    const path = await save({
      defaultPath: from === to ? `tally-${from}.csv` : `tally-${from}_${to}.csv`,
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (!path) return;
    try {
      const n = await invoke<number>("export_csv", { path, from, to });
      message = t("log.exported", { n });
    } catch (e) {
      message = String(e);
    }
  }

  function pickDate(v: string) {
    if (!v) return;
    ref = v > today ? today : v;
  }

  function autofocus(el: HTMLInputElement) {
    el.focus();
    el.select();
  }
</script>

<div class="log">
  <div class="seg">
    {#each VIEWS as v}
      <button type="button" class:on={view === v} onclick={() => setView(v, today)}>{t(`log.${v}` as Key)}</button>
    {/each}
  </div>

  <div class="period">
    <button type="button" class="nav" aria-label={t("log.prev")} onclick={() => shift(-1)}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
        ><path d="M15 5l-7 7 7 7" /></svg
      >
    </button>
    <button type="button" class="period-label" aria-label={t("log.pickDate")} onclick={() => dateInput?.showPicker()}>{label}</button>
    <input bind:this={dateInput} type="date" class="hidden-date" max={today} value={ref} tabindex="-1" aria-hidden="true" onchange={(e) => pickDate(e.currentTarget.value)} />
    <button type="button" class="nav" aria-label={t("log.next")} disabled={!canNext} onclick={() => shift(1)}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
        ><path d="M9 5l7 7-7 7" /></svg
      >
    </button>
  </div>

  <div class="total">
    <span class="num big">{dur(sumWork)}</span>
    <span class="sub">
      {#if view === "day"}
        {t("log.daySub", { elapsed: dur(sumElapsed), focus: focus(sumWork, sumElapsed) })}
      {:else if view === "week"}
        {t("log.weekSub", { avg: dur(sumWork / daysSoFar) })}
      {:else if view === "month"}
        {t("log.monthSub", { n: month.worked })}
      {:else}
        {t("log.yearSub", { avg: dur(sumWork / monthsSoFar) })}
      {/if}
    </span>
  </div>

  {#if view === "day"}
    <div class="timeline">
      {#each timeline as s}
        <div class="tl-seg" style="left: {s.left}%; width: {Math.max(s.width, 0.15)}%; background: {s.bg}" title={s.title}></div>
      {/each}
    </div>
    <div class="axis num">{#each axis as a}<span>{a}</span>{/each}</div>
    <div class="legend">
      <span><i style="background: var(--accent)"></i>{t("today.work")}</span>
      <span><i style="background: var(--idle-soft)"></i>{t("status.other")}</span>
      <span><i style="background: var(--idle)"></i>{t("status.idle")}</span>
    </div>

    <div class="label" style="margin-top: 18px">{t("log.sessions")}</div>
    {#each detail?.sessions ?? [] as s (s.id)}
      <div class="session">
        <div class="session-head">
          {#if editingNote === s.id}
            <input
              type="text"
              class="note-input"
              value={s.note ?? ""}
              maxlength="80"
              placeholder={t("today.memoPlaceholder")}
              use:autofocus
              onkeydown={(e) => {
                if (e.key === "Enter") saveNote(s.id, e.currentTarget.value);
                if (e.key === "Escape") editingNote = null;
              }}
              onblur={(e) => saveNote(s.id, e.currentTarget.value)}
            />
          {:else}
            <button type="button" class="memo" class:empty={!s.note} title={t("log.editMemo")} onclick={() => (editingNote = s.id)}>
              {s.note ?? t("log.noMemo")}
            </button>
          {/if}
          <span class="num">{hms(s.workMs)}</span>
          <button type="button" class="trash" aria-label={t("log.delete")} title={t("log.delete")} onclick={() => (confirmDel = confirmDel === s.id ? null : s.id)}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"
              ><path d="M4 7h16M10 11v6M14 11v6M6 7l1 12a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2l1-12M9 7V4h6v3" /></svg
            >
          </button>
        </div>
        <span class="session-sub num">{#if s.current}<b class="cur">{t("log.current")}</b>{" · "}{/if}{D.hm(s.startMs)}–{D.hm(s.endMs)} · {s.apps.join(", ")}</span>
        {#if confirmDel === s.id}
          <div class="confirm">
            <span>{s.current ? t("log.deleteCurrentAsk") : t("log.deleteSessionAsk")}</span>
            <div class="confirm-btns">
              <button type="button" class="ghost" onclick={() => (confirmDel = null)}>{t("today.cancel")}</button>
              <button type="button" class="danger" onclick={() => remove(s.id)}>{t("log.delete")}</button>
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <div class="muted">{t("log.noSessions")}</div>
    {/each}
  {:else if view === "week"}
    <div class="bars week">
      {#each week as d}
        <div class="bar-col">
          <span class="num bar-num" class:strong={d.isToday}>{d.future ? "" : hours(d.ms)}</span>
          <button
            type="button"
            class="bar"
            class:today={d.isToday}
            class:zero={d.ms === 0}
            style="height: {d.future ? 0 : Math.max(d.ms ? 3 : 0, d.pct * 0.96)}px"
            disabled={d.future}
            aria-label="{d.name} {hours(d.ms)}"
            onclick={() => setView("day", d.day)}
          ></button>
        </div>
      {/each}
    </div>
    <div class="bar-labels">
      {#each week as d}<span class:strong={d.isToday}>{d.name}</span>{/each}
    </div>
    <div class="hint">{t("log.tapBar")}</div>
  {:else if view === "month"}
    <div class="cal heads">{#each month.heads as h}<span>{h}</span>{/each}</div>
    <div class="cal">
      {#each month.cells as c}
        {#if c === null}
          <span></span>
        {:else}
          <button
            type="button"
            class="cell l{c.level}"
            class:future={c.future}
            class:today={c.isToday}
            disabled={c.future}
            aria-label="{c.day} {dur(c.ms)}"
            onclick={() => setView("day", c.day)}>{c.n}</button
          >
        {/if}
      {/each}
    </div>
    <div class="hint">{t("log.heatHint")}</div>
  {:else}
    <div class="bars year">
      {#each year as m}
        <button
          type="button"
          class="bar"
          class:today={m.best}
          class:zero={m.ms === 0}
          style="height: {m.future ? 0 : Math.max(m.ms ? 2 : 0, m.pct * 1.2)}px"
          disabled={m.future}
          aria-label="{m.n} {dur(m.ms)}"
          onclick={() => setView("month", m.first)}
        ></button>
      {/each}
    </div>
    <div class="bar-labels num">
      {#each year as m}<span class:strong={m.best}>{m.n}</span>{/each}
    </div>
    <div class="hint">
      {#if best}{t("log.bestMonth", { month: D.fmt(lang, best.first, { month: "long" }), time: dur(best.ms) })}{" · "}{/if}{t("log.tapMonth")}
    </div>
  {/if}

  {#if confirmDel === "day"}
    <div class="confirm">
      <span>{t("log.deleteDayAsk", { date: label })}</span>
      <div class="confirm-btns">
        <button type="button" class="ghost" onclick={() => (confirmDel = null)}>{t("today.cancel")}</button>
        <button type="button" class="danger" onclick={() => remove("day")}>{t("log.delete")}</button>
      </div>
    </div>
  {/if}
  <div class="export">
    {#if view === "day" && detail?.segments.length}
      <button type="button" class="text del-day" onclick={() => (confirmDel = confirmDel === "day" ? null : "day")}>{t("log.deleteDay")}</button>
    {/if}
    {#if message}<span class="msg">{message}</span>{/if}
    <button type="button" class="text" onclick={exportCsv}>{t("log.export")}</button>
  </div>
</div>

<style>
  .log {
    display: flex;
    flex-direction: column;
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
    height: 32px;
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
  .period {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 12px;
  }
  .nav {
    width: 40px;
    height: 40px;
    border: none;
    background: none;
    color: var(--ink);
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 6px;
  }
  .nav:disabled {
    color: var(--idle);
    cursor: default;
  }
  .period-label {
    height: 40px;
    border: none;
    background: none;
    color: var(--ink);
    font-size: 15px;
    font-weight: 600;
  }
  .hidden-date {
    position: absolute;
    left: 50%;
    bottom: 0;
    width: 1px;
    height: 1px;
    opacity: 0;
    pointer-events: none;
  }
  .total {
    display: flex;
    align-items: baseline;
    gap: 12px;
    margin-top: 4px;
    flex-wrap: wrap;
  }
  .big {
    font-size: 28px;
    font-weight: 500;
  }
  .sub,
  .label,
  .hint,
  .muted {
    font-size: 13px;
    color: var(--sub);
  }
  .hint {
    margin-top: 14px;
  }
  /* 일 */
  .timeline {
    position: relative;
    height: 20px;
    margin-top: 18px;
    border-radius: 3px;
    background: var(--line);
    overflow: hidden;
  }
  .tl-seg {
    position: absolute;
    top: 0;
    height: 20px;
  }
  .axis {
    display: flex;
    justify-content: space-between;
    margin-top: 4px;
    font-size: 12px;
    color: var(--sub);
  }
  .legend {
    display: flex;
    gap: 14px;
    margin-top: 6px;
    font-size: 12px;
    color: var(--sub);
  }
  .legend span {
    display: flex;
    align-items: center;
    gap: 5px;
  }
  .legend i {
    width: 8px;
    height: 8px;
    border-radius: 2px;
  }
  .session {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 10px 0;
    border-bottom: 1px solid var(--line);
  }
  .session-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 10px;
    font-size: 15px;
  }
  .memo {
    flex: 1;
    min-width: 0;
    padding: 0;
    border: none;
    background: none;
    color: var(--ink);
    font-size: 15px;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .memo.empty {
    color: var(--sub);
  }
  .note-input {
    flex: 1;
    min-width: 0;
    height: 30px;
    box-sizing: border-box;
    padding: 0 8px;
    border: 1px solid var(--ink);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 15px;
    outline: none;
  }
  .session-sub {
    font-size: 13px;
    color: var(--sub);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .muted {
    padding: 10px 0;
  }
  .trash {
    width: 28px;
    height: 28px;
    margin-right: -6px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    border-radius: 6px;
    background: none;
    color: var(--idle);
  }
  .trash:hover {
    color: var(--accent);
  }
  .cur {
    font-weight: 600;
    color: var(--accent);
  }
  .confirm {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 6px;
    padding: 10px 12px;
    border: 1px solid var(--accent);
    border-radius: 8px;
    font-size: 14px;
    line-height: 1.45;
  }
  .confirm-btns {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
  .confirm-btns button {
    height: 30px;
    padding: 0 14px;
    border-radius: 6px;
    font: inherit;
    font-size: 14px;
  }
  .ghost {
    border: 1px solid var(--line);
    background: var(--surface);
    color: var(--ink);
  }
  .danger {
    border: 1px solid var(--accent);
    background: var(--accent);
    color: var(--on-accent);
    font-weight: 600;
  }
  /* 주·연 막대 */
  .bars {
    display: flex;
    align-items: flex-end;
    margin-top: 20px;
    border-bottom: 1px solid var(--line);
  }
  .bars.week {
    gap: 10px;
    height: 132px;
  }
  .bars.year {
    gap: 6px;
    height: 140px;
  }
  .bar-col {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-end;
    gap: 4px;
    height: 100%;
  }
  .bar-num {
    font-size: 12px;
    color: var(--sub);
  }
  .bar {
    width: 100%;
    flex: none;
    padding: 0;
    border: none;
    border-radius: 3px 3px 0 0;
    background: var(--idle);
  }
  .bars.year .bar {
    flex: 1 1 0;
    width: auto;
  }
  .bar.today {
    background: var(--accent);
  }
  .bar.zero {
    background: transparent;
  }
  .bar:disabled {
    cursor: default;
  }
  .bar-labels {
    display: flex;
    gap: 10px;
    margin-top: 6px;
  }
  .bars.year + .bar-labels {
    gap: 6px;
  }
  .bar-labels span {
    flex: 1;
    text-align: center;
    font-size: 13px;
    color: var(--sub);
  }
  .bars.year + .bar-labels span {
    font-size: 12px;
  }
  .strong {
    color: var(--ink) !important;
    font-weight: 600;
  }
  /* 월 달력 */
  .cal {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    gap: 5px;
    margin-top: 6px;
  }
  .cal.heads {
    margin-top: 18px;
    font-size: 12px;
    color: var(--sub);
    text-align: center;
  }
  .cell {
    height: 34px;
    padding: 0;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    color: var(--ink);
    background: var(--line);
  }
  .cell.l0 {
    color: var(--sub);
  }
  .cell.l1 {
    background: color-mix(in srgb, var(--accent) 30%, var(--bg));
  }
  .cell.l2 {
    background: color-mix(in srgb, var(--accent) 50%, var(--bg));
  }
  .cell.l3 {
    background: color-mix(in srgb, var(--accent) 75%, var(--bg));
    color: var(--on-accent);
  }
  .cell.l4 {
    background: var(--accent);
    color: var(--on-accent);
  }
  .cell.today {
    outline: 1.5px solid var(--ink);
    outline-offset: 1px;
  }
  .cell.future {
    background: transparent;
    border: 1px dashed var(--line);
    color: var(--sub);
    cursor: default;
  }
  .export {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 10px;
    margin-top: 14px;
  }
  .msg {
    font-size: 13px;
    color: var(--sub);
  }
  .text {
    height: 36px;
    padding: 0;
    border: none;
    background: none;
    color: var(--sub);
    font-size: 14px;
  }
  .text:hover {
    color: var(--ink);
  }
  .del-day {
    margin-right: auto;
  }
  .del-day:hover {
    color: var(--accent);
  }
</style>
