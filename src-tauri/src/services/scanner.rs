use crate::models::desktop_item::{DesktopItem, DesktopItemType, ItemMetadata};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

/// 扫描桌面目录，返回所有桌面项目
/// 业务规则：只扫描当前用户 Desktop，不扫描 Public Desktop
pub fn scan_desktop() -> Result<Vec<DesktopItem>, String> {
    let desktop_path = get_desktop_path()?;
    let mut items = Vec::new();

    let entries = match fs::read_dir(&desktop_path) {
        Ok(e) => e,
        Err(err) => {
            log::error!("无法读取桌面目录: path={}, error={}", desktop_path, err);
            return Err(format!("无法读取桌面目录: {}", err));
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(item) = path_to_item(&path) {
            items.push(item);
        }
    }

    log::info!("桌面扫描完成: 共 {} 个项目", items.len());
    Ok(items)
}

/// 获取桌面路径
/// 反直觉说明：使用 %USERPROFILE%\Desktop 而非 SHGetKnownFolderPath，
/// 因为 Tauri v2 在 Windows 上通过 env 获取更稳定，且本项目 Phase 1 只支持标准配置。
/// 后续企业环境（OneDrive 重定向）再迁移到 SHGetKnownFolderPath。
fn get_desktop_path() -> Result<String, String> {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map_err(|_| "无法获取用户主目录".to_string())?;
    let desktop = Path::new(&home).join("Desktop");
    Ok(desktop.to_string_lossy().to_string())
}

/// 将单个路径转换为 DesktopItem
fn path_to_item(path: &Path) -> Option<DesktopItem> {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(err) => {
            log::warn!("无法读取文件元数据: path={}, error={}", path.display(), err);
            return None;
        }
    };

    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let extension = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .to_lowercase();

    let item_type = if metadata.is_dir() {
        DesktopItemType::Folder
    } else if extension == "lnk" {
        DesktopItemType::Shortcut
    } else {
        DesktopItemType::File
    };

    let is_executable = extension == "exe" || extension == "bat" || extension == "cmd";

    let id = compute_file_id(path, &metadata);

    let modified_at = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let path_keywords: Vec<String> = path
        .ancestors()
        .filter_map(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .collect();

    Some(DesktopItem {
        id,
        name,
        path: path.to_string_lossy().to_string(),
        item_type,
        icon_url: None, // Phase 1 先使用扩展名映射图标，真实图标提取后续实现
        extension: extension.clone(),
        size: metadata.len(),
        modified_at,
        metadata: ItemMetadata {
            extension,
            is_executable,
            path_keywords,
        },
    })
}

/// 计算文件唯一 ID：基于文件路径 + 修改时间 + 大小的组合 hash
/// 为什么不直接用内容 hash？因为大文件（如视频、ISO）读取全部内容太慢。
/// 路径+时间+大小的组合在绝大多数场景下足够唯一，且性能可接受。
fn compute_file_id(path: &Path, metadata: &fs::Metadata) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    if let Ok(modified) = metadata.modified() {
        if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
            hasher.update(duration.as_secs().to_le_bytes());
        }
    }
    hasher.update(metadata.len().to_le_bytes());
    format!("{:.16}", hex::encode(hasher.finalize()))
}

/// 为扩展名生成默认图标路径（Phase 1 简化方案）
/// 后续替换为真实 Windows 图标提取
pub fn get_default_icon_for_extension(ext: &str) -> String {
    match ext {
        "exe" | "msi" => "icons/app-icon.png".to_string(),
        "txt" | "md" | "log" => "icons/text-icon.png".to_string(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" => "icons/image-icon.png".to_string(),
        "mp4" | "avi" | "mkv" | "mov" => "icons/video-icon.png".to_string(),
        "mp3" | "wav" | "flac" | "aac" => "icons/audio-icon.png".to_string(),
        "pdf" => "icons/pdf-icon.png".to_string(),
        "zip" | "rar" | "7z" | "tar" | "gz" => "icons/archive-icon.png".to_string(),
        "doc" | "docx" => "icons/word-icon.png".to_string(),
        "xls" | "xlsx" => "icons/excel-icon.png".to_string(),
        "ppt" | "pptx" => "icons/powerpoint-icon.png".to_string(),
        "lnk" => "icons/shortcut-icon.png".to_string(),
        _ => "icons/file-icon.png".to_string(),
    }
}
