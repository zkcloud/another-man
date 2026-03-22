<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const history = ref([]);
const activeTab = ref("history");

async function loadHistory() {
  try {
    const result = await invoke("get_history", { limit: 100 });
    history.value = result;
  } catch (err) {
    console.error("Failed to load history:", err);
  }
}

function formatTime(timestamp) {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString();
}

function formatUrl(url) {
  try {
    const urlObj = new URL(url);
    return urlObj.pathname + urlObj.search;
  } catch {
    return url;
  }
}

function selectHistory(item) {
  // Emit event to parent to load this request
  emit("select", item);
}

const emit = defineEmits(["select"]);

onMounted(() => {
  loadHistory();
});

// Expose loadHistory for parent to call
defineExpose({ loadHistory });
</script>

<template>
  <aside class="sidebar">
    <div class="tabs">
      <button
        :class="['tab-btn', { active: activeTab === 'history' }]"
        @click="activeTab = 'history'"
      >
        History
      </button>
      <button
        :class="['tab-btn', { active: activeTab === 'collections' }]"
        @click="activeTab = 'collections'"
      >
        Collections
      </button>
    </div>

    <!-- History Tab -->
    <div v-show="activeTab === 'history'" class="tab-content">
      <div v-if="history.length === 0" class="empty-state">
        No history yet
      </div>
      <ul v-else class="history-list">
        <li
          v-for="item in history"
          :key="item.id"
          class="history-item"
          @click="selectHistory(item)"
        >
          <div class="history-main">
            <span :class="['method', item.method.toLowerCase()]">
              {{ item.method }}
            </span>
            <span class="url">{{ formatUrl(item.url) }}</span>
          </div>
          <div class="history-meta">
            <span
              v-if="item.response_status"
              :class="[
                'status',
                item.response_status >= 200 && item.response_status < 300
                  ? 'success'
                  : 'error',
              ]"
            >
              {{ item.response_status }}
            </span>
            <span v-if="item.time_ms" class="time">{{ item.time_ms }}ms</span>
            <span class="timestamp">{{ formatTime(item.created_at) }}</span>
          </div>
        </li>
      </ul>
    </div>

    <!-- Collections Tab -->
    <div v-show="activeTab === 'collections'" class="tab-content">
      <div class="empty-state">Collections coming soon</div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 280px;
  background: #f8f9fa;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.tabs {
  display: flex;
  border-bottom: 1px solid #e0e0e0;
  background: white;
}

.tab-btn {
  flex: 1;
  padding: 12px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 13px;
  color: #666;
  border-bottom: 2px solid transparent;
  transition: all 0.2s;
}

.tab-btn:hover {
  color: #333;
  background: #f0f0f0;
}

.tab-btn.active {
  color: #007bff;
  border-bottom-color: #007bff;
  background: white;
}

.tab-content {
  flex: 1;
  overflow-y: auto;
}

.empty-state {
  padding: 40px 20px;
  text-align: center;
  color: #999;
  font-size: 13px;
}

.history-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.history-item {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background 0.2s;
}

.history-item:hover {
  background: #e3f2fd;
}

.history-main {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}

.method {
  font-size: 11px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 3px;
  text-transform: uppercase;
}

.method.get {
  background: #d4edda;
  color: #155724;
}

.method.post {
  background: #cce5ff;
  color: #004085;
}

.method.put {
  background: #fff3cd;
  color: #856404;
}

.method.delete {
  background: #f8d7da;
  color: #721c24;
}

.method.patch {
  background: #e2e3e5;
  color: #383d41;
}

.url {
  font-size: 13px;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.history-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 11px;
}

.status {
  padding: 1px 6px;
  border-radius: 3px;
  font-weight: 500;
}

.status.success {
  background: #d4edda;
  color: #155724;
}

.status.error {
  background: #f8d7da;
  color: #721c24;
}

.time {
  color: #666;
}

.timestamp {
  color: #999;
  margin-left: auto;
}
</style>