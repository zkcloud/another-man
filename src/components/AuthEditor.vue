<script setup>
import { ref, computed } from "vue";

const props = defineProps({
  auth: { type: Object, default: () => ({ type: "none" }) }
});

const emit = defineEmits(["update:auth"]);

const authType = ref(props.auth?.type || "none");

// Auth fields
const basicUsername = ref(props.auth?.username || "");
const basicPassword = ref(props.auth?.password || "");
const bearerToken = ref(props.auth?.token || "");
const apiKeyKey = ref(props.auth?.key || "");
const apiKeyValue = ref(props.auth?.value || "");
const apiKeyLocation = ref(props.auth?.addTo || "header");
const oauth2AccessToken = ref(props.auth?.access_token || "");
const oauth2TokenType = ref(props.auth?.token_type || "Bearer");

function updateAuth() {
  const auth = { type: authType.value };
  
  switch (authType.value) {
    case "basic":
      auth.username = basicUsername.value;
      auth.password = basicPassword.value;
      break;
    case "bearer":
      auth.token = bearerToken.value;
      break;
    case "apikey":
      auth.key = apiKeyKey.value;
      auth.value = apiKeyValue.value;
      auth.addTo = apiKeyLocation.value;
      break;
    case "oauth2":
      auth.access_token = oauth2AccessToken.value;
      auth.token_type = oauth2TokenType.value;
      break;
  }
  
  emit("update:auth", auth);
}

const authTypeOptions = [
  { value: "none", label: "No Auth" },
  { value: "basic", label: "Basic Auth" },
  { value: "bearer", label: "Bearer Token" },
  { value: "apikey", label: "API Key" },
  { value: "oauth2", label: "OAuth 2.0" },
];
</script>

<template>
  <div class="auth-editor">
    <div class="auth-type-select">
      <label>Type:</label>
      <select v-model="authType" @change="updateAuth">
        <option v-for="opt in authTypeOptions" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </div>

    <!-- Basic Auth -->
    <div v-if="authType === 'basic'" class="auth-fields">
      <div class="field-row">
        <label>Username:</label>
        <input v-model="basicUsername" type="text" placeholder="Username" @input="updateAuth" />
      </div>
      <div class="field-row">
        <label>Password:</label>
        <input v-model="basicPassword" type="password" placeholder="Password" @input="updateAuth" />
      </div>
    </div>

    <!-- Bearer Token -->
    <div v-if="authType === 'bearer'" class="auth-fields">
      <div class="field-row">
        <label>Token:</label>
        <input v-model="bearerToken" type="text" placeholder="Bearer token" @input="updateAuth" />
      </div>
    </div>

    <!-- API Key -->
    <div v-if="authType === 'apikey'" class="auth-fields">
      <div class="field-row">
        <label>Key:</label>
        <input v-model="apiKeyKey" type="text" placeholder="X-API-Key" @input="updateAuth" />
      </div>
      <div class="field-row">
        <label>Value:</label>
        <input v-model="apiKeyValue" type="text" placeholder="your-api-key" @input="updateAuth" />
      </div>
      <div class="field-row">
        <label>Add to:</label>
        <select v-model="apiKeyLocation" @change="updateAuth">
          <option value="header">Header</option>
          <option value="query">Query Parameter</option>
        </select>
      </div>
    </div>

    <!-- OAuth 2.0 -->
    <div v-if="authType === 'oauth2'" class="auth-fields">
      <div class="field-row">
        <label>Access Token:</label>
        <input v-model="oauth2AccessToken" type="password" placeholder="Access token" @input="updateAuth" />
      </div>
      <div class="field-row">
        <label>Token Type:</label>
        <input v-model="oauth2TokenType" type="text" placeholder="Bearer" @input="updateAuth" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.auth-editor {
  padding: 15px;
  background: #f9f9f9;
  border-radius: 6px;
}

.auth-type-select {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 15px;
}

.auth-type-select label {
  font-weight: 500;
  min-width: 60px;
}

.auth-type-select select {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.auth-fields {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.field-row {
  display: flex;
  align-items: center;
  gap: 10px;
}

.field-row label {
  min-width: 100px;
  font-size: 13px;
  color: #555;
}

.field-row input,
.field-row select {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.field-row input:focus,
.field-row select:focus {
  outline: none;
  border-color: #3498db;
}
</style>