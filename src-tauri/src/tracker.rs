//! Windows API로 포그라운드 창의 프로세스와 마지막 입력 시각을 읽는다.

use crate::engine::Sample;

#[cfg(windows)]
mod imp {
    use super::Sample;
    use windows::core::PWSTR;
    use windows::Win32::Foundation::{CloseHandle, HWND};
    use windows::Win32::System::SystemInformation::GetTickCount;
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    };

    /// 같은 프로세스를 매초 다시 열지 않도록 마지막 pid → exe를 기억한다.
    #[derive(Default)]
    pub struct Tracker {
        last_pid: u32,
        last_exe: Option<String>,
    }

    impl Tracker {
        pub fn sample(&mut self) -> Sample {
            let idle_ms = idle_ms();
            let hwnd = unsafe { GetForegroundWindow() };
            if hwnd.is_invalid() {
                return Sample { exe: None, title: String::new(), idle_ms };
            }
            let mut pid = 0u32;
            unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
            if pid == 0 {
                return Sample { exe: None, title: String::new(), idle_ms };
            }
            if pid != self.last_pid {
                self.last_pid = pid;
                self.last_exe = exe_of(pid);
            }
            Sample { exe: self.last_exe.clone(), title: title_of(hwnd), idle_ms }
        }
    }

    fn idle_ms() -> u64 {
        let mut info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };
        if unsafe { GetLastInputInfo(&mut info) }.as_bool() {
            // 둘 다 49.7일마다 되감기는 u32 틱이라 wrapping_sub로 차이를 구한다
            unsafe { GetTickCount() }.wrapping_sub(info.dwTime) as u64
        } else {
            0
        }
    }

    fn exe_of(pid: u32) -> Option<String> {
        let path = path_of(pid)?;
        path.rsplit(['\\', '/']).next().map(str::to_string)
    }

    fn title_of(hwnd: HWND) -> String {
        let mut buf = [0u16; 512];
        let n = unsafe { GetWindowTextW(hwnd, &mut buf) };
        String::from_utf16_lossy(&buf[..n.max(0) as usize])
    }

    fn path_of(pid: u32) -> Option<String> {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
            let mut buf = [0u16; 1024];
            let mut len = buf.len() as u32;
            let res = QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(buf.as_mut_ptr()), &mut len);
            let _ = CloseHandle(handle);
            res.ok()?;
            Some(String::from_utf16_lossy(&buf[..len as usize]))
        }
    }

    /// 작업 표시줄에 보이는 창을 가진 프로그램들(첫 실행·추가 화면용)
    pub fn running_programs() -> Vec<super::RunningProgram> {
        use windows::core::BOOL;
        use windows::Win32::Foundation::LPARAM;
        use windows::Win32::Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED};
        use windows::Win32::UI::WindowsAndMessaging::{
            EnumWindows, GetWindow, GetWindowLongW, GetWindowTextLengthW, IsWindowVisible, GWL_EXSTYLE,
            GW_OWNER, WS_EX_TOOLWINDOW,
        };

        unsafe extern "system" fn collect(hwnd: HWND, lp: LPARAM) -> BOOL {
            let list = &mut *(lp.0 as *mut Vec<HWND>);
            list.push(hwnd);
            BOOL(1)
        }
        let mut hwnds: Vec<HWND> = Vec::new();
        unsafe {
            let _ = EnumWindows(Some(collect), LPARAM(&mut hwnds as *mut _ as isize));
        }
        let skip = ["tally.exe", "applicationframehost.exe", "textinputhost.exe", "shellexperiencehost.exe", "searchhost.exe", "startmenuexperiencehost.exe"];
        let mut out: Vec<super::RunningProgram> = Vec::new();
        for hwnd in hwnds {
            unsafe {
                if !IsWindowVisible(hwnd).as_bool() || GetWindowTextLengthW(hwnd) == 0 {
                    continue;
                }
                if GetWindow(hwnd, GW_OWNER).map_or(false, |o| !o.is_invalid()) {
                    continue;
                }
                if GetWindowLongW(hwnd, GWL_EXSTYLE) as u32 & WS_EX_TOOLWINDOW.0 != 0 {
                    continue;
                }
                let mut cloaked = 0u32;
                let _ = DwmGetWindowAttribute(
                    hwnd,
                    DWMWA_CLOAKED,
                    &mut cloaked as *mut u32 as *mut _,
                    std::mem::size_of::<u32>() as u32,
                );
                if cloaked != 0 {
                    continue;
                }
                let mut pid = 0u32;
                GetWindowThreadProcessId(hwnd, Some(&mut pid));
                let Some(path) = path_of(pid) else { continue };
                let exe = path.rsplit(['\\', '/']).next().unwrap_or(&path).to_string();
                if skip.contains(&exe.to_lowercase().as_str())
                    || out.iter().any(|p| p.exe.eq_ignore_ascii_case(&exe))
                {
                    continue;
                }
                out.push(super::RunningProgram { exe, path, title: title_of(hwnd) });
            }
        }
        out
    }

    /// exe 아이콘을 32px RGBA PNG로
    pub fn icon_png(path: &str) -> Option<Vec<u8>> {
        use windows::core::HSTRING;
        use windows::Win32::Graphics::Gdi::{
            CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, BITMAP, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HGDIOBJ,
        };
        use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
        use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
        use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, ICONINFO};

        unsafe {
            let mut info = SHFILEINFOW::default();
            let ok = SHGetFileInfoW(
                &HSTRING::from(path),
                FILE_FLAGS_AND_ATTRIBUTES(0),
                Some(&mut info),
                std::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_ICON | SHGFI_LARGEICON,
            );
            if ok == 0 || info.hIcon.is_invalid() {
                return None;
            }
            let mut ii = ICONINFO::default();
            let got = GetIconInfo(info.hIcon, &mut ii).is_ok();
            let _ = DestroyIcon(info.hIcon);
            if !got {
                return None;
            }
            let result = (|| {
                let mut bm = BITMAP::default();
                if GetObjectW(HGDIOBJ(ii.hbmColor.0), std::mem::size_of::<BITMAP>() as i32, Some(&mut bm as *mut _ as *mut _)) == 0 {
                    return None;
                }
                let (w, h) = (bm.bmWidth, bm.bmHeight);
                let mut bi = BITMAPINFO::default();
                bi.bmiHeader = BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h, // 위에서 아래로
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                };
                let mut px = vec![0u8; (w * h * 4) as usize];
                let dc = CreateCompatibleDC(None);
                let lines = GetDIBits(dc, ii.hbmColor, 0, h as u32, Some(px.as_mut_ptr() as *mut _), &mut bi, DIB_RGB_COLORS);
                let _ = DeleteDC(dc);
                if lines == 0 {
                    return None;
                }
                let has_alpha = px.chunks(4).any(|p| p[3] != 0);
                for p in px.chunks_mut(4) {
                    p.swap(0, 2); // BGRA → RGBA
                    if !has_alpha {
                        p[3] = 255;
                    }
                }
                let mut png_bytes = Vec::new();
                {
                    let mut enc = png::Encoder::new(&mut png_bytes, w as u32, h as u32);
                    enc.set_color(png::ColorType::Rgba);
                    enc.set_depth(png::BitDepth::Eight);
                    let mut wr = enc.write_header().ok()?;
                    wr.write_image_data(&px).ok()?;
                }
                Some(png_bytes)
            })();
            let _ = DeleteObject(HGDIOBJ(ii.hbmColor.0));
            let _ = DeleteObject(HGDIOBJ(ii.hbmMask.0));
            result
        }
    }

    /// 작업 표시줄이 라이트 테마인지(트레이 아이콘 색 선택용)
    pub fn taskbar_is_light() -> bool {
        use windows::core::w;
        use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};
        let mut value = 0u32;
        let mut size = std::mem::size_of::<u32>() as u32;
        let res = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
                w!("SystemUsesLightTheme"),
                RRF_RT_REG_DWORD,
                None,
                Some(&mut value as *mut u32 as *mut _),
                Some(&mut size),
            )
        };
        res.is_ok() && value == 1
    }
}

#[cfg(not(windows))]
mod imp {
    use super::Sample;

    #[derive(Default)]
    pub struct Tracker;

    impl Tracker {
        pub fn sample(&mut self) -> Sample {
            Sample::default()
        }
    }

    pub fn taskbar_is_light() -> bool {
        false
    }

    pub fn running_programs() -> Vec<super::RunningProgram> {
        Vec::new()
    }

    pub fn icon_png(_path: &str) -> Option<Vec<u8>> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct RunningProgram {
    pub exe: String,
    pub path: String,
    pub title: String,
}

pub use imp::{icon_png, running_programs, taskbar_is_light, Tracker};
