// 트레이용 상태별 SVG → 32px PNG (Windows가 16px로 줄여 쓴다)
import { Resvg } from "@resvg/resvg-js";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";

const src = "assets/logo";
const out = "src-tauri/icons/tray";
mkdirSync(out, { recursive: true });
for (const state of ["working", "idle", "paused"]) {
  for (const theme of ["dark", "light"]) {
    const name = `tray-${state}-${theme}`;
    const svg = readFileSync(`${src}/${name}.svg`, "utf8");
    const png = new Resvg(svg, { fitTo: { mode: "width", value: 32 } }).render().asPng();
    writeFileSync(`${out}/${name}.png`, png);
    console.log(`${out}/${name}.png`);
  }
}
