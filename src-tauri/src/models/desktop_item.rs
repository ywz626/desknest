use serde::{Deserialize, Serialize};

/// 桌面项目类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopItem {
    /// 唯一标识：文件内容 hash
    pub id: String,
    /// 显示名称
    pub name: String,
    /// 完整路径
    pub path: String,
    /// 项目类型
    #[serde(rename = "itemType")]
    pub item_type: DesktopItemType,
    /// 图标 URL（通过 asset protocol 加载）
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    /// 文件扩展名
    pub extension: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 修改时间（Unix 时间戳）
    #[serde(rename = "modifiedAt")]
    pub modified_at: u64,
    /// 元数据，为 AI 分类预留
    pub metadata: ItemMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DesktopItemType {
    File,
    Folder,
    Shortcut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMetadata {
    pub extension: String,
    pub is_executable: bool,
    pub path_keywords: Vec<String>,
}

/// 虚拟收纳盒
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderGroup {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(rename = "itemIds")]
    pub item_ids: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}
