<script setup lang="ts">
import { ref } from "vue";
import { useDesktopStore } from "../stores/desktop";

const store = useDesktopStore();
const searchQuery = ref("");
const isRefreshing = ref(false);

async function handleRefresh() {
  if (isRefreshing.value) return;
  isRefreshing.value = true;
  await store.scanDesktop();
  isRefreshing.value = false;
}

async function handleCreateGroup() {
  const name = prompt("请输入收纳盒名称：", "新建收纳盒");
  if (name?.trim()) {
    await store.createGroup(name.trim());
  }
}
</script>

<template>
  <div class="shell">
    <!-- 标题栏 -->
    <header class="titlebar" data-tauri-drag-region>
      <div class="titlebar-left">
        <span class="app-icon">🪹</span>
        <span class="app-name">DeskNest</span>
      </div>

      <div class="titlebar-center">
        <div class="search-box">
          <span class="search-icon">🔍</span>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索桌面项目..."
            @input="store.searchItems(searchQuery)"
          />
        </div>
      </div>

      <div class="titlebar-right">
        <button
          class="toolbar-btn"
          :class="{ spinning: isRefreshing }"
          title="刷新"
          @click="handleRefresh"
        >
          🔄
        </button>
        <button class="toolbar-btn primary" title="新建收纳盒" @click="handleCreateGroup">
          + 新建收纳盒
        </button>
      </div>
    </header>

    <!-- 内容区 -->
    <div class="shell-body">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.shell {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.titlebar {
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  background: rgba(40, 40, 40, 0.9);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
  -webkit-app-region: drag;
  app-region: drag;
}

.titlebar-left,
.titlebar-right {
  display: flex;
  align-items: center;
  gap: 10px;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.titlebar-center {
  flex: 1;
  display: flex;
  justify-content: center;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.app-icon {
  font-size: 18px;
}

.app-name {
  font-size: 14px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.9);
  letter-spacing: 0.3px;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 6px 12px;
  width: 320px;
  transition: all 0.2s ease;
}

.search-box:focus-within {
  background: rgba(255, 255, 255, 0.12);
  border-color: rgba(255, 255, 255, 0.2);
}

.search-icon {
  font-size: 12px;
  opacity: 0.5;
}

.search-box input {
  background: transparent;
  border: none;
  outline: none;
  color: rgba(255, 255, 255, 0.9);
  font-size: 13px;
  width: 100%;
}

.search-box input::placeholder {
  color: rgba(255, 255, 255, 0.35);
}

.toolbar-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  border-radius: 6px;
  border: none;
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.8);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  -webkit-app-region: no-drag;
  app-region: no-drag;
}

.toolbar-btn:hover {
  background: rgba(255, 255, 255, 0.15);
}

.toolbar-btn.primary {
  background: rgba(76, 175, 80, 0.2);
  color: #81c784;
}

.toolbar-btn.primary:hover {
  background: rgba(76, 175, 80, 0.3);
}

.spinning {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.shell-body {
  flex: 1;
  overflow: hidden;
}
</style>
