<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { t, type Key } from "./i18n.svelte";

  // 내장 소리 + (지정한 파일) + "파일에서 고르기…". 값: "builtin:이름" | 파일 이름 | null(기본값)
  let {
    value,
    fallback,
    label,
    onchange,
  }: {
    value: string | null;
    fallback: string;
    label: string;
    onchange: (v: string) => void;
  } = $props();

  const BUILTINS = ["bell", "chime", "marimba", "soft", "bowl", "rise"];
  const current = $derived(value ?? fallback);
  const isFile = $derived(!current.startsWith("builtin:"));

  async function change(e: Event & { currentTarget: HTMLSelectElement }) {
    const v = e.currentTarget.value;
    if (v !== "__file") return onchange(v);
    e.currentTarget.value = current; // 고르기를 취소해도 원래 값 유지
    const path = await open({ multiple: false, filters: [{ name: t("sound.fileFilter"), extensions: ["wav", "mp3", "ogg"] }] });
    if (typeof path === "string") onchange(path);
  }
</script>

<select aria-label={label} value={current} onchange={change}>
  {#each BUILTINS as b}
    <option value="builtin:{b}">{t(`sound.${b}` as Key)}</option>
  {/each}
  {#if isFile}<option value={current}>{current}</option>{/if}
  <option value="__file">{t("sound.file")}</option>
</select>

<style>
  select {
    flex: 1;
    min-width: 0;
    height: 30px;
    padding: 0 6px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--surface);
    color: var(--ink);
    font: inherit;
    font-size: 14px;
  }
  select:focus {
    outline: none;
    border-color: var(--ink);
  }
</style>
