<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { DesktopItem } from "../types/desktop";

const props = withDefaults(
  defineProps<{
    item: DesktopItem;
    variant?: "desktop" | "compact";
  }>(),
  {
    variant: "desktop",
  }
);

const realIconUrl = ref<string | null>(null);

onMounted(async () => {
  if (props.item.iconUrl) {
    realIconUrl.value = props.item.iconUrl;
    return;
  }

  try {
    const iconDataUrl = await invoke<string>("extract_file_icon", {
      path: props.item.path,
    });
    if (iconDataUrl) {
      realIconUrl.value = iconDataUrl;
    }
  } catch (err) {
    console.error("图标提取失败:", props.item.path, err);
  }
});

const fallbackIcon = computed(() => {
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
</script>

<template>
  <div class="item-icon-preview" :class="variant">
    <img
      v-if="realIconUrl"
      :src="realIconUrl"
      class="item-icon-img"
      draggable="false"
      alt=""
    />
    <span v-else class="item-icon-emoji">{{ fallbackIcon }}</span>
  </div>
</template>

<style scoped>
.item-icon-preview {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  background: rgba(255, 255, 255, 0.06);
}

.item-icon-preview.desktop {
  width: 56px;
  height: 56px;
  border-radius: 14px;
  font-size: 28px;
  transition: transform 0.15s ease;
}

.item-icon-preview.compact {
  width: 22px;
  height: 22px;
  border-radius: 6px;
  font-size: 13px;
}

.item-icon-img {
  object-fit: contain;
  pointer-events: none;
}

.item-icon-preview.desktop .item-icon-img {
  width: 36px;
  height: 36px;
}

.item-icon-preview.compact .item-icon-img {
  width: 16px;
  height: 16px;
}

.item-icon-emoji {
  line-height: 1;
}
</style>
