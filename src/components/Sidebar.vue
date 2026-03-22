<script setup>
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  createCollection,
  getCollections,
  deleteCollection,
  createSavedRequest,
  getRequestsByCollection,
  deleteSavedRequest,
} from "../api/collections.js";

const history = ref([]);
const collections = ref([]);
const activeTab = ref("history");
const expandedCollections = ref(new Set());

// New collection form
const showNewCollection = ref(false);
const newCollectionName = ref("");

// Computed: tree structure with requests
const collectionTree = computed(() => {
  return collections.value.map((col) => ({
    ...col,
    requests: expandedCollections.value.has(col.id) ? col._requests || [] : [],
  }));
});

async function loadHistory() {
  try {
    const result = await invoke("get_history", { limit: 100 });
    history.value = result;
  } catch (err) {
    console.error("Failed to load history:", err);
  }
}

async function loadCollections() {
  try {
    const result = await getCollections();
    collections.value = result;
  } catch (err) {
    console.error("Failed to load collections:", err);
  }
}

async function toggleCollection(col) {
  if (expandedCollections.value.has(col.id)) {
    expandedCollections.value.delete(col.id);
  } else {
    expandedCollections.value.add(col.id);
    // Load requests for this collection
    if (!col._requests) {
      try {
        const requests = await getRequestsByCollection(col.id);
        col._requests = requests;
      } catch (err) {
        console.error("Failed to load requests:", err);
        col._requests = [];
      }
    }
  }
}

async function handleCreateCollection() {
  if (!newCollectionName.value.trim()) return;
  
  try {
    await createCollection({
      name: newCollectionName.value.trim(),
      description: null,
      parent_id: null,
    });
    newCollectionName.value = "";
    showNewCollection.value = false;
    await loadCollections();
  } catch (err) {
    console.error("Failed to create collection:", err);
  }
}

async function handleDeleteCollection(id) {
  if (!confirm("Delete this collection and all its requests?")) return;
  
  try {
    await deleteCollection(id);
    await loadCollections();
  } catch (err) {
    console.error("Failed to delete collection:", err);
  }
}

async function handleDeleteRequest(id) {
  if (!confirm("Delete this request?")) return;
  
  try {
    await deleteSavedRequest(id);
    await loadCollections();
  } catch (err) {
    console.error("Failed to delete request:", err);
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
  emit("select", item);
}

function selectRequest(request) {
  emit("select-request", request);
}

const emit = defineEmits(["select", "select-request"]);

onMounted(() => {
  loadHistory();
  loadCollections();
});

// Expose methods for parent
defineExpose({ loadHistory, loadCollections });
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
      <div class="collections-header">
        <span>Collections</span>
        <button class="add-btn" @click="showNewCollection = true">+</button>
      </div>

      <!-- New Collection Form -->
      <div v-if="showNewCollection" class="new-collection-form">
        <input
          v-model="newCollectionName"
          type="text"
          placeholder="Collection name..."
          @keyup.enter="handleCreateCollection"
          @keyup.esc="showNewCollection = false"
        />
        <div class="form-actions">
          <button @click="handleCreateCollection">Create</button>
          <button @click="showNewCollection = false">Cancel</button>
        </div>
      </div>

      <!-- Collection Tree -->
      <div v-if="collections.length === 0" class="empty-state">
        No collections yet
      </div>
      <div v-else class="collection-tree">
        <div
          v-for="col in collections"
          :key="col.id"
          class="collection-node"
        >
          <div class="collection-header" @click="toggleCollection(col)">
            <span class="expand-icon">
              {{ expandedCollections.has(col.id) ? "▼" : "▶" }}
            </span>
            <span class="folder-icon">📁</span>
            <span class="collection-name">{{ col.name }}</span>
            <button class="delete-btn" @click.stop="handleDeleteCollection(col.id)">
              ×
            </button>
          </div>
          <div v-if="expandedCollections.has(col.id)" class="request-list">
            <div
              v-for="req in col._requests || []"
              :key="req.id"
              class="request-item"
              @click="selectRequest(req)"
            >
              <span :class="['method', req.method.toLowerCase()]">
                {{ req.method }}
              </span>
              <span class="request-name">{{ req.name }}</span>
              <button class="delete-btn" @click.stop="handleDeleteRequest(req.id)">
                ×
              </button>
            </div>
            <div v-if="!col._requests?.length" class="empty-requests">
              No requests
            </div>
          </div>
        </div>
      </div>
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

/* Collections styles */
.collections-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f0f0f0;
  font-weight: 600;
  font-size: 13px;
  color: #333;
}

.add-btn {
  width: 24px;
  height: 24px;
  border: none;
  background: #007bff;
  color: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
}

.add-btn:hover {
  background: #0056b3;
}

.new-collection-form {
  padding: 12px;
  background: #fff;
  border-bottom: 1px solid #e0e0e0;
}

.new-collection-form input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  margin-bottom: 8px;
}

.form-actions {
  display: flex;
  gap: 8px;
}

.form-actions button {
  flex: 1;
  padding: 6px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.form-actions button:first-child {
  background: #007bff;
  color: white;
}

.form-actions button:last-child {
  background: #f0f0f0;
  color: #333;
}

.collection-tree {
  padding: 8px 0;
}

.collection-node {
  margin-bottom: 2px;
}

.collection-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  cursor: pointer;
  transition: background 0.2s;
}

.collection-header:hover {
  background: #e3f2fd;
}

.expand-icon {
  font-size: 10px;
  color: #666;
  width: 12px;
}

.folder-icon {
  font-size: 14px;
}

.collection-name {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.delete-btn {
  opacity: 0;
  width: 20px;
  height: 20px;
  border: none;
  background: #dc3545;
  color: white;
  border-radius: 50%;
  cursor: pointer;
  font-size: 14px;
  line-height: 1;
}

.collection-header:hover .delete-btn,
.request-item:hover .delete-btn {
  opacity: 1;
}

.request-list {
  padding-left: 24px;
}

.request-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  cursor: pointer;
  transition: background 0.2s;
}

.request-item:hover {
  background: #e3f2fd;
}

.request-name {
  flex: 1;
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-requests {
  padding: 8px 12px;
  font-size: 12px;
  color: #999;
}
</style>