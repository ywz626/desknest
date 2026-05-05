<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useDesktopStore } from "../stores/desktop";
import DesktopItemIcon from "./DesktopItemIcon.vue";

const store = useDesktopStore();
const expandedGroups = ref<Set<string>>(new Set());

const itemsById = computed(() => new Map(store.items.map((item) => [item.id, item])));

function toggleGroup(groupId: string) {
  if (expandedGroups.value.has(groupId)) {
    expandedGroups.value.delete(groupId);
  } else {
    expandedGroups.value.add(groupId);
  }
}

function handleDragOver(event: DragEvent) {
  event.preventDefault();
  event.dataTransfer!.dropEffect = "move";
}

async function handleDrop(event: DragEvent, groupId: string) {
  event.preventDefault();
  const itemId = event.dataTransfer?.getData("text/plain");
  if (itemId) {
    await store.addItemsToGroup(groupId, [itemId]);
  }
}

async function handleRemoveItem(groupId: string, itemId: string) {
  await store.removeItemFromGroup(groupId, itemId);
}

function getItemById(id: string) {
  return itemsById.value.get(id);
}

async function handleOpenItem(itemId: string) {
  const item = itemsById.value.get(itemId);
  if (!item) return;
  try {
    await invoke("open_item", { path: item.path });
  } catch (err) {
    console.error("无法打开文件:", item.path, err);
  }
}

async function handleDeleteGroup(groupId: string) {
  const group = store.groups.find((g) => g.id === groupId);
  if (!group) return;
  const confirmed = confirm(`确定要删除收纳盒「${group.name}」吗？盒内项目会回到桌面网格。`);
  if (confirmed) {
    await store.deleteGroup(groupId);
  }
}

async function handleRenameGroup(groupId: string) {
  const group = store.groups.find((g) => g.id === groupId);
  if (!group) return;
  const newName = prompt("请输入新名称：", group.name);
  if (newName?.trim()) {
    await store.renameGroup(groupId, newName.trim());
  }
}
</script>

<template>
  <div class="group-panel">
    <div class="panel-header">
      <span class="panel-title">📁 收纳盒</span>
      <span class="panel-count">{{ store.groups.length }}</span>
    </div>

    <div class="group-list">
      <div
        v-for="group in store.groups"
        :key="group.id"
        class="group-card"
        :style="{ borderLeftColor: group.color }"
        @dragover="handleDragOver"
        @drop="handleDrop($event, group.id)"
      >
        <div class="group-header" @click="toggleGroup(group.id)">
          <span class="group-toggle">{{ expandedGroups.has(group.id) ? "▼" : "▶" }}</span>
          <span class="group-name">{{ group.name }}</span>
          <span class="group-badge">{{ group.itemIds.length }}</span>

          <div class="group-actions" @click.stop>
            <button
              class="action-btn"
              title="重命名"
              @click="handleRenameGroup(group.id)"
            >
              ✏️
            </button>
            <button
              class="action-btn delete"
              title="删除"
              @click="handleDeleteGroup(group.id)"
            >
              🗑️
            </button>
          </div>
        </div>

        <div v-if="expandedGroups.has(group.id)" class="group-items">
          <div
            v-for="itemId in group.itemIds"
            :key="itemId"
            class="group-item"
            @dblclick="handleOpenItem(itemId)"
          >
            <DesktopItemIcon
              v-if="getItemById(itemId)"
              :item="getItemById(itemId)!"
              variant="compact"
            />
            <span v-else class="missing-item-icon">?</span>
            <span class="item-name">{{ getItemById(itemId)?.name || "未知文件" }}</span>
            <button class="item-remove" @click.stop="handleRemoveItem(group.id, itemId)">×</button>
          </div>

          <div v-if="group.itemIds.length === 0" class="group-empty">
            拖拽图标到此处
          </div>
        </div>
      </div>

      <div v-if="store.groups.length === 0" class="empty-groups">
        <p>暂无收纳盒</p>
        <p class="hint">点击上方「+ 新建收纳盒」创建</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.group-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-bottom: 12px;
  margin-bottom: 12px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.panel-title {
  font-size: 13px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.7);
}

.panel-count {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.4);
  background: rgba(255, 255, 255, 0.08);
  padding: 2px 8px;
  border-radius: 10px;
}

.group-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.group-card {
  background: rgba(255, 255, 255, 0.04);
  border-radius: 10px;
  border-left: 3px solid;
  overflow: hidden;
  transition: background 0.2s;
}

.group-card:hover {
  background: rgba(255, 255, 255, 0.08);
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  cursor: pointer;
  user-select: none;
}

.group-toggle {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.4);
  width: 14px;
}

.group-name {
  flex: 1;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.85);
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.group-badge {
  font-size: 10px;
  color: rgba(255, 255, 255, 0.4);
  background: rgba(255, 255, 255, 0.08);
  padding: 2px 7px;
  border-radius: 8px;
}

/* Group action buttons */
.group-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.15s;
}

.group-header:hover .group-actions {
  opacity: 1;
}

.action-btn {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 5px;
  border: none;
  background: transparent;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.15s;
  padding: 0;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.12);
}

.action-btn.delete:hover {
  background: rgba(239, 83, 80, 0.2);
}

.group-items {
  padding: 4px 12px 12px 34px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.group-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 6px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  transition: background 0.15s;
}

.group-item:hover {
  background: rgba(255, 255, 255, 0.06);
}

.missing-item-icon {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.06);
  color: rgba(255, 255, 255, 0.35);
  font-size: 12px;
}

.item-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.item-remove {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.3);
  font-size: 14px;
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
}

.group-item:hover .item-remove {
  opacity: 1;
}

.item-remove:hover {
  background: rgba(239, 83, 80, 0.2);
  color: #ef5350;
}

.group-empty {
  padding: 12px;
  text-align: center;
  font-size: 11px;
  color: rgba(255, 255, 255, 0.25);
  font-style: italic;
}

.empty-groups {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: rgba(255, 255, 255, 0.3);
  font-size: 13px;
  text-align: center;
  gap: 6px;
}

.empty-groups .hint {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.2);
}
</style>
