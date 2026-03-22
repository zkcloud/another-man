import { invoke } from "@tauri-apps/api/core";

export async function executePreRequestScript(script, envId = null) {
  return await invoke("execute_pre_request_script", { script, envId });
}

export async function executeTestScript(script, response, envId = null) {
  return await invoke("execute_test_script", { script, response, envId });
}

export async function validateScript(script) {
  return await invoke("validate_script", { script });
}