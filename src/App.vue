<script setup lang="ts">
import { onMounted } from "vue";
import { useDesktopStore } from "./stores/desktop";
import AppShell from "./components/AppShell.vue";
import DesktopGrid from "./components/DesktopGrid.vue";
import FolderGroupPanel from "./components/FolderGroupPanel.vue";

const store = useDesktopStore();

onMounted(async () => {
  await store.initStore();
  await store.scanDesktop();
});
</script>

<template>
  <AppShell>
    <div class="app-layout">
      <aside class="sidebar">
        <FolderGroupPanel />
      </aside>
      <main class="main-content">
        <DesktopGrid />
      </main>
    </div>
  </AppShell>
</template>

<style>
/* 全局重置与基础样式 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: "Segoe UI Variable", "Segoe UI", system-ui, -apple-system,
    sans-serif;
  background: transparent;
  overflow: hidden;
  user-select: none;
}

#app {
  width: 100vw;
  height: 100vh;
}

.app-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.sidebar {
  width: 280px;
  min-width: 280px;
  background: rgba(32, 32, 32, 0.85);
  backdrop-filter: blur(20px);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-content {
  flex: 1;
  background: rgba(25, 25, 25, 0.9);
  backdrop-filter: blur(20px);
  overflow: auto;
  padding: 24px;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.25);
}
</style>
