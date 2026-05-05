use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// 打开指定路径的文件或应用
/// 使用 Windows ShellExecuteW，比前端插件更可靠，支持 .lnk 快捷方式直接解析打开
#[tauri::command]
pub async fn open_item(path: String) -> Result<(), String> {
    let wide_path: Vec<u16> = OsStr::new(&path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let ret = unsafe {
        windows::Win32::UI::Shell::ShellExecuteW(
            None,
            None,
            windows::core::PCWSTR(wide_path.as_ptr()),
            None,
            None,
            windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
        )
    };

    // ShellExecuteW 返回值 > 32 表示成功，否则为错误码
    if ret.0 as isize > 32 {
        Ok(())
    } else {
        Err(format!("ShellExecute 失败，错误码: {:?}", ret.0))
    }
}
