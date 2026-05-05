/// 提取指定文件的 Windows 图标并返回 base64 编码的 PNG 数据 URL
/// 前端可直接作为 img src 使用
#[tauri::command]
pub fn extract_file_icon(path: String) -> Result<String, String> {
    log::info!("提取图标: path={}", path);
    crate::services::icon_extractor::extract_file_icon_base64(&path)
}
