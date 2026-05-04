/**
 * 桌面项目类型
 * 对应 Rust 端的 DesktopItem 结构
 */
export interface DesktopItem {
  /** 唯一标识：文件内容 hash */
  id: string;
  /** 显示名称 */
  name: string;
  /** 完整路径 */
  path: string;
  /** 项目类型：file | folder | shortcut */
  itemType: DesktopItemType;
  /** 图标 URL（通过 asset protocol 加载） */
  iconUrl: string | null;
  /** 文件扩展名 */
  extension: string;
  /** 文件大小（字节） */
  size: number;
  /** 修改时间 */
  modifiedAt: number;
  /** 元数据，为 AI 分类预留 */
  metadata: ItemMetadata;
}

export type DesktopItemType = 'file' | 'folder' | 'shortcut';

export interface ItemMetadata {
  extension: string;
  isExecutable: boolean;
  pathKeywords: string[];
}

/**
 * 虚拟收纳盒
 */
export interface FolderGroup {
  id: string;
  name: string;
  color: string;
  itemIds: string[];
  createdAt: number;
}

/**
 * 应用全局状态
 */
export interface AppState {
  items: DesktopItem[];
  groups: FolderGroup[];
  selectedItemIds: string[];
  isScanning: boolean;
  scanError: string | null;
}

/**
 * AI 分类建议
 */
export interface CategorySuggestion {
  categoryName: string;
  itemIds: string[];
  confidence: number;
}
