<script setup lang="ts">
import { computed } from "vue";
import { openPath } from "@tauri-apps/plugin-opener";
import type { DesktopItem } from "../types/desktop";

const props = defineProps<{
  item: DesktopItem;
  selected?: boolean;
}>();

const emit = defineEmits<{
  select: [id: string];
  dragstart: [event: DragEvent, item: DesktopItem];
}>();

const iconClass = computed(() => {
  const map: Record<string, string> = {
    exe: "📦",
    txt: "📄",
    md: "📝",
    pdf: "📕",
    jpg: "🖼️",
    jpeg: "🖼️",
    png: "🖼️",
    gif: "🖼️",
    mp4: "🎬",
    mp3: "🎵",
    zip: "🗜️",
    rar: "🗜️",
    doc: "📘",
    docx: "📘",
    xls: "📗",
    xlsx: "📗",
    ppt: "📙",
    pptx: "📙",
    lnk: "🔗",
  };
  return map[props.item.extension.toLowerCase()] || "📄";
});

async function handleOpen() {
  emit("select", props.item.id);
  try {
    await openPath(props.item.path);
  } catch (err) {
    console.error("无法打开文件:", props.item.path, err);
  }
}

function handleDragStart(e: DragEvent) {
  emit("dragstart", e, props.item);
}
</script>

<template>
  <div
    class="desktop-icon"
    :class="{ selected }"
    draggable="true"
    @click="handleOpen"
    @dragstart="handleDragStart"
  >
    <div class="icon-preview">
      <span class="icon-emoji">{{ iconClass }}</span>
    </div>
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

.icon-preview {
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.06);
  font-size: 28px;
  transition: transform 0.15s ease;
}

.desktop-icon:hover .icon-preview {
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
