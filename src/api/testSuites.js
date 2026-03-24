import { invoke } from "@tauri-apps/api/core";

// Test Suite APIs
export async function getTestSuites() {
  return await invoke("get_test_suites");
}

export async function createTestSuite(name, description = null, collectionId = null) {
  return await invoke("create_test_suite", {
    name,
    description,
    collectionId
  });
}

export async function deleteTestSuite(id) {
  return await invoke("delete_test_suite", { id });
}

// Test Case APIs
export async function getTestCases(suiteId) {
  return await invoke("get_test_cases", { suiteId });
}

export async function createTestCase(suiteId, name, requestId = null) {
  return await invoke("create_test_case", {
    suiteId,
    name,
    requestId
  });
}

export async function updateTestCaseScript(id, preScript = null, testScript = null) {
  return await invoke("update_test_case_script", {
    id,
    preScript,
    testScript
  });
}

export async function deleteTestCase(id) {
  return await invoke("delete_test_case", { id });
}

// Test Run APIs
export async function getTestRuns(suiteId) {
  return await invoke("get_test_runs", { suiteId });
}
