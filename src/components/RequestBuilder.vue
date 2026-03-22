<script setup>
import { ref, reactive, watch } from "vue";
import { sendHttpRequest, HttpMethods, createRequest, createHeader } from "../api/http.js";
import { createSavedRequest, getCollections } from "../api/collections.js";
import { executePreRequestScript, executeTestScript } from "../api/scripts.js";
import ResponseViewer from "./ResponseViewer.vue";
import ScriptEditor from "./ScriptEditor.vue";

const request = reactive(createRequest("GET", ""));
const response = ref(null);
const loading = ref(false);
const error = ref(null);

const newHeader = reactive({ key: "", value: "" });

// Save request dialog
const showSaveDialog = ref(false);
const saveName = ref("");
const saveCollectionId = ref("");
const collections = ref([]);

// Scripts
const activeTab = ref("params"); // params, headers, body, pre-script, tests
const preRequestScript = ref("");
const testScript = ref("");
const scriptEnvChanges = ref({});

const emit = defineEmits(["request-sent"]);

async function sendRequest() {
  loading.value = true;
  error.value = null;
  response.value = null;
  
  try {
    // Execute pre-request script
    if (preRequestScript.value.trim()) {
      try {
        const changes = await executePreRequestScript(preRequestScript.value);
        scriptEnvChanges.value = changes;
        // Apply environment changes to request
        for (const [key, value] of Object.entries(changes)) {
          request.url = request.url.replace(`{{${key}}}`, value);
          if (request.body) {
            request.body = request.body.replace(`{{${key}}}`, value);
          }
          request.headers.forEach(h => {
            h.value = h.value.replace(`{{${key}}}`, value);
          });
        }
      } catch (scriptErr) {
        console.error("Pre-request script error:", scriptErr);
      }
    }
    
    // Prepare body
    let body = null;
    if (request.body) {
      body = { Json: request.body };
    }
    
    const req = {
      ...request,
      body,
    };
    
    const result = await sendHttpRequest(req);
    response.value = result;
    
    // Execute test script
    if (testScript.value.trim() && result) {
      try {
        await executeTestScript(testScript.value, result);
      } catch (testErr) {
        console.error("Test script error:", testErr);
      }
    }
    
    emit("request-sent");
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

// Save request functionality
async function openSaveDialog() {
  try {
    collections.value = await getCollections();
    saveName.value = request.url ? new URL(request.url).pathname : "New Request";
    showSaveDialog.value = true;
  } catch (err) {
    console.error("Failed to load collections:", err);
  }
}

async function handleSaveRequest() {
  if (!saveName.value.trim()) return;
  
  try {
    await createSavedRequest({
      collection_id: saveCollectionId.value || null,
      name: saveName.value.trim(),
      method: request.method,
      url: request.url,
      headers: request.headers,
      body: request.body || null,
    });
    showSaveDialog.value = false;
    emit("request-sent"); // Refresh sidebar
  } catch (err) {
    console.error("Failed to save request:", err);
  }
}

// Load request from collection
function loadRequest(savedRequest) {
  request.id = savedRequest.id;
  request.method = savedRequest.method;
  request.url = savedRequest.url;
  request.headers = savedRequest.headers ? JSON.parse(savedRequest.headers) : [];
  request.body = savedRequest.body || null;
}

// Expose loadRequest for parent
defineExpose({ loadRequest });
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
      <button @click="openSaveDialog" :disabled="!request.url" class="save-btn">
        Save
      </button>
    </div>

    <!-- Request Tabs -->
    <div class="request-tabs">
      <button 
        :class="['tab-btn', { active: activeTab === 'headers' }]" 
        @click="activeTab = 'headers'"
      >
        Headers
      </button>
      <button 
        :class="['tab-btn', { active: activeTab === 'body' }]" 
        @click="activeTab = 'body'"
      >
        Body
      </button>
      <button 
        :class="['tab-btn', { active: activeTab === 'pre-script' }]" 
        @click="activeTab = 'pre-script'"
      >
        Pre-request Script
      </button>
      <button 
        :class="['tab-btn', { active: activeTab === 'tests' }]" 
        @click="activeTab = 'tests'"
      >
        Tests
      </button>
    </div>

    <!-- Headers Tab -->
    <div v-if="activeTab === 'headers'" class="tab-content">
      <div class="section">
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
    </div>

    <!-- Body Tab -->
    <div v-if="activeTab === 'body'" class="tab-content">
      <div class="section">
        <textarea
          v-model="request.body"
          placeholder="Request body (JSON/Text)..."
          class="body-input"
          rows="6"
        ></textarea>
      </div>
    </div>

    <!-- Pre-request Script Tab -->
    <div v-if="activeTab === 'pre-script'" class="tab-content">
      <ScriptEditor
        type="pre-request"
        @script-change="(s) => preRequestScript = s"
      />
    </div>

    <!-- Tests Tab -->
    <div v-if="activeTab === 'tests'" class="tab-content">
      <ScriptEditor
        type="test"
        :response="response"
        @script-change="(s) => testScript = s"
      />
    </div>

    <!-- Response Section -->
    <ResponseViewer v-if="response" :response="response" />

    <!-- Error Section -->
    <div v-if="error" class="error-section">
      <h3>Error</h3>
      <p class="error-message">{{ error }}</p>
    </div>

    <!-- Save Dialog -->
    <div v-if="showSaveDialog" class="dialog-overlay" @click.self="showSaveDialog = false">
      <div class="dialog">
        <h3>Save Request</h3>
        <div class="form-group">
          <label>Name</label>
          <input v-model="saveName" type="text" placeholder="Request name..." />
        </div>
        <div class="form-group">
          <label>Collection</label>
          <select v-model="saveCollectionId">
            <option value="">No collection</option>
            <option v-for="col in collections" :key="col.id" :value="col.id">
              {{ col.name }}
            </option>
          </select>
        </div>
        <div class="dialog-actions">
          <button @click="showSaveDialog = false">Cancel</button>
          <button @click="handleSaveRequest" class="primary">Save</button>
        </div>
      </div>
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

.save-btn {
  padding: 10px 20px;
  background: #28a745;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.save-btn:hover:not(:disabled) {
  background: #1e7e34;
}

.save-btn:disabled {
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

.error-section {
  margin-top: 20px;
  padding: 15px;
  border: 1px solid #f5c6cb;
  border-radius: 8px;
  background: #fff0f0;
}

.error-message {
  color: #dc3545;
  font-weight: 500;
}

/* Dialog styles */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: white;
  border-radius: 8px;
  padding: 24px;
  width: 400px;
  max-width: 90%;
}

.dialog h3 {
  margin: 0 0 20px 0;
  font-size: 18px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 13px;
  font-weight: 500;
  color: #333;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
}

.dialog-actions button {
  padding: 8px 20px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.dialog-actions button:first-child {
  background: #f0f0f0;
  color: #333;
}

.dialog-actions button.primary {
  background: #007bff;
  color: white;
}

/* Tabs */
.request-tabs {
  display: flex;
  gap: 4px;
  margin-bottom: 0;
  border-bottom: 1px solid #ddd;
}

.tab-btn {
  padding: 10px 20px;
  background: #f8f9fa;
  border: 1px solid transparent;
  border-bottom: none;
  border-radius: 4px 4px 0 0;
  cursor: pointer;
  font-size: 13px;
  color: #666;
}

.tab-btn:hover {
  background: #e9ecef;
}

.tab-btn.active {
  background: white;
  border-color: #ddd;
  border-bottom-color: white;
  color: #333;
  font-weight: 500;
}

.tab-content {
  padding: 20px;
  border: 1px solid #ddd;
  border-top: none;
  border-radius: 0 0 4px 4px;
  margin-bottom: 20px;
}
</style>