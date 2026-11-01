// 구간 편집기 계산: 해독, 파형 요약, WAV 만들기. 해독은 WebView2(Chromium)가 한다.

export const RATE = 44_100;
/** 편집할 수 있는 원본 길이 상한(해독하면 분당 약 21MB를 쓴다) */
export const MAX_SOURCE_MIN = 10;
/** 알림음 길이 상한 */
export const MAX_CLIP_SEC = 60;
export const MIN_CLIP_SEC = 0.05;
/** 파형 요약 한 칸의 샘플 수 */
const BIN = 256;

export class TooLong extends Error {}

const MIME: Record<string, string> = {
  mp3: "audio/mpeg",
  wav: "audio/wav",
  ogg: "audio/ogg",
  oga: "audio/ogg",
  opus: "audio/ogg",
  m4a: "audio/mp4",
  aac: "audio/aac",
  flac: "audio/flac",
  mp4: "video/mp4",
  m4v: "video/mp4",
  mov: "video/quicktime",
  webm: "video/webm",
  mkv: "video/x-matroska",
};

/** 미디어 요소로 길이만 먼저 읽는다(해독 전에 너무 긴 파일을 거르려고). 모르면 NaN */
function probeDuration(bytes: ArrayBuffer, ext: string): Promise<number> {
  const url = URL.createObjectURL(new Blob([bytes], { type: MIME[ext] ?? "" }));
  return new Promise<number>((resolve) => {
    const el = document.createElement("video");
    const done = (v: number) => {
      el.removeAttribute("src");
      el.load();
      URL.revokeObjectURL(url);
      resolve(v);
    };
    el.preload = "metadata";
    el.muted = true;
    el.onloadedmetadata = () => done(el.duration);
    el.onerror = () => done(NaN);
    setTimeout(() => done(NaN), 5000);
    el.src = url;
  });
}

/** 파일 바이트 → 모노 샘플(RATE). 길이를 모르는 파일은 크기로 어림한다 */
export async function decode(bytes: ArrayBuffer, ext: string): Promise<Float32Array> {
  const dur = await probeDuration(bytes, ext);
  const tooLong = Number.isFinite(dur) ? dur > MAX_SOURCE_MIN * 60 + 1 : bytes.byteLength > 150 * 1024 * 1024;
  if (tooLong) throw new TooLong();
  const ctx = new OfflineAudioContext(1, 1, RATE);
  const buf = await ctx.decodeAudioData(bytes);
  if (buf.duration > MAX_SOURCE_MIN * 60 + 1) throw new TooLong();
  const out = new Float32Array(buf.length);
  for (let c = 0; c < buf.numberOfChannels; c++) {
    const ch = buf.getChannelData(c);
    for (let i = 0; i < ch.length; i++) out[i] += ch[i] / buf.numberOfChannels;
  }
  return out;
}

export type Peaks = { min: Float32Array; max: Float32Array };

/** BIN 샘플마다 최소·최대 — 많이 축소했을 때 이걸로 그린다 */
export function summarize(s: Float32Array): Peaks {
  const n = Math.ceil(s.length / BIN);
  const min = new Float32Array(n);
  const max = new Float32Array(n);
  for (let b = 0; b < n; b++) {
    let lo = 0;
    let hi = 0;
    const end = Math.min(s.length, (b + 1) * BIN);
    for (let i = b * BIN; i < end; i++) {
      const v = s[i];
      if (v < lo) lo = v;
      if (v > hi) hi = v;
    }
    min[b] = lo;
    max[b] = hi;
  }
  return { min, max };
}

/** 샘플 구간 [a, b)의 최소·최대 */
export function rangePeak(s: Float32Array, p: Peaks, a: number, b: number): [number, number] {
  a = Math.max(0, Math.floor(a));
  b = Math.min(s.length, Math.ceil(b));
  let lo = 0;
  let hi = 0;
  if (b - a >= BIN * 2) {
    for (let i = Math.ceil(a / BIN); i < Math.floor(b / BIN); i++) {
      if (p.min[i] < lo) lo = p.min[i];
      if (p.max[i] > hi) hi = p.max[i];
    }
  } else {
    for (let i = a; i < b; i++) {
      if (s[i] < lo) lo = s[i];
      if (s[i] > hi) hi = s[i];
    }
  }
  return [lo, hi];
}

/** 구간을 잘라 앞뒤를 줄여(딸깍 소리 방지 5ms, fade면 최대 0.3초) 모노 16비트 WAV로 */
export function clipWav(s: Float32Array, start: number, end: number, fade: boolean): Uint8Array {
  const part = s.slice(Math.max(0, start), Math.min(s.length, end));
  const n = part.length;
  const ramp = Math.min(Math.floor(n / 4), Math.round((fade ? 0.3 : 0.005) * RATE));
  for (let i = 0; i < ramp; i++) {
    const g = i / ramp;
    part[i] *= g;
    part[n - 1 - i] *= g;
  }
  const out = new Uint8Array(44 + n * 2);
  const v = new DataView(out.buffer);
  const str = (o: number, t: string) => [...t].forEach((c, i) => v.setUint8(o + i, c.charCodeAt(0)));
  str(0, "RIFF");
  v.setUint32(4, 36 + n * 2, true);
  str(8, "WAVE");
  str(12, "fmt ");
  v.setUint32(16, 16, true);
  v.setUint16(20, 1, true); // PCM
  v.setUint16(22, 1, true); // 모노
  v.setUint32(24, RATE, true);
  v.setUint32(28, RATE * 2, true);
  v.setUint16(32, 2, true);
  v.setUint16(34, 16, true);
  str(36, "data");
  v.setUint32(40, n * 2, true);
  for (let i = 0; i < n; i++) {
    const x = Math.max(-1, Math.min(1, part[i]));
    v.setInt16(44 + i * 2, Math.round(x * 32767), true);
  }
  return out;
}

/** 초 → "1:05.3" */
export function secLabel(sec: number): string {
  const m = Math.floor(sec / 60);
  const s = sec - m * 60;
  return `${m}:${s.toFixed(1).padStart(4, "0")}`;
}
