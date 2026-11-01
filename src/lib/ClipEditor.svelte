<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import { MAX_CLIP_SEC, MAX_SOURCE_MIN, MIN_CLIP_SEC, RATE, TooLong, clipWav, decode, rangePeak, secLabel, summarize, type Peaks } from "./clip";
  import { t } from "./i18n.svelte";

  // 소리·영상 파일에서 일부만 잘라 알림음(WAV)으로 저장한다. 저장하면 sounds 폴더의 전체 경로를 넘긴다
  let { path, onsave, oncancel }: { path: string; onsave: (saved: string) => void; oncancel: () => void } = $props();

  // path는 열려 있는 동안 바뀌지 않는다
  // svelte-ignore state_referenced_locally
  const fileName = path.split(/[\\/]/).pop() ?? "";
  const ext = (fileName.split(".").pop() ?? "").toLowerCase();
  const stem = fileName.replace(/\.[^.]*$/, "");

  let phase = $state<"loading" | "ready" | "saving" | "error">("loading");
  let error = $state("");
  let samples = new Float32Array(0);
  let peaks: Peaks = { min: new Float32Array(0), max: new Float32Array(0) };
  let total = $state(0);
  // 고른 구간과 보이는 범위(샘플 단위)
  let sel = $state({ a: 0, b: 0 });
  let view = $state({ a: 0, b: 1 });
  let fade = $state(true);
  let head = $state<number | null>(null);
  let rev = $state(0); // 테마가 바뀌면 다시 그리기

  let main = $state<HTMLCanvasElement>();
  let mini = $state<HTMLCanvasElement>();

  const maxLen = MAX_CLIP_SEC * RATE;
  const minLen = Math.round(MIN_CLIP_SEC * RATE);
  const minView = Math.round(0.2 * RATE);

  onMount(() => {
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && phase !== "saving" && oncancel();
    window.addEventListener("keydown", onKey);
    const mo = new MutationObserver(() => rev++);
    mo.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
    load();
    return () => {
      window.removeEventListener("keydown", onKey);
      mo.disconnect();
    };
  });

  async function load() {
    try {
      const bytes = await invoke<ArrayBuffer>("read_media", { path });
      samples = (await decode(bytes, ext)) as Float32Array<ArrayBuffer>;
      if (samples.length < minLen) throw new Error();
      peaks = summarize(samples);
      total = samples.length;
      sel = { a: 0, b: Math.min(total, 10 * RATE) };
      view = { a: 0, b: total };
      phase = "ready";
    } catch (e) {
      error = e instanceof TooLong ? t("clip.errLong", { min: MAX_SOURCE_MIN }) : typeof e === "string" ? e : t("clip.errRead");
      phase = "error";
    }
  }

  // ── 그리기 ──
  function color(name: string) {
    return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  }

  function prep(c: HTMLCanvasElement) {
    const dpr = window.devicePixelRatio || 1;
    const w = c.clientWidth;
    const h = c.clientHeight;
    if (c.width !== Math.round(w * dpr) || c.height !== Math.round(h * dpr)) {
      c.width = Math.round(w * dpr);
      c.height = Math.round(h * dpr);
    }
    const g = c.getContext("2d")!;
    g.setTransform(dpr, 0, 0, dpr, 0, 0);
    g.clearRect(0, 0, w, h);
    return { g, w, h };
  }

  function wave(g: CanvasRenderingContext2D, w: number, h: number, a: number, b: number, from: number, to: number) {
    const spp = (to - from) / w;
    const ink = color("--sub");
    const hot = color("--accent");
    for (let x = 0; x < w; x++) {
      const s0 = from + x * spp;
      const [lo, hi] = rangePeak(samples, peaks, s0, s0 + Math.max(1, spp));
      const y0 = h / 2 - hi * (h / 2 - 2);
      const y1 = h / 2 - lo * (h / 2 - 2);
      g.fillStyle = s0 + spp > a && s0 < b ? hot : ink;
      g.fillRect(x, y0, 1, Math.max(1, y1 - y0));
    }
  }

  $effect(() => {
    void rev;
    if (phase === "loading" || phase === "error" || !main || !mini) return;
    const { a, b } = sel;
    // 큰 화면: 보이는 범위
    {
      const { g, w, h } = prep(main);
      const x = (s: number) => ((s - view.a) / (view.b - view.a)) * w;
      g.fillStyle = color("--accent");
      g.globalAlpha = 0.12;
      g.fillRect(x(a), 0, x(b) - x(a), h);
      g.globalAlpha = 1;
      wave(g, w, h, a, b, view.a, view.b);
      g.fillStyle = color("--accent");
      for (const s of [a, b]) {
        g.fillRect(Math.round(x(s)) - 1, 0, 2, h);
        g.fillRect(Math.round(x(s)) - 4, 0, 8, 6);
      }
      if (head !== null) {
        g.fillStyle = color("--ink");
        g.fillRect(Math.round(x(head)), 0, 1, h);
      }
    }
    // 작은 화면: 전체와 보이는 범위
    {
      const { g, w, h } = prep(mini);
      wave(g, w, h, a, b, 0, total);
      const x = (s: number) => (s / total) * w;
      g.strokeStyle = color("--ink");
      g.lineWidth = 1;
      g.strokeRect(x(view.a) + 0.5, 0.5, Math.max(2, x(view.b) - x(view.a)) - 1, h - 1);
    }
  });

  // ── 조작 ──
  const clamp = (v: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, v));

  function setView(a: number, b: number) {
    const len = clamp(b - a, minView, total);
    a = clamp(a, 0, total - len);
    view = { a, b: a + len };
  }

  function zoom(f: number, at = (view.a + view.b) / 2) {
    const len = (view.b - view.a) * f;
    const k = (at - view.a) / (view.b - view.a);
    setView(at - len * k, at - len * k + len);
  }

  function fitSel() {
    const pad = (sel.b - sel.a) * 0.25;
    setView(sel.a - pad, sel.b + pad);
  }

  /** 시작이나 끝을 옮기고 길이 제한을 지킨다(움직인 쪽만 고친다) */
  function setSel(a: number, b: number, moved: "a" | "b") {
    a = clamp(Math.round(a), 0, total);
    b = clamp(Math.round(b), 0, total);
    if (moved === "a") a = clamp(a, Math.max(0, b - maxLen), b - minLen);
    else b = clamp(b, a + minLen, Math.min(total, a + maxLen));
    sel = { a, b };
  }

  let drag: { kind: "a" | "b" | "move" | "new"; from: number; sel: { a: number; b: number } } | null = null;

  function sampleAt(e: PointerEvent) {
    const r = main!.getBoundingClientRect();
    return view.a + ((e.clientX - r.left) / r.width) * (view.b - view.a);
  }

  function down(e: PointerEvent) {
    if (e.button !== 0) return;
    stop();
    const s = sampleAt(e);
    const px = (view.b - view.a) / main!.clientWidth;
    const near = (v: number) => Math.abs(s - v) <= 6 * px;
    const kind = near(sel.a) ? "a" : near(sel.b) ? "b" : s > sel.a && s < sel.b ? "move" : "new";
    drag = { kind, from: s, sel: { ...sel } };
    main!.setPointerCapture(e.pointerId);
  }

  function move(e: PointerEvent) {
    const s = sampleAt(e);
    if (!drag) {
      const px = (view.b - view.a) / main!.clientWidth;
      const edge = Math.abs(s - sel.a) <= 6 * px || Math.abs(s - sel.b) <= 6 * px;
      main!.style.cursor = edge ? "ew-resize" : s > sel.a && s < sel.b ? "grab" : "crosshair";
      return;
    }
    const d = drag;
    if (d.kind === "a") setSel(s, sel.b, "a");
    else if (d.kind === "b") setSel(sel.a, s, "b");
    else if (d.kind === "move") {
      const len = d.sel.b - d.sel.a;
      const a = clamp(d.sel.a + (s - d.from), 0, total - len);
      sel = { a: Math.round(a), b: Math.round(a + len) };
    } else if (Math.abs(s - d.from) > (view.b - view.a) / main!.clientWidth * 3) {
      if (s > d.from) setSel(d.from, s, "b");
      else setSel(s, d.from, "a");
    }
  }

  function up() {
    drag = null;
  }

  function wheel(e: WheelEvent) {
    e.preventDefault();
    if (e.shiftKey || Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
      const d = (e.shiftKey ? e.deltaY : e.deltaX) / main!.clientWidth;
      setView(view.a + d * (view.b - view.a), view.b + d * (view.b - view.a));
      return;
    }
    const r = main!.getBoundingClientRect();
    const at = view.a + ((e.clientX - r.left) / r.width) * (view.b - view.a);
    zoom(e.deltaY < 0 ? 1 / 1.25 : 1.25, at);
  }

  // 작은 화면: 누르고 끌면 그 자리를 가운데로
  let miniDrag = false;
  function miniAt(e: PointerEvent) {
    const r = mini!.getBoundingClientRect();
    const c = ((e.clientX - r.left) / r.width) * total;
    const len = view.b - view.a;
    setView(c - len / 2, c + len / 2);
  }

  // ── 숫자 입력(초) ──
  function setSec(which: "a" | "b", e: Event & { currentTarget: HTMLInputElement }) {
    const v = Number(e.currentTarget.value);
    if (Number.isFinite(v)) {
      if (which === "a") setSel(v * RATE, sel.b, "a");
      else setSel(sel.a, v * RATE, "b");
    }
    e.currentTarget.value = ((which === "a" ? sel.a : sel.b) / RATE).toFixed(2);
  }

  // ── 들어보기 ──
  let ctx: AudioContext | null = null;
  let src: AudioBufferSourceNode | null = null;
  let raf = 0;

  function play() {
    stop();
    ctx ??= new AudioContext({ sampleRate: RATE });
    const n = sel.b - sel.a;
    const buf = ctx.createBuffer(1, n, RATE);
    buf.copyToChannel(samples.slice(sel.a, sel.b), 0);
    const gain = ctx.createGain();
    const ramp = Math.min(n / 4, (fade ? 0.3 : 0.005) * RATE) / RATE;
    const dur = n / RATE;
    const t0 = ctx.currentTime + 0.02;
    gain.gain.setValueAtTime(0, t0);
    gain.gain.linearRampToValueAtTime(1, t0 + ramp);
    gain.gain.setValueAtTime(1, t0 + dur - ramp);
    gain.gain.linearRampToValueAtTime(0, t0 + dur);
    const node = ctx.createBufferSource();
    node.buffer = buf;
    node.connect(gain).connect(ctx.destination);
    node.onended = () => src === node && stop();
    node.start(t0);
    src = node;
    const a = sel.a;
    const tick = () => {
      if (!ctx || src !== node) return;
      head = a + Math.max(0, ctx.currentTime - t0) * RATE;
      raf = requestAnimationFrame(tick);
    };
    tick();
  }

  function stop() {
    cancelAnimationFrame(raf);
    const s = src;
    src = null;
    try {
      s?.stop();
    } catch {}
    head = null;
  }

  onDestroy(() => {
    stop();
    ctx?.close();
  });

  async function save() {
    stop();
    phase = "saving";
    try {
      const wav = clipWav(samples, sel.a, sel.b, fade);
      const saved = await invoke<string>("save_clip", wav, { headers: { "x-name": encodeURIComponent(stem) } });
      onsave(saved);
    } catch (e) {
      error = String(e);
      phase = "ready";
    }
  }

  const len = $derived((sel.b - sel.a) / RATE);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="back" onclick={(e) => e.target === e.currentTarget && phase !== "saving" && oncancel()}>
  <div class="box" role="dialog" aria-modal="true" aria-label={t("clip.title")}>
    <h2>{t("clip.title")}</h2>
    <div class="file" title={path}>{fileName}</div>

    {#if phase === "loading"}
      <p class="msg">{t("clip.loading")}</p>
    {:else if phase === "error" && total === 0}
      <p class="msg err">{error}</p>
      <div class="foot"><button type="button" onclick={oncancel}>{t("clip.close")}</button></div>
    {:else}
      <canvas
        class="main"
        bind:this={main}
        onpointerdown={down}
        onpointermove={move}
        onpointerup={up}
        onpointercancel={up}
        onwheel={wheel}
        ondblclick={fitSel}
      ></canvas>
      <div class="range num"><span>{secLabel(view.a / RATE)}</span><span>{secLabel(view.b / RATE)}</span></div>
      <canvas
        class="mini"
        bind:this={mini}
        onpointerdown={(e) => ((miniDrag = true), mini!.setPointerCapture(e.pointerId), miniAt(e))}
        onpointermove={(e) => miniDrag && miniAt(e)}
        onpointerup={() => (miniDrag = false)}
      ></canvas>
      <p class="hint">{t("clip.hint")}</p>

      <div class="row">
        <div class="zoom">
          <button type="button" class="sq" aria-label={t("clip.zoomOut")} title={t("clip.zoomOut")} onclick={() => zoom(2)}>−</button>
          <button type="button" class="sq" aria-label={t("clip.zoomIn")} title={t("clip.zoomIn")} onclick={() => zoom(0.5)}>+</button>
          <button type="button" onclick={fitSel}>{t("clip.fitSel")}</button>
          <button type="button" onclick={() => setView(0, total)}>{t("clip.fitAll")}</button>
        </div>
      </div>

      <div class="row times">
        <label>{t("clip.start")}<input class="num" type="number" step="0.1" min="0" value={(sel.a / RATE).toFixed(2)} onchange={(e) => setSec("a", e)} /></label>
        <label>{t("clip.end")}<input class="num" type="number" step="0.1" min="0" value={(sel.b / RATE).toFixed(2)} onchange={(e) => setSec("b", e)} /></label>
        <span class="len num">{t("clip.length", { sec: len.toFixed(1), max: MAX_CLIP_SEC })}</span>
      </div>

      <div class="row">
        <button type="button" class="play" onclick={() => (head === null ? play() : stop())}>{head === null ? t("clip.play") : t("clip.stop")}</button>
        <label class="check"><input type="checkbox" bind:checked={fade} />{t("clip.fade")}</label>
      </div>

      {#if error}<p class="msg err">{error}</p>{/if}

      <div class="foot">
        <button type="button" disabled={phase === "saving"} onclick={oncancel}>{t("clip.cancel")}</button>
        <button type="button" class="primary" disabled={phase === "saving"} onclick={save}>{phase === "saving" ? t("clip.saving") : t("clip.save")}</button>
      </div>
    {/if}
  </div>
</div>

<style>
  .back {
    position: fixed;
    inset: 0;
    z-index: 50;
    background: color-mix(in srgb, var(--bg) 60%, transparent);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 12px;
  }
  .box {
    width: 100%;
    max-width: 520px;
    max-height: 100%;
    overflow-y: auto;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 10px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: 0 8px 28px rgb(0 0 0 / 0.18);
  }
  h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }
  .file {
    font-size: 13px;
    color: var(--sub);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  canvas {
    box-sizing: border-box;
    display: block;
    width: 100%;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--bg);
    touch-action: none;
  }
  .main {
    height: 110px;
    margin-top: 4px;
  }
  .mini {
    height: 28px;
    cursor: pointer;
  }
  .range {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: var(--sub);
    margin-top: -4px;
  }
  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--sub);
  }
  .msg {
    margin: 8px 0;
    font-size: 14px;
  }
  .err {
    color: var(--accent);
  }
  .row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .zoom {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
  }
  .times label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: var(--sub);
  }
  .times input {
    width: 72px;
    height: 28px;
    padding: 0 6px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 14px;
  }
  .times input:focus {
    outline: none;
    border-color: var(--ink);
  }
  .len {
    font-size: 13px;
    color: var(--sub);
    margin-left: auto;
  }
  button {
    height: 30px;
    padding: 0 12px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 14px;
  }
  button:hover:not(:disabled) {
    border-color: var(--ink);
  }
  button.sq {
    width: 30px;
    padding: 0;
  }
  button.play {
    min-width: 88px;
  }
  button.primary {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--on-accent);
    font-weight: 600;
  }
  button:disabled {
    opacity: 0.6;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
  }
  .check input {
    accent-color: var(--accent);
  }
  .foot {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 6px;
  }
</style>
