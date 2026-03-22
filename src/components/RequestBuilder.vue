<script setup>
import { ref, reactive } from "vue";
import { sendHttpRequest, HttpMethods, createRequest, createHeader } from "../api/http.js";
import ResponseViewer from "./ResponseViewer.vue";

const request = reactive(createRequest("GET", ""));
const response = ref(null);
const loading = ref(false);
const error = ref(null);

const newHeader = reactive({ key: "", value: "" });

async function sendRequest() {
  loading.value = true;
  error.value = null;
  response.value = null;
  
  try {
    const result = await sendHttpRequest(request);
    response.value = result;
  } catch (err) {
    error.value = err.message || "Request failed";
  } finally {
    loading.value = false;
  }
}

function addHeader() {
  if (newHeader.key.trim()) {
    request.headers.push(createHeader(newHeader.key, newHeader.value));
    newHeader.key = "";
    newHeader.value = "";
  }
}

function removeHeader(index) {
  request.headers.splice(index, 1);
}

function toggleHeader(index) {
  request.headers[index].enabled = !request.headers[index].enabled;
}
</script>

<template>
  <div class="request-builder">
    <!-- URL Bar -->
    <div class="url-bar">
      <select v-model="request.method" class="method-select">
        <option v-for="method in HttpMethods" :key="method" :value="method">
          {{ method }}
        </option>
      </select>
      <input
        v-model="request.url"
        type="text"
        placeholder="Enter URL..."
        class="url-input"
      />
      <button @click="sendRequest" :disabled="loading || !request.url" class="send-btn">
        {{ loading ? "Sending..." : "Send" }}
      </button>
    </div>

    <!-- Headers Section -->
    <div class="section">
      <h3>Headers</h3>
      <div class="headers-list">
        <div
          v-for="(header, index) in request.headers"
          :key="index"
          class="header-row"
        >
          <input
            type="checkbox"
            :checked="header.enabled"
            @change="toggleHeader(index)"
          />
          <input v-model="header.key" placeholder="Key" class="header-key" />
          <input v-model="header.value" placeholder="Value" class="header-value" />
          <button @click="removeHeader(index)" class="remove-btn">×</button>
        </div>
        <div class="header-row new-header">
          <input v-model="newHeader.key" placeholder="Key" class="header-key" />
          <input v-model="newHeader.value" placeholder="Value" class="header-value" />
          <button @click="addHeader" class="add-btn">+</button>
        </div>
      </div>
    </div>

    <!-- Body Section -->
    <div class="section">
      <h3>Body</h3>
      <textarea
        v-model="request.body"
        placeholder="Request body (JSON/Text)..."
        class="body-input"
        rows="6"
      ></textarea>
    </div>

    <!-- Response Section -->
    <ResponseViewer v-if="response" :response="response" />

    <!-- Error Section -->
    <div v-if="error" class="error-section">
      <h3>Error</h3>
      <p class="error-message">{{ error }}</p>
    </div>
  </div>
</template>

<style scoped>
.request-builder {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.url-bar {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.method-select {
  padding: 10px 15px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 600;
  background: white;
  min-width: 100px;
}

.url-input {
  flex: 1;
  padding: 10px 15px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.send-btn {
  padding: 10px 30px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.send-btn:hover:not(:disabled) {
  background: #0056b3;
}

.send-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.section {
  margin-bottom: 20px;
  padding: 15px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: #fafafa;
}

.section h3 {
  margin: 0 0 15px 0;
  font-size: 16px;
  color: #333;
}

.headers-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.header-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-row input[type="checkbox"] {
  width: 18px;
  height: 18px;
}

.header-key {
  width: 200px;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.header-value {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.remove-btn, .add-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 4px;
  font-size: 18px;
  cursor: pointer;
}

.remove-btn {
  background: #dc3545;
  color: white;
}

.add-btn {
  background: #28a745;
  color: white;
}

.body-input {
  width: 100%;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: monospace;
  font-size: 13px;
  resize: vertical;
}

.response-section, .error-section {
  margin-top: 20px;
  padding: 15px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
}

.response-section {
  background: #f0f8ff;
}

.error-section {
  background: #fff0f0;
}

.response-meta {
  display: flex;
  gap: 20px;
  margin-bottom: 15px;
  padding-bottom: 15px;
  border-bottom: 1px solid #ddd;
}

.status {
  font-weight: 600;
  padding: 4px 12px;
  border-radius: 4px;
}

.status.success {
  background: #d4edda;
  color: #155724;
}

.status.error {
  background: #f8d7da;
  color: #721c24;
}

.time, .size {
  color: #666;
  font-size: 14px;
}

.response-body {
  background: white;
  padding: 15px;
  border-radius: 4px;
  overflow-x: auto;
  font-family: monospace;
  font-size: 13px;
  line-height: 1.5;
  max-height: 400px;
  overflow-y: auto;
}

.error-message {
  color: #dc3545;
  font-weight: 500;
}
</style>