<script setup lang="ts">
import { ref } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useDesktopStore } from "../stores/desktop";

const store = useDesktopStore();
const isRefreshing = ref(false);

const appWindow = getCurrentWebviewWindow();

function startDrag(e: MouseEvent) {
  // 只有左键才触发窗口拖动
  if (e.button !== 0) return;
  // 如果点击的是交互元素（按钮、输入框），不触发拖动
  const target = e.target as HTMLElement;
  if (target.closest("button, input, .search-box")) {
    return;
  }
  appWindow.startDragging();
}

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

async function handleMinimize() {
  await appWindow.minimize();
}

async function handleMaximize() {
  const isMax = await appWindow.isMaximized();
  if (isMax) {
    await appWindow.unmaximize();
  } else {
    await appWindow.maximize();
  }
}

async function handleClose() {
  await appWindow.close();
}
</script>

<template>
  <div class="shell">
    <!-- 标题栏：手动实现拖动，避免 data-tauri-drag-region 拦截所有鼠标事件 -->
    <header class="titlebar" @mousedown="startDrag">
      <div class="titlebar-left">
        <span class="app-icon">🪹</span>
        <span class="app-name">DeskNest</span>
      </div>

      <div class="titlebar-center">
        <div class="search-box">
          <span class="search-icon">🔍</span>
          <input
            v-model="store.searchQuery"
            type="text"
            placeholder="搜索桌面项目..."
          />
        </div>
      </div>

      <div class="titlebar-right">
        <button
          class="toolbar-btn"
          :class="{ spinning: isRefreshing }"
          title="刷新"
          @click="handleRefresh"
          @mousedown.stop
        >
          🔄
        </button>
        <button class="toolbar-btn primary" title="新建收纳盒" @click="handleCreateGroup" @mousedown.stop>
          + 新建收纳盒
        </button>

        <!-- 窗口控制按钮 -->
        <div class="window-controls">
          <button class="win-btn minimize" title="最小化" @click="handleMinimize" @mousedown.stop>−</button>
          <button class="win-btn maximize" title="最大化" @click="handleMaximize" @mousedown.stop>□</button>
          <button class="win-btn close" title="关闭" @click="handleClose" @mousedown.stop>×</button>
        </div>
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
}

.titlebar-left,
.titlebar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.titlebar-center {
  flex: 1;
  display: flex;
  justify-content: center;
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

.window-controls {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-left: 12px;
  padding-left: 12px;
  border-left: 1px solid rgba(255, 255, 255, 0.08);
}

.win-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.5);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.win-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.85);
}

.win-btn.minimize:hover {
  background: rgba(255, 193, 7, 0.2);
  color: #ffc107;
}

.win-btn.maximize:hover {
  background: rgba(76, 175, 80, 0.2);
  color: #81c784;
}

.win-btn.close:hover {
  background: rgba(239, 83, 80, 0.2);
  color: #ef5350;
}

.shell-body {
  flex: 1;
  overflow: hidden;
}
</style>
