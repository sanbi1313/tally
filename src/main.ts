import { getCurrentWindow } from "@tauri-apps/api/window";
import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import Mini from "./Mini.svelte";

// 같은 페이지를 두 창이 쓴다. 창 이름(label)이 "mini"면 작은 타이머
const isMini = getCurrentWindow().label === "mini";
mount(isMini ? Mini : App, { target: document.getElementById("app")! });
