//! 알림 소리. 사용자 파일(wav/mp3/ogg)은 %APPDATA%\Tally\sounds\ 로 복사해 두고 이름만 저장한다.
//! 내장 소리("builtin:이름")는 코드로 합성한다(실행 파일에 소리 파일을 넣지 않으려고).
//! 값이 없거나 파일을 못 읽으면 기본 종소리.

use std::path::{Path, PathBuf};

pub const SOUND_EXTS: [&str; 3] = ["wav", "mp3", "ogg"];

pub fn sounds_dir() -> PathBuf {
    crate::settings::data_dir().join("sounds")
}

#[derive(Debug, PartialEq)]
pub enum ImportError {
    /// wav/mp3/ogg가 아님
    BadType,
    Io(String),
}

/// 고른 파일을 sounds 폴더로 복사하고 저장할 파일 이름을 돌려준다. 같은 이름이 있으면 뒤에 번호를 붙인다.
pub fn import(src: &Path) -> Result<String, ImportError> {
    let io = |e: std::io::Error| ImportError::Io(e.to_string());
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .filter(|e| SOUND_EXTS.contains(&e.as_str()))
        .ok_or(ImportError::BadType)?;
    let stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or("sound").to_string();
    let dir = sounds_dir();
    std::fs::create_dir_all(&dir).map_err(io)?;
    // 이미 sounds 폴더 안의 파일이면 그대로
    if src.parent().map_or(false, |p| p == dir) {
        return Ok(src.file_name().unwrap().to_string_lossy().into_owned());
    }
    let mut name = format!("{stem}.{ext}");
    let mut n = 2;
    while dir.join(&name).exists() {
        if same_file(src, &dir.join(&name)) {
            return Ok(name);
        }
        name = format!("{stem} ({n}).{ext}");
        n += 1;
    }
    std::fs::copy(src, dir.join(&name)).map_err(io)?;
    Ok(name)
}

fn same_file(a: &Path, b: &Path) -> bool {
    match (std::fs::read(a), std::fs::read(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

const RATE: f32 = 44_100.0;

/// 내장 소리 이름. UI 표시 이름은 프론트 번역에 있다.
pub const BUILTINS: [&str; 6] = ["bell", "chime", "marimba", "soft", "bowl", "rise"];

/// 음 하나를 buf에 더한다. partials = (배수, 세기, 추가 감쇠)
fn note(buf: &mut [f32], freq: f32, start: f32, decay: f32, gain: f32, partials: &[(f32, f32, f32)]) {
    let from = (start * RATE) as usize;
    for (i, out) in buf.iter_mut().enumerate().skip(from) {
        let t = (i - from) as f32 / RATE;
        let attack = (t / 0.006).min(1.0);
        let mut v = 0.0;
        for &(mul, amp, extra) in partials {
            v += amp * (std::f32::consts::TAU * freq * mul * t).sin() * (-t * extra).exp();
        }
        *out += gain * attack * (-t * decay).exp() * v;
    }
}

/// 내장 소리 합성(44.1kHz 모노)
pub fn builtin_samples(name: &str) -> Vec<f32> {
    let bell = [(1.0, 1.0, 0.0), (1.5, 0.45, 2.0), (3.0, 0.2, 6.0)];
    let wood = [(1.0, 1.0, 0.0), (4.0, 0.25, 18.0)];
    let pure = [(1.0, 1.0, 0.0)];
    let (len, notes): (f32, Vec<(f32, f32, f32, f32, &[(f32, f32, f32)])>) = match name {
        // 딩-동
        "chime" => (2.2, vec![(1318.5, 0.0, 2.6, 0.26, &bell), (1046.5, 0.45, 2.2, 0.26, &bell)]),
        // 도-미-솔 나무 건반
        "marimba" => (
            1.4,
            vec![(523.3, 0.0, 7.0, 0.4, &wood), (659.3, 0.16, 7.0, 0.4, &wood), (784.0, 0.32, 6.0, 0.4, &wood)],
        ),
        // 부드러운 두 번 삐
        "soft" => (1.0, vec![(660.0, 0.0, 9.0, 0.3, &pure), (660.0, 0.3, 9.0, 0.3, &pure)]),
        // 싱잉볼: 낮고 길게, 살짝 맥놀이
        "bowl" => (
            4.0,
            vec![(293.7, 0.0, 0.9, 0.22, &[(1.0, 1.0, 0.0), (1.004, 0.6, 0.0), (2.76, 0.3, 1.5)])],
        ),
        // 올라가는 아르페지오(휴식 끝)
        "rise" => (
            1.8,
            vec![
                (523.3, 0.0, 4.0, 0.22, &bell),
                (659.3, 0.14, 4.0, 0.22, &bell),
                (784.0, 0.28, 4.0, 0.22, &bell),
                (1046.5, 0.42, 2.6, 0.24, &bell),
            ],
        ),
        // bell(기본 종소리)
        _ => (1.6, vec![(880.0, 0.0, 3.2, 0.32, &bell)]),
    };
    let mut buf = vec![0.0f32; (RATE * len) as usize];
    for (f, start, decay, gain, partials) in notes {
        note(&mut buf, f, start, decay, gain, partials);
    }
    // 겹친 음이 넘치지 않게
    let peak = buf.iter().fold(0.0f32, |m, x| m.max(x.abs()));
    if peak > 0.9 {
        buf.iter_mut().for_each(|x| *x *= 0.9 / peak);
    }
    buf
}

/// 별도 스레드에서 끝까지 재생한다. 장치가 없거나 파일이 깨졌으면 조용히 기본 종소리로.
pub fn play(sound: Option<String>, volume: f32) {
    std::thread::spawn(move || {
        if let Err(e) = play_blocking(sound.as_deref(), volume) {
            eprintln!("소리 재생 실패: {e}");
        }
    });
}

fn play_blocking(sound: Option<&str>, volume: f32) -> Result<(), String> {
    use std::num::NonZero;
    let sink = rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| e.to_string())?;
    let player = rodio::Player::connect_new(sink.mixer());
    player.set_volume(volume.clamp(0.0, 1.0));
    let builtin = sound.and_then(|s| s.strip_prefix("builtin:"));
    let file = sound
        .filter(|_| builtin.is_none())
        .map(|name| sounds_dir().join(name))
        .and_then(|p| std::fs::File::open(p).ok())
        .and_then(|f| rodio::Decoder::try_from(f).ok());
    match file {
        Some(dec) => player.append(dec),
        None => player.append(rodio::buffer::SamplesBuffer::new(
            NonZero::new(1).unwrap(),
            NonZero::new(RATE as u32).unwrap(),
            builtin_samples(builtin.unwrap_or("bell")),
        )),
    }
    player.sleep_until_end();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins_are_audible_short_and_not_clipping() {
        for name in BUILTINS {
            let s = builtin_samples(name);
            let secs = s.len() as f32 / RATE;
            assert!((0.9..=4.1).contains(&secs), "{name} {secs}");
            assert!(s.iter().all(|x| x.abs() <= 0.95), "{name} clips");
            assert!(s.iter().any(|x| x.abs() > 0.15), "{name} too quiet");
            assert!(s.last().unwrap().abs() < 0.03, "{name} cut off");
        }
        assert_eq!(builtin_samples("unknown").len(), builtin_samples("bell").len());
    }

    #[test]
    fn import_rejects_other_file_types() {
        assert_eq!(import(Path::new("C:/x/notes.txt")), Err(ImportError::BadType));
    }
}
