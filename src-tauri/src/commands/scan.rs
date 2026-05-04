use crate::models::desktop_item::DesktopItem;
use crate::services::scanner::scan_desktop;

/// 扫描桌面目录，返回所有桌面项目列表
/// 这是前端调用的核心命令
#[tauri::command]
pub fn scan_desktop_items() -> Result<Vec<DesktopItem>, String> {
    log::info!("收到桌面扫描请求");
    scan_desktop()
}
