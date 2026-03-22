<script setup>
import { ref, onMounted, computed } from "vue";
import {
  createEnvironment,
  getEnvironments,
  updateEnvironment,
  deleteEnvironment,
  getActiveEnvironment,
} from "../api/environments.js";

const environments = ref([]);
const activeEnvironment = ref(null);
const selectedEnvironment = ref(null);
const showCreateDialog = ref(false);
const editingEnvironment = ref(null);

const newEnv = ref({
  name: "",
  variables: [{ key: "", value: "" }],
  isGlobal: false,
});

const emit = defineEmits(["environment-changed"]);

async function loadEnvironments() {
  try {
    environments.value = await getEnvironments();
    activeEnvironment.value = await getActiveEnvironment();
  } catch (err) {
    console.error("Failed to load environments:", err);
  }
}

async function selectEnvironment(env) {
  selectedEnvironment.value = env;
}

async function activateEnvironment(env) {
  try {
    await updateEnvironment(
      env.id,
      env.name,
      env.variables,
      true
    );
    await loadEnvironments();
    emit("environment-changed");
  } catch (err) {
    console.error("Failed to activate environment:", err);
  }
}

async function deactivateEnvironment() {
  if (!activeEnvironment.value) return;
  
  try {
    await updateEnvironment(
      activeEnvironment.value.id,
      activeEnvironment.value.name,
      activeEnvironment.value.variables,
      false
    );
    await loadEnvironments();
    emit("environment-changed");
  } catch (err) {
    console.error("Failed to deactivate environment:", err);
  }
}

function startCreate() {
  newEnv.value = {
    name: "",
    variables: [{ key: "", value: "" }],
    isGlobal: false,
  };
  showCreateDialog.value = true;
}

function startEdit(env) {
  editingEnvironment.value = {
    ...env,
    variables: Object.entries(env.variables || {}).map(([key, value]) => ({
      key,
      value,
    })),
  };
  if (editingEnvironment.value.variables.length === 0) {
    editingEnvironment.value.variables.push({ key: "", value: "" });
  }
}

function addVariable(type) {
  const target = type === "new" ? newEnv.value : editingEnvironment.value;
  if (target) {
    target.variables.push({ key: "", value: "" });
  }
}

function removeVariable(type, index) {
  const target = type === "new" ? newEnv.value : editingEnvironment.value;
  if (target && target.variables.length > 1) {
    target.variables.splice(index, 1);
  }
}

async function handleCreate() {
  if (!newEnv.value.name.trim()) return;
  
  const variables = {};
  newEnv.value.variables.forEach((v) => {
    if (v.key.trim()) {
      variables[v.key.trim()] = v.value;
    }
  });
  
  try {
    await createEnvironment({
      name: newEnv.value.name.trim(),
      variables,
      isGlobal: newEnv.value.isGlobal,
    });
    showCreateDialog.value = false;
    await loadEnvironments();
    emit("environment-changed");
  } catch (err) {
    console.error("Failed to create environment:", err);
  }
}

async function handleUpdate() {
  if (!editingEnvironment.value?.name.trim()) return;
  
  const variables = {};
  editingEnvironment.value.variables.forEach((v) => {
    if (v.key.trim()) {
      variables[v.key.trim()] = v.value;
    }
  });
  
  try {
    await updateEnvironment(
      editingEnvironment.value.id,
      editingEnvironment.value.name.trim(),
      variables,
      editingEnvironment.value.isGlobal
    );
    editingEnvironment.value = null;
    await loadEnvironments();
    emit("environment-changed");
  } catch (err) {
    console.error("Failed to update environment:", err);
  }
}

async function handleDelete(id) {
  if (!confirm("Delete this environment?")) return;
  
  try {
    await deleteEnvironment(id);
    if (selectedEnvironment.value?.id === id) {
      selectedEnvironment.value = null;
    }
    await loadEnvironments();
    emit("environment-changed");
  } catch (err) {
    console.error("Failed to delete environment:", err);
  }
}

onMounted(() => {
  loadEnvironments();
});

defineExpose({ loadEnvironments });
</script>

<template>
  <div class="environment-manager">
    <div class="header">
      <h3>Environments</h3>
      <button class="add-btn" @click="startCreate">+ New</button>
    </div>

    <!-- Active Environment -->
    <div v-if="activeEnvironment" class="active-env">
      <span class="label">Active:</span>
      <span class="name">{{ activeEnvironment.name }}</span>
      <span class="vars">({{ Object.keys(activeEnvironment.variables || {}).length }} vars)</span>
      <button class="deactivate-btn" @click="deactivateEnvironment">Deactivate</button>
    </div>

    <!-- Environment List -->
    <div class="env-list">
      <div
        v-for="env in environments"
        :key="env.id"
        :class="['env-item', { active: activeEnvironment?.id === env.id }]"
        @click="selectEnvironment(env)"
      >
        <div class="env-info">
          <span class="env-name">{{ env.name }}</span>
          <span class="var-count">{{ Object.keys(env.variables || {}).length }} variables</span>
        </div>
        <div class="env-actions">
          <button v-if="activeEnvironment?.id !== env.id" @click.stop="activateEnvironment(env)">
            Activate
          </button>
          <button @click.stop="startEdit(env)">Edit</button>
          <button class="delete" @click.stop="handleDelete(env.id)">Delete</button>
        </div>
      </div>
      <div v-if="environments.length === 0" class="empty">
        No environments yet
      </div>
    </div>

    <!-- Create Dialog -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click.self="showCreateDialog = false">
      <div class="dialog">
        <h3>New Environment</h3>
        <div class="form-group">
          <label>Name</label>
          <input v-model="newEnv.name" type="text" placeholder="Environment name..." />
        </div>
        <div class="form-group">
          <label>Variables</label>
          <div class="variables-list">
            <div v-for="(v, i) in newEnv.variables" :key="i" class="var-row">
              <input v-model="v.key" placeholder="Key" class="var-key" />
              <input v-model="v.value" placeholder="Value" class="var-value" />
              <button v-if="newEnv.variables.length > 1" @click="removeVariable('new', i)" class="remove-btn">×</button>
            </div>
          </div>
          <button @click="addVariable('new')" class="add-var-btn">+ Add Variable</button>
        </div>
        <div class="form-group checkbox">
          <label>
            <input type="checkbox" v-model="newEnv.isGlobal" />
            Set as active environment
          </label>
        </div>
        <div class="dialog-actions">
          <button @click="showCreateDialog = false">Cancel</button>
          <button @click="handleCreate" class="primary">Create</button>
        </div>
      </div>
    </div>

    <!-- Edit Dialog -->
    <div v-if="editingEnvironment" class="dialog-overlay" @click.self="editingEnvironment = null">
      <div class="dialog">
        <h3>Edit Environment</h3>
        <div class="form-group">
          <label>Name</label>
          <input v-model="editingEnvironment.name" type="text" placeholder="Environment name..." />
        </div>
        <div class="form-group">
          <label>Variables</label>
          <div class="variables-list">
            <div v-for="(v, i) in editingEnvironment.variables" :key="i" class="var-row">
              <input v-model="v.key" placeholder="Key" class="var-key" />
              <input v-model="v.value" placeholder="Value" class="var-value" />
              <button v-if="editingEnvironment.variables.length > 1" @click="removeVariable('edit', i)" class="remove-btn">×</button>
            </div>
          </div>
          <button @click="addVariable('edit')" class="add-var-btn">+ Add Variable</button>
        </div>
        <div class="form-group checkbox">
          <label>
            <input type="checkbox" v-model="editingEnvironment.isGlobal" />
            Set as active environment
          </label>
        </div>
        <div class="dialog-actions">
          <button @click="editingEnvironment = null">Cancel</button>
          <button @click="handleUpdate" class="primary">Save</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.environment-manager {
  padding: 16px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.header h3 {
  margin: 0;
  font-size: 16px;
}

.add-btn {
  padding: 6px 12px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
}

.active-env {
  padding: 12px;
  background: #d4edda;
  border-radius: 4px;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.active-env .label {
  font-weight: 600;
  color: #155724;
}

.active-env .name {
  font-weight: 500;
}

.active-env .vars {
  color: #666;
  font-size: 12px;
}

.deactivate-btn {
  margin-left: auto;
  padding: 4px 10px;
  background: #6c757d;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.env-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.env-item {
  padding: 12px;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.env-item:hover {
  border-color: #007bff;
}

.env-item.active {
  border-color: #28a745;
  background: #f8fff8;
}

.env-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.env-name {
  font-weight: 500;
}

.var-count {
  color: #666;
  font-size: 12px;
}

.env-actions {
  display: flex;
  gap: 8px;
}

.env-actions button {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  background: #f0f0f0;
}

.env-actions button.delete {
  background: #f8d7da;
  color: #721c24;
}

.empty {
  text-align: center;
  padding: 40px;
  color: #999;
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
  width: 500px;
  max-width: 90%;
  max-height: 80vh;
  overflow-y: auto;
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

.form-group input[type="text"] {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.variables-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 8px;
}

.var-row {
  display: flex;
  gap: 8px;
}

.var-key {
  width: 150px;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.var-value {
  flex: 1;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.remove-btn {
  width: 32px;
  height: 32px;
  background: #dc3545;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.add-var-btn {
  width: 100%;
  padding: 8px;
  background: #f0f0f0;
  border: 1px dashed #ccc;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  color: #666;
}

.checkbox label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.checkbox input {
  width: 16px;
  height: 16px;
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
</style>