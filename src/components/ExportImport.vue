<script setup>
import { ref } from "vue";
import { exportCollection, importCollection } from "../api/export.js";
import { getCollections } from "../api/collections.js";

const collections = ref([]);
const selectedCollection = ref(null);
const showExportDialog = ref(false);
const showImportDialog = ref(false);
const importData = ref("");
const importError = ref("");

const emit = defineEmits(["collections-changed"]);

async function loadCollections() {
  try {
    collections.value = await getCollections();
  } catch (err) {
    console.error("Failed to load collections:", err);
  }
}

function openExport() {
  loadCollections();
  selectedCollection.value = null;
  showExportDialog.value = true;
}

function openImport() {
  importData.value = "";
  importError.value = "";
  showImportDialog.value = true;
}

async function handleExport() {
  if (!selectedCollection.value) return;
  
  try {
    const data = await exportCollection(selectedCollection.value.id);
    const json = JSON.stringify(data, null, 2);
    
    // Create download
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${data.info.name.replace(/\s+/g, '-').toLowerCase()}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    showExportDialog.value = false;
  } catch (err) {
    console.error("Export failed:", err);
    alert("Export failed: " + err.message);
  }
}

async function handleImport() {
  if (!importData.value.trim()) return;
  
  try {
    const data = JSON.parse(importData.value);
    await importCollection(data);
    showImportDialog.value = false;
    emit("collections-changed");
  } catch (err) {
    console.error("Import failed:", err);
    importError.value = err.message || "Invalid JSON format";
  }
}

loadCollections();
</script>

<template>
  <div class="export-import">
    <button @click="openExport" class="btn">Export</button>
    <button @click="openImport" class="btn">Import</button>

    <!-- Export Dialog -->
    <div v-if="showExportDialog" class="dialog-overlay" @click.self="showExportDialog = false">
      <div class="dialog">
        <h3>Export Collection</h3>
        <div class="form-group">
          <label>Select Collection</label>
          <select v-model="selectedCollection">
            <option v-for="col in collections" :key="col.id" :value="col">
              {{ col.name }}
            </option>
          </select>
        </div>
        <div class="dialog-actions">
          <button @click="showExportDialog = false">Cancel</button>
          <button @click="handleExport" class="primary" :disabled="!selectedCollection">
            Export
          </button>
        </div>
      </div>
    </div>

    <!-- Import Dialog -->
    <div v-if="showImportDialog" class="dialog-overlay" @click.self="showImportDialog = false">
      <div class="dialog">
        <h3>Import Collection</h3>
        <div class="form-group">
          <label>Paste Collection JSON</label>
          <textarea
            v-model="importData"
            rows="10"
            placeholder='{"info": {"name": "My Collection"}, "item": [...]}'
          ></textarea>
          <p v-if="importError" class="error">{{ importError }}</p>
        </div>
        <div class="dialog-actions">
          <button @click="showImportDialog = false">Cancel</button>
          <button @click="handleImport" class="primary" :disabled="!importData.trim()">
            Import
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.export-import {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 6px 12px;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
}

.btn:hover {
  background: #5a6268;
}

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
  width: 500px;
  max-width: 90%;
}

.dialog h3 {
  margin: 0 0 20px 0;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 13px;
  font-weight: 500;
}

.form-group select,
.form-group textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.form-group textarea {
  font-family: monospace;
  resize: vertical;
}

.error {
  color: #dc3545;
  font-size: 13px;
  margin-top: 8px;
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
}

.dialog-actions button.primary {
  background: #007bff;
  color: white;
}

.dialog-actions button:disabled {
  background: #ccc;
  cursor: not-allowed;
}
</style>