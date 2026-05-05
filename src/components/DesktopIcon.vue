<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import DesktopItemIcon from "./DesktopItemIcon.vue";
import type { DesktopItem } from "../types/desktop";

const props = defineProps<{
  item: DesktopItem;
  selected?: boolean;
}>();

const emit = defineEmits<{
  select: [id: string];
  dragstart: [event: DragEvent, item: DesktopItem];
}>();

// 防止拖拽结束后误触发 click
let dragStarted = false;

// 手动双击检测：draggable 元素的 @dblclick 在 WebView 中不可靠
let clickCount = 0;
let clickTimer: ReturnType<typeof setTimeout> | null = null;
const DOUBLE_CLICK_DELAY = 400; // ms，与 Windows 默认双击速度接近

function handleClick() {
  // 拖拽结束后浏览器会补发一个 click，此时 dragStarted 为 true，直接忽略
  if (dragStarted) {
    dragStarted = false;
    return;
  }

  clickCount++;

  if (clickCount === 1) {
    // 第一次点击：启动定时器，超时则判定为单击（选择）
    clickTimer = setTimeout(() => {
      emit("select", props.item.id);
      clickCount = 0;
    }, DOUBLE_CLICK_DELAY);
  } else if (clickCount === 2) {
    // 第二次点击：清除定时器，判定为双击（打开）
    if (clickTimer) {
      clearTimeout(clickTimer);
      clickTimer = null;
    }
    clickCount = 0;
    openItem();
  }
}

async function openItem() {
  try {
    await invoke("open_item", { path: props.item.path });
  } catch (err) {
    console.error("无法打开文件:", props.item.path, err);
  }
}

function handleDragStart(e: DragEvent) {
  dragStarted = true;
  // 拖拽开始时如果有未决的单击定时器，立即取消，避免拖拽后误触发选择
  if (clickTimer) {
    clearTimeout(clickTimer);
    clickTimer = null;
  }
  clickCount = 0;
  emit("dragstart", e, props.item);
}
</script>

<template>
  <div
    class="desktop-icon"
    :class="{ selected }"
    draggable="true"
    @click="handleClick"
    @dragstart="handleDragStart"
  >
    <DesktopItemIcon :item="item" variant="desktop" />
    <div class="icon-label" :title="item.name">
      {{ item.name }}
    </div>
  </div>
</template>

<style scoped>
.desktop-icon {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px 8px;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.15s ease;
  width: 88px;
  user-select: none;
}

.desktop-icon:hover {
  background: rgba(255, 255, 255, 0.08);
}

.desktop-icon.selected {
  background: rgba(76, 175, 80, 0.2);
  box-shadow: 0 0 0 2px rgba(76, 175, 80, 0.4);
}

.desktop-icon:hover :deep(.item-icon-preview.desktop) {
  transform: scale(1.08);
}

.icon-label {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.75);
  text-align: center;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  word-break: break-all;
  line-height: 1.3;
}
</style>
