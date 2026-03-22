<script setup>
import { ref, computed } from "vue";
import { formatBody, detectContentType } from "../utils/formatter.js";

const props = defineProps({
  response: {
    type: Object,
    default: null,
  },
});

const activeTab = ref("body");

const formattedBody = computed(() => {
  if (!props.response) return "";
  const contentType = props.response.headers?.find(
    (h) => h[0].toLowerCase() === "content-type"
  )?.[1];
  return formatBody(props.response.body, contentType);
});

const contentType = computed(() => {
  if (!props.response) return "text";
  const ct = props.response.headers?.find(
    (h) => h[0].toLowerCase() === "content-type"
  )?.[1];
  return detectContentType(ct, props.response.body);
});

const responseHeaders = computed(() => {
  if (!props.response) return [];
  return props.response.headers || [];
});

const statusClass = computed(() => {
  if (!props.response) return "";
  const status = props.response.status;
  if (status >= 200 && status < 300) return "success";
  if (status >= 300 && status < 400) return "redirect";
  if (status >= 400 && status < 500) return "client-error";
  if (status >= 500) return "server-error";
  return "";
});
</script>

<template>
  <div v-if="response" class="response-viewer">
    <!-- Response Meta -->
    <div class="response-meta">
      <span :class="['status-badge', statusClass]">
        {{ response.status }} {{ response.status_text }}
      </span>
      <span class="meta-item">
        <span class="meta-label">Time:</span>
        {{ response.time_ms }}ms
      </span>
      <span class="meta-item">
        <span class="meta-label">Size:</span>
        {{ response.size_bytes }} bytes
      </span>
      <span class="meta-item">
        <span class="meta-label">Type:</span>
        {{ contentType }}
      </span>
    </div>

    <!-- Tabs -->
    <div class="tabs">
      <button
        :class="['tab-btn', { active: activeTab === 'body' }]"
        @click="activeTab = 'body'"
      >
        Body
      </button>
      <button
        :class="['tab-btn', { active: activeTab === 'headers' }]"
        @click="activeTab = 'headers'"
      >
        Headers ({{ responseHeaders.length }})
      </button>
    </div>

    <!-- Body Tab -->
    <div v-show="activeTab === 'body'" class="tab-content">
      <pre class="response-body"><code>{{ formattedBody }}</code></pre>
    </div>

    <!-- Headers Tab -->
    <div v-show="activeTab === 'headers'" class="tab-content">
      <table class="headers-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(header, index) in responseHeaders" :key="index">
            <td class="header-name">{{ header[0] }}</td>
            <td class="header-value">{{ header[1] }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.response-viewer {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: white;
}

.response-meta {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 12px 16px;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
}

.status-badge {
  padding: 4px 12px;
  border-radius: 4px;
  font-weight: 600;
  font-size: 14px;
}

.status-badge.success {
  background: #d4edda;
  color: #155724;
}

.status-badge.redirect {
  background: #fff3cd;
  color: #856404;
}

.status-badge.client-error {
  background: #f8d7da;
  color: #721c24;
}

.status-badge.server-error {
  background: #f8d7da;
  color: #721c24;
}

.meta-item {
  font-size: 13px;
  color: #666;
}

.meta-label {
  color: #999;
  margin-right: 4px;
}

.tabs {
  display: flex;
  border-bottom: 1px solid #e0e0e0;
  background: #fafafa;
}

.tab-btn {
  padding: 10px 20px;
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
  padding: 16px;
}

.response-body {
  margin: 0;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 4px;
  font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  font-size: 13px;
  line-height: 1.5;
  overflow-x: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.headers-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.headers-table th {
  text-align: left;
  padding: 10px 12px;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
  font-weight: 600;
  color: #333;
}

.headers-table td {
  padding: 10px 12px;
  border-bottom: 1px solid #f0f0f0;
}

.header-name {
  font-weight: 500;
  color: #333;
  width: 30%;
}

.header-value {
  color: #666;
  font-family: monospace;
  word-break: break-all;
}
</style>