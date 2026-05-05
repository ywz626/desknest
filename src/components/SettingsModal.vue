<script setup lang="ts">
import { ref, onMounted } from "vue";
import { isEnabled, enable, disable } from "@tauri-apps/plugin-autostart";

const show = defineModel<boolean>("show", { required: true });

const autoStart = ref(false);
const loading = ref(false);

onMounted(async () => {
  try {
    autoStart.value = await isEnabled();
  } catch (err) {
    console.error("[Settings] 获取开机自启状态失败:", err);
  }
});

async function toggleAutoStart() {
  if (loading.value) return;
  loading.value = true;
  try {
    if (autoStart.value) {
      await disable();
      autoStart.value = false;
    } else {
      await enable();
      autoStart.value = true;
    }
  } catch (err) {
    console.error("[Settings] 切换开机自启失败:", err);
  } finally {
    loading.value = false;
  }
}

function close() {
  show.value = false;
}
</script>

<template>
  <transition name="fade">
    <div v-if="show" class="modal-overlay" @click.self="close">
      <div class="modal-panel">
        <div class="modal-header">
          <span class="modal-title">⚙️ 设置</span>
          <button class="modal-close" @click="close">×</button>
        </div>

        <div class="modal-body">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">开机自启</span>
              <span class="setting-desc">登录 Windows 时自动启动 DeskNest</span>
            </div>
            <button
              class="toggle-btn"
              :class="{ active: autoStart }"
              :disabled="loading"
              @click="toggleAutoStart"
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
}

.modal-panel {
  width: 400px;
  background: rgba(30, 30, 30, 0.95);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5);
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.modal-title {
  font-size: 15px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.9);
}

.modal-close {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.4);
  font-size: 18px;
  cursor: pointer;
  transition: all 0.15s;
}

.modal-close:hover {
  background: rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.85);
}

.modal-body {
  padding: 12px 20px 20px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.setting-label {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.85);
  font-weight: 500;
}

.setting-desc {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
}

/* Toggle switch */
.toggle-btn {
  width: 44px;
  height: 24px;
  border-radius: 12px;
  border: none;
  background: rgba(255, 255, 255, 0.15);
  cursor: pointer;
  position: relative;
  transition: background 0.2s ease;
  padding: 0;
}

.toggle-btn.active {
  background: #4caf50;
}

.toggle-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: #fff;
  transition: transform 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.toggle-btn.active .toggle-thumb {
  transform: translateX(20px);
}

/* Fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
