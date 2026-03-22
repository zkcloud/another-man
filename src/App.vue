<script setup>
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import RequestBuilder from "./components/RequestBuilder.vue";
import EnvironmentManager from "./components/EnvironmentManager.vue";
import ExportImport from "./components/ExportImport.vue";

const sidebarRef = ref(null);
const requestBuilderRef = ref(null);
const envManagerRef = ref(null);

const showEnvironments = ref(false);
const activeEnvName = ref(null);

function onRequestSent() {
  if (sidebarRef.value) {
    sidebarRef.value.loadHistory();
    sidebarRef.value.loadCollections();
  }
}

function onSelectRequest(savedRequest) {
  if (requestBuilderRef.value) {
    requestBuilderRef.value.loadRequest(savedRequest);
  }
}

function onEnvironmentChanged() {
  loadActiveEnvironment();
}

function onCollectionsChanged() {
  if (sidebarRef.value) {
    sidebarRef.value.loadCollections();
  }
}

async function loadActiveEnvironment() {
  try {
    const { getActiveEnvironment } = await import("./api/environments.js");
    const env = await getActiveEnvironment();
    activeEnvName.value = env?.name || null;
  } catch (err) {
    console.error("Failed to load active environment:", err);
  }
}

loadActiveEnvironment();
</script>

<template>
  <div class="app">
    <header class="app-header">
      <div class="header-left">
        <h1>Another Man</h1>
        <p class="subtitle">本地优先 API 测试工具</p>
      </div>
      <div class="header-right">
        <ExportImport @collections-changed="onCollectionsChanged" />
        <button
          :class="['env-btn', { active: activeEnvName }]"
          @click="showEnvironments = !showEnvironments"
        >
          {{ activeEnvName || "No Environment" }}
        </button>
      </div>
    </header>
    
    <div class="main-content">
      <Sidebar ref="sidebarRef" @select-request="onSelectRequest" />
      <main class="content">
        <RequestBuilder ref="requestBuilderRef" @request-sent="onRequestSent" />
      </main>
      
      <aside v-if="showEnvironments" class="env-sidebar">
        <EnvironmentManager
          ref="envManagerRef"
          @environment-changed="onEnvironmentChanged"
        />
      </aside>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    "Helvetica Neue", Arial, sans-serif;
  background: #f5f5f5;
  color: #333;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-header {
  background: #1a1a2e;
  color: white;
  padding: 12px 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.app-header h1 {
  font-size: 20px;
  font-weight: 600;
}

.app-header .subtitle {
  font-size: 13px;
  color: #888;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.env-btn {
  padding: 8px 16px;
  background: #2d2d44;
  color: #888;
  border: 1px solid #3d3d54;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
}

.env-btn.active {
  background: #28a745;
  color: white;
  border-color: #28a745;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.env-sidebar {
  width: 350px;
  background: #f8f9fa;
  border-left: 1px solid #e0e0e0;
  overflow-y: auto;
}
</style>