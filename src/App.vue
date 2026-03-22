<script setup>
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import RequestBuilder from "./components/RequestBuilder.vue";

const sidebarRef = ref(null);

function onRequestSent() {
  // Refresh history after request is sent
  if (sidebarRef.value) {
    sidebarRef.value.loadHistory();
  }
}
</script>

<template>
  <div class="app">
    <header class="app-header">
      <h1>Another Man</h1>
      <p class="subtitle">本地优先 API 测试工具</p>
    </header>
    <div class="main-content">
      <Sidebar ref="sidebarRef" />
      <main class="content">
        <RequestBuilder @request-sent="onRequestSent" />
      </main>
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
  padding: 16px 24px;
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
</style>