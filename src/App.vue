<script setup>
import RequestBuilder from "./components/RequestBuilder.vue";
</script>

<template>
  <main class="app">
    <header class="app-header">
      <h1>Another Man</h1>
      <p class="subtitle">本地优先 API 测试工具</p>
    </header>
    <RequestBuilder />
  </main>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  background: #f5f5f5;
  color: #333;
}

.app {
  min-height: 100vh;
}

.app-header {
  background: #1a1a2e;
  color: white;
  padding: 20px;
  text-align: center;
}

.app-header h1 {
  font-size: 28px;
  font-weight: 600;
  margin-bottom: 8px;
}

.app-header .subtitle {
  font-size: 14px;
  color: #888;
}
</style>