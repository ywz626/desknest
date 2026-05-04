import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@tauri-apps/plugin-store';
import type { DesktopItem, FolderGroup } from '../types/desktop';

const STORE_PATH = 'desknest-store.json';

interface DesktopState {
  items: DesktopItem[];
  groups: FolderGroup[];
  isScanning: boolean;
  scanError: string | null;
  selectedItemIds: string[];
  store: Store | null;
}

/**
 * 桌面状态管理 Store
 * 负责：扫描桌面、管理分组、持久化配置
 */
export const useDesktopStore = defineStore('desktop', {
  state: (): DesktopState => ({
    items: [],
    groups: [],
    isScanning: false,
    scanError: null,
    selectedItemIds: [],
    store: null,
  }),

  getters: {
    /** 未分组的桌面项目 */
    ungroupedItems: (state): DesktopItem[] => {
      const groupedIds = new Set(state.groups.flatMap((g) => g.itemIds));
      return state.items.filter((item) => !groupedIds.has(item.id));
    },

    /** 按名称搜索 */
    searchItems:
      (state) =>
      (query: string): DesktopItem[] => {
        if (!query.trim()) return state.items;
        const lower = query.toLowerCase();
        return state.items.filter((item) =>
          item.name.toLowerCase().includes(lower)
        );
      },
  },

  actions: {
    /** 初始化 Tauri Store */
    async initStore() {
      this.store = await Store.load(STORE_PATH);
      await this.loadPersistedData();
    },

    /** 从持久化存储加载分组数据 */
    async loadPersistedData() {
      if (!this.store) return;
      const groups = await this.store.get<FolderGroup[]>('groups');
      if (groups) {
        this.groups = groups;
      }
    },

    /** 保存分组数据到持久化存储 */
    async saveGroups() {
      if (!this.store) return;
      await this.store.set('groups', this.groups);
      await this.store.save();
    },

    /** 扫描桌面 */
    async scanDesktop() {
      this.isScanning = true;
      this.scanError = null;
      try {
        const items = await invoke<DesktopItem[]>('scan_desktop_items');
        this.items = items;
        console.log('[DesktopStore] 扫描完成:', items.length, '个项目');
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        this.scanError = message;
        console.error('[DesktopStore] 扫描失败:', message);
      } finally {
        this.isScanning = false;
      }
    },

    /** 创建新分组 */
    async createGroup(name: string, color?: string) {
      const group: FolderGroup = {
        id: crypto.randomUUID(),
        name,
        color: color || this.getRandomColor(),
        itemIds: [],
        createdAt: Date.now(),
      };
      this.groups.push(group);
      await this.saveGroups();
      return group;
    },

    /** 删除分组 */
    async deleteGroup(groupId: string) {
      this.groups = this.groups.filter((g) => g.id !== groupId);
      await this.saveGroups();
    },

    /** 将项目移入分组 */
    async addItemsToGroup(groupId: string, itemIds: string[]) {
      const group = this.groups.find((g) => g.id === groupId);
      if (!group) return;
      for (const id of itemIds) {
        if (!group.itemIds.includes(id)) {
          group.itemIds.push(id);
        }
      }
      await this.saveGroups();
    },

    /** 从分组移除项目 */
    async removeItemFromGroup(groupId: string, itemId: string) {
      const group = this.groups.find((g) => g.id === groupId);
      if (!group) return;
      group.itemIds = group.itemIds.filter((id) => id !== itemId);
      await this.saveGroups();
    },

    /** 重命名分组 */
    async renameGroup(groupId: string, newName: string) {
      const group = this.groups.find((g) => g.id === groupId);
      if (group) {
        group.name = newName;
        await this.saveGroups();
      }
    },

    /** 选择/取消选择项目 */
    toggleSelection(itemId: string) {
      const idx = this.selectedItemIds.indexOf(itemId);
      if (idx >= 0) {
        this.selectedItemIds.splice(idx, 1);
      } else {
        this.selectedItemIds.push(itemId);
      }
    },

    /** 清空选择 */
    clearSelection() {
      this.selectedItemIds = [];
    },

    getRandomColor(): string {
      const colors = [
        '#FF6B6B',
        '#4ECDC4',
        '#45B7D1',
        '#96CEB4',
        '#FFEAA7',
        '#DDA0DD',
        '#98D8C8',
        '#F7DC6F',
      ];
      return colors[Math.floor(Math.random() * colors.length)];
    },
  },
});
