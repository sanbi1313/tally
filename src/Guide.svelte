<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { t, type Key } from "./lib/i18n.svelte";
  import type { SettingsOut } from "./lib/types";

  // 사용법. 단축키는 지금 설정값으로 보여 준다
  const sections = ["start", "numbers", "programs", "mini", "chip", "alerts", "shortcuts", "tray", "tools", "log"];
  let keys = $state({ pause: "Ctrl+Alt+P", reset: "Ctrl+Alt+R" });

  onMount(async () => {
    const s = await invoke<SettingsOut>("get_settings");
    keys = {
      pause: s.shortcuts.pause || t("general.scNone"),
      reset: s.shortcuts.reset || t("general.scNone"),
    };
  });

  const lines = (sec: string) => t(`guide.${sec}.b` as Key, keys).split("\n");
</script>

<div class="guide">
  <h1>{t("guide.title")}</h1>
  {#each sections as sec}
    <section>
      <h2>{t(`guide.${sec}.t` as Key)}</h2>
      <ul>
        {#each lines(sec) as line}<li>{line}</li>{/each}
      </ul>
    </section>
  {/each}
</div>

<style>
  .guide {
    display: flex;
    flex-direction: column;
  }
  h1 {
    margin: 0 0 4px;
    font-size: 22px;
    font-weight: 600;
  }
  section {
    padding: 14px 0;
    border-bottom: 1px solid var(--line);
  }
  section:last-child {
    border-bottom: none;
  }
  h2 {
    margin: 0 0 6px;
    font-size: 13px;
    font-weight: 600;
    color: var(--sub);
  }
  ul {
    margin: 0;
    padding-left: 18px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  li {
    font-size: 14px;
    line-height: 1.5;
    color: var(--ink);
    user-select: text;
  }
  li::marker {
    color: var(--idle);
  }
</style>
