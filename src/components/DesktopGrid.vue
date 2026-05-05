<script setup lang="ts">
import { computed } from "vue";
import { useDesktopStore } from "../stores/desktop";
import DesktopIcon from "./DesktopIcon.vue";
import type { DesktopItem } from "../types/desktop";

const store = useDesktopStore();

const displayItems = computed(() => store.filteredUngroupedItems);

function handleSelect(id: string) {
  store.toggleSelection(id);
}

function handleDragStart(event: DragEvent, item: DesktopItem) {
  event.dataTransfer?.setData("text/plain", item.id);
  event.dataTransfer!.effectAllowed = "move";
}
</script>

<template>
  <div class="grid-container">
    <div v-if="store.isScanning" class="status-message">
      <span class="spinner">🔄</span>
      <p>正在扫描桌面...</p>
    </div>

    <div v-else-if="store.scanError" class="status-message error">
      <p>扫描失败: {{ store.scanError }}</p>
      <button class="retry-btn" @click="store.scanDesktop">重试</button>
    </div>

    <div v-else-if="displayItems.length === 0" class="status-message">
      <p>桌面上暂无项目</p>
    </div>

    <div v-else class="grid">
      <DesktopIcon
        v-for="item in displayItems"
        :key="item.id"
        :item="item"
        :selected="store.selectedItemIds.includes(item.id)"
        @select="handleSelect"
        @dragstart="handleDragStart"
      />
    </div>
  </div>
</template>

<style scoped>
.grid-container {
  width: 100%;
  height: 100%;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(88px, 1fr));
  gap: 16px;
  padding: 8px;
}

.status-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: rgba(255, 255, 255, 0.5);
  font-size: 14px;
  gap: 12px;
}

.status-message.error {
  color: #ef5350;
}

.spinner {
  font-size: 24px;
  animation: spin 1s linear infinite;
}

.retry-btn {
  padding: 6px 16px;
  border-radius: 6px;
  border: none;
  background: rgba(239, 83, 80, 0.2);
  color: #ef5350;
  font-size: 13px;
  cursor: pointer;
  transition: background 0.2s;
}

.retry-btn:hover {
  background: rgba(239, 83, 80, 0.3);
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
