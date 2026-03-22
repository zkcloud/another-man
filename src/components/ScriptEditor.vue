<script setup>
import { ref, watch } from "vue";
import { executePreRequestScript, executeTestScript, validateScript } from "../api/scripts.js";

const props = defineProps({
  type: { type: String, default: "pre-request" },
  envId: { type: String, default: null },
  response: { type: Object, default: null },
});

const emit = defineEmits(["script-change", "environment-change", "test-results"]);

const script = ref("");
const isValid = ref(true);
const validationError = ref("");
const isExecuting = ref(false);
const executionResult = ref(null);

// Default templates
const preRequestTemplate = `// Pre-request Script
// Modify environment variables before sending request

pm.environment.set("timestamp", Date.now().toString());
pm.environment.set("random", Math.random().toString(36).substring(7));
`;

const testTemplate = `// Test Script
// Validate response after receiving

pm.test("Status is 200", () => {
    pm.response.to.have.status(200);
});

pm.test("Response has body", () => {
    pm.response.to.have.jsonBody("data");
});
`;

// Initialize with template
if (!script.value) {
  script.value = props.type === "pre-request" ? preRequestTemplate : testTemplate;
}

watch(script, async (newScript) => {
  emit("script-change", newScript);
  
  if (newScript.trim()) {
    try {
      await validateScript(newScript);
      isValid.value = true;
      validationError.value = "";
    } catch (err) {
      isValid.value = false;
      validationError.value = err;
    }
  }
});

async function execute() {
  if (!script.value.trim()) return;
  
  isExecuting.value = true;
  executionResult.value = null;
  
  try {
    if (props.type === "pre-request") {
      const changes = await executePreRequestScript(script.value, props.envId);
      executionResult.value = { type: "environment", changes };
      emit("environment-change", changes);
    } else {
      if (!props.response) {
        throw new Error("No response available for test execution");
      }
      const result = await executeTestScript(script.value, props.response, props.envId);
      executionResult.value = { type: "test", result };
      emit("test-results", result);
    }
  } catch (err) {
    executionResult.value = { type: "error", error: err.message };
  } finally {
    isExecuting.value = false;
  }
}

function insertSnippet(snippet) {
  const textarea = document.querySelector(".script-textarea");
  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const text = script.value;
  
  script.value = text.substring(0, start) + snippet + text.substring(end);
  
  setTimeout(() => {
    textarea.focus();
    textarea.setSelectionRange(start + snippet.length, start + snippet.length);
  }, 0);
}
</script>

<template>
  <div class="script-editor">
    <div class="editor-header">
      <span class="editor-title">
        {{ type === "pre-request" ? "Pre-request Script" : "Tests" }}
      </span>
      <div class="editor-actions">
        <span v-if="!isValid" class="validation-error">⚠️ {{ validationError }}</span>
        <button @click="execute" :disabled="isExecuting || !isValid" class="btn-run">
          {{ isExecuting ? "Running..." : "Run" }}
        </button>
      </div>
    </div>
    
    <div class="editor-toolbar">
      <button @click="insertSnippet('pm.environment.set(\"key\", \"value\");')" class="snippet-btn">
        Set Env
      </button>
      <button v-if="type === 'test'" @click="insertSnippet('pm.test(\"name\", () => { pm.response.to.have.status(200); });')" class="snippet-btn">
        Status Test
      </button>
      <button v-if="type === 'test'" @click="insertSnippet('pm.response.json();')" class="snippet-btn">
        JSON Body
      </button>
    </div>
    
    <textarea
      v-model="script"
      class="script-textarea"
      :class="{ invalid: !isValid }"
      rows="10"
      spellcheck="false"
    ></textarea>
    
    <div v-if="executionResult" class="execution-result">
      <div v-if="executionResult.type === 'environment'" class="result-success">
        <h4>Environment Changes:</h4>
        <ul>
          <li v-for="(value, key) in executionResult.changes" :key="key">
            {{ key }} = {{ value }}
          </li>
        </ul>
      </div>
      
      <div v-else-if="executionResult.type === 'test'" class="result-tests">
        <h4>Test Results:</h4>
        <ul>
          <li v-for="test in executionResult.result.test_results" :key="test.name" :class="{ passed: test.passed, failed: !test.passed }">
            {{ test.passed ? "✓" : "✗" }} {{ test.name }}
            <span v-if="test.error" class="test-error">{{ test.error }}</span>
          </li>
        </ul>
      </div>
      
      <div v-else-if="executionResult.type === 'error'" class="result-error">
        Error: {{ executionResult.error }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.script-editor {
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  background: #fafafa;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  background: #f0f0f0;
  border-bottom: 1px solid #e0e0e0;
  border-radius: 6px 6px 0 0;
}

.editor-title {
  font-weight: 600;
  font-size: 14px;
}

.editor-actions {
  display: flex;
  gap: 10px;
  align-items: center;
}

.validation-error {
  color: #e74c3c;
  font-size: 12px;
}

.btn-run {
  padding: 6px 16px;
  background: #3498db;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
}

.btn-run:hover:not(:disabled) {
  background: #2980b9;
}

.btn-run:disabled {
  background: #95a5a6;
  cursor: not-allowed;
}

.editor-toolbar {
  display: flex;
  gap: 8px;
  padding: 8px 15px;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
}

.snippet-btn {
  padding: 4px 10px;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.snippet-btn:hover {
  background: #f0f0f0;
}

.script-textarea {
  width: 100%;
  padding: 12px 15px;
  border: none;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.5;
  resize: vertical;
  background: #fff;
}

.script-textarea:focus {
  outline: none;
}

.script-textarea.invalid {
  background: #fff5f5;
}

.execution-result {
  padding: 12px 15px;
  border-top: 1px solid #e0e0e0;
  background: #f8f9fa;
}

.execution-result h4 {
  margin: 0 0 8px 0;
  font-size: 13px;
}

.execution-result ul {
  margin: 0;
  padding-left: 20px;
}

.execution-result li {
  font-size: 12px;
  margin: 4px 0;
}

.result-success {
  color: #27ae60;
}

.result-tests li.passed {
  color: #27ae60;
}

.result-tests li.failed {
  color: #e74c3c;
}

.test-error {
  color: #e74c3c;
  font-size: 11px;
  margin-left: 8px;
}

.result-error {
  color: #e74c3c;
}
</style>