<script setup>
import { ref, onMounted, watch } from "vue";
import {
  getTestSuites,
  createTestSuite,
  deleteTestSuite,
  getTestCases,
  createTestCase,
  deleteTestCase,
  updateTestCaseScript,
  getTestRuns
} from "../api/testSuites.js";
import { getCollections } from "../api/collections.js";

const props = defineProps({ visible: Boolean });
const emit = defineEmits(["close"]);

const testSuites = ref([]);
const selectedSuite = ref(null);
const testCases = ref([]);
const testRuns = ref([]);
const collections = ref([]);
const showCreateSuite = ref(false);
const showCreateCase = ref(false);
const showCaseDetail = ref(false);
const selectedCase = ref(null);

const newSuite = ref({ name: "", description: "", collectionId: "" });
const newCase = ref({ name: "" });

onMounted(() => { loadTestSuites(); loadCollections(); });
watch(() => props.visible, (val) => { if (val) { loadTestSuites(); loadCollections(); } });

async function loadTestSuites() {
  try { testSuites.value = await getTestSuites(); }
  catch (err) { console.error("Failed to load test suites:", err); }
}

async function loadCollections() {
  try { collections.value = await getCollections(); }
  catch (err) { console.error("Failed to load collections:", err); }
}

async function onSelectSuite(suite) {
  selectedSuite.value = suite;
  await loadTestCases(suite.id);
  await loadTestRuns(suite.id);
}

async function loadTestCases(suiteId) {
  try { testCases.value = await getTestCases(suiteId); }
  catch (err) { console.error("Failed to load test cases:", err); }
}

async function loadTestRuns(suiteId) {
  try { testRuns.value = await getTestRuns(suiteId); }
  catch (err) { console.error("Failed to load test runs:", err); }
}

async function onCreateSuite() {
  if (!newSuite.value.name.trim()) return;
  try {
    await createTestSuite(newSuite.value.name, newSuite.value.description || null, newSuite.value.collectionId || null);
    newSuite.value = { name: "", description: "", collectionId: "" };
    showCreateSuite.value = false;
    await loadTestSuites();
  } catch (err) {
    console.error("Failed to create test suite:", err);
    alert("创建测试套件失败: " + err.message);
  }
}

async function onDeleteSuite(suite) {
  if (!confirm(`确定要删除测试套件 "${suite.name}" 吗？`)) return;
  try {
    await deleteTestSuite(suite.id);
    if (selectedSuite.value?.id === suite.id) { selectedSuite.value = null; testCases.value = []; }
    await loadTestSuites();
  } catch (err) {
    console.error("Failed to delete test suite:", err);
    alert("删除测试套件失败: " + err.message);
  }
}

async function onCreateCase() {
  if (!newCase.value.name.trim() || !selectedSuite.value) return;
  try {
    await createTestCase(selectedSuite.value.id, newCase.value.name, null);
    newCase.value = { name: "" };
    showCreateCase.value = false;
    await loadTestCases(selectedSuite.value.id);
  } catch (err) {
    console.error("Failed to create test case:", err);
    alert("创建测试用例失败: " + err.message);
  }
}

async function onDeleteCase(testCase) {
  if (!confirm(`确定要删除测试用例 "${testCase.name}" 吗？`)) return;
  try {
    await deleteTestCase(testCase.id);
    await loadTestCases(selectedSuite.value.id);
  } catch (err) {
    console.error("Failed to delete test case:", err);
    alert("删除测试用例失败: " + err.message);
  }
}

function onEditCase(testCase) {
  selectedCase.value = { ...testCase };
  showCaseDetail.value = true;
}

async function onSaveCaseScript() {
  if (!selectedCase.value) return;
  try {
    await updateTestCaseScript(
      selectedCase.value.id,
      selectedCase.value.pre_script || null,
      selectedCase.value.test_script || null
    );
    showCaseDetail.value = false;
    await loadTestCases(selectedSuite.value.id);
  } catch (err) {
    console.error("Failed to update test case script:", err);
    alert("保存脚本失败: " + err.message);
  }
}

function formatDate(timestamp) {
  if (!timestamp) return "-";
  return new Date(timestamp * 1000).toLocaleString();
}

function getStatusClass(status) {
  switch (status) {
    case "passed": return "status-passed";
    case "failed": return "status-failed";
    case "skipped": return "status-skipped";
    case "running": return "status-running";
    default: return "";
  }
}

function getStatusText(status) {
  switch (status) {
    case "passed": return "通过";
    case "failed": return "失败";
    case "skipped": return "跳过";
    case "running": return "运行中";
    default: return status;
  }
}
</script>

<template>
  <div v-if="visible" class="test-suite-manager">
    <div class="manager-header">
      <h2>测试套件管理</h2>
      <button class="close-btn" @click="$emit('close')">×</button>
    </div>
    <div class="manager-content">
      <div class="suites-panel">
        <div class="panel-header">
          <h3>测试套件</h3>
          <button class="btn-small" @click="showCreateSuite = true">+ 新建</button>
        </div>
        <div v-if="showCreateSuite" class="create-form">
          <input v-model="newSuite.name" placeholder="套件名称" @keyup.enter="onCreateSuite" />
          <input v-model="newSuite.description" placeholder="描述（可选）" />
          <select v-model="newSuite.collectionId">
            <option value="">关联 Collection（可选）</option>
            <option v-for="c in collections" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
          <div class="form-actions">
            <button class="btn-primary" @click="onCreateSuite">创建</button>
            <button class="btn-secondary" @click="showCreateSuite = false">取消</button>
          </div>
        </div>
        <div class="suites-list">
          <div v-for="suite in testSuites" :key="suite.id" :class="['suite-item', { active: selectedSuite?.id === suite.id }]" @click="onSelectSuite(suite)">
            <div class="suite-info">
              <span class="suite-name">{{ suite.name }}</span>
              <span v-if="suite.description" class="suite-desc">{{ suite.description }}</span>
            </div>
            <button class="btn-icon" @click.stop="onDeleteSuite(suite)">🗑</button>
          </div>
        </div>
      </div>
      <div v-if="selectedSuite" class="cases-panel">
        <div class="panel-section">
          <div class="panel-header">
            <h3>测试用例</h3>
            <button class="btn-small" @click="showCreateCase = true">+ 添加</button>
          </div>
          <div v-if="showCreateCase" class="create-form">
            <input v-model="newCase.name" placeholder="用例名称" @keyup.enter="onCreateCase" />
            <div class="form-actions">
              <button class="btn-primary" @click="onCreateCase">添加</button>
              <button class="btn-secondary" @click="showCreateCase = false">取消</button>
            </div>
          </div>
          <div class="cases-list">
            <div v-for="(testCase, index) in testCases" :key="testCase.id" class="case-item">
              <span class="case-index">{{ index + 1 }}</span>
              <span class="case-name">{{ testCase.name }}</span>
              <div class="case-actions">
                <button class="btn-icon" @click="onEditCase(testCase)">✏️</button>
                <button class="btn-icon" @click="onDeleteCase(testCase)">🗑</button>
              </div>
            </div>
          </div>
        </div>
        <div class="panel-section">
          <div class="panel-header">
            <h3>运行记录</h3>
          </div>
          <div class="runs-list">
            <div v-for="run in testRuns" :key="run.id" class="run-item">
              <div class="run-header">
                <span :class="['run-status', getStatusClass(run.status)]">{{ getStatusText(run.status) }}</span>
                <span class="run-time">{{ formatDate(run.started_at) }}</span>
              </div>
              <div class="run-stats">
                <span class="stat total">总计: {{ run.total_tests }}</span>
                <span class="stat passed">通过: {{ run.passed }}</span>
                <span class="stat failed">失败: {{ run.failed }}</span>
                <span class="stat duration">{{ run.duration_ms }}ms</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-if="showCaseDetail && selectedCase" class="case-detail-modal">
      <div class="modal-content">
        <div class="modal-header">
          <h3>编辑测试用例: {{ selectedCase.name }}</h3>
          <button class="close-btn" @click="showCaseDetail = false">×</button>
        </div>
        <div class="modal-body">
          <div class="script-section">
            <label>Pre-request Script</label>
            <textarea v-model="selectedCase.pre_script" rows="6" placeholder="// 在请求前执行的脚本"></textarea>
          </div>
          <div class="script-section">
            <label>Test Script</label>
            <textarea v-model="selectedCase.test_script" rows="10" placeholder="// 在请求后执行的测试脚本"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn-primary" @click="onSaveCaseScript">保存</button>
          <button class="btn-secondary" @click="showCaseDetail = false">取消</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.test-suite-manager {
  position: fixed; top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0,0,0,0.5); z-index: 1000;
  display: flex; flex-direction: column;
}
.manager-header {
  background: #1a1a2e; color: white; padding: 16px 24px;
  display: flex; justify-content: space-between; align-items: center;
}
.manager-header h2 { font-size: 18px; font-weight: 600; }
.close-btn { background: none; border: none; color: white; font-size: 24px; cursor: pointer; }
.manager-content { flex: 1; display: flex; background: white; overflow: hidden; }
.suites-panel { width: 300px; border-right: 1px solid #e0e0e0; overflow-y: auto; }
.cases-panel { flex: 1; display: flex; flex-direction: column; overflow-y: auto; }
.panel-section { padding: 16px; border-bottom: 1px solid #e0e0e0; }
.panel-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.panel-header h3 { font-size: 14px; font-weight: 600; color: #333; }
.btn-small { padding: 4px 12px; font-size: 12px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer; }
.btn-small:hover { background: #218838; }
.create-form { background: #f8f9fa; padding: 12px; border-radius: 4px; margin-bottom: 12px; }
.create-form input, .create-form select { width: 100%; padding: 8px; margin-bottom: 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 13px; }
.form-actions { display: flex; gap: 8px; }
.btn-primary { padding: 6px 16px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; }
.btn-secondary { padding: 6px 16px; background: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px; }
.suites-list, .cases-list, .runs-list { display: flex; flex-direction: column; gap: 8px; }
.suite-item, .case-item { display: flex; justify-content: space-between; align-items: center; padding: 12px; border: 1px solid #e0e0e0; border-radius: 4px; cursor: pointer; transition: background 0.2s; }
.suite-item:hover, .case-item:hover { background: #f8f9fa; }
.suite-item.active { background: #e3f2fd; border-color: #2196f3; }
.suite-info { display: flex; flex-direction: column; gap: 4px; }
.suite-name { font-weight: 500; color: #333; }
.suite-desc { font-size: 12px; color: #666; }
.case-index { width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; background: #e0e0e0; border-radius: 50%; font-size: 12px; color: #666; }
.case-name { flex: 1; margin-left: 8px; }
.case-actions { display: flex; gap: 4px; }
.btn-icon { background: none; border: none; cursor: pointer; font-size: 14px; padding: 4px; }
.run-item { padding: 12px; border: 1px solid #e0e0e0; border-radius: 4px; }
.run-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; }
.run-status { padding: 2px 8px; border-radius: 4px; font-size: 12px; font-weight: 500; }
.run-status.status-passed { background: #d4edda; color: #155724; }
.run-status.status-failed { background: #f8d7da; color: #721c24; }
.run-status.status-running { background: #fff3cd; color: #856404; }
.run-time { font-size: 12px; color: #666; }
.run-stats { display: flex; gap: 12px; font-size: 12px; }
.stat { color: #666; }
.stat.passed { color: #28a745; }
.stat.failed { color: #dc3545; }
.case-detail-modal { position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 1100; display: flex; align-items: center; justify-content: center; }
.modal-content { background: white; border-radius: 8px; width: 600px; max-height: 80vh; overflow: hidden; display: flex; flex-direction: column; }
.modal-header { padding: 16px; border-bottom: 1px solid #e0e0e0; display: flex; justify-content: space-between; align-items: center; }
.modal-header h3 { font-size: 16px; font-weight: 600; }
.modal-body { padding: 16px; overflow-y: auto; }
.script-section { margin-bottom: 16px; }
.script-section label { display: block; margin-bottom: 8px; font-weight: 500; color: #333; }
.script-section textarea { width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 4px; font-family: monospace; font-size: 13px; resize: vertical; }
.modal-footer { padding: 16px; border-top: 1px solid #e0e0e0; display: flex; justify-content: flex-end; gap: 12px; }
</style>
