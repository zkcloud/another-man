import { invoke } from "@tauri-apps/api/core";

export async function createEnvironment(input) {
  return await invoke("create_environment", { input });
}

export async function getEnvironments() {
  return await invoke("get_environments");
}

export async function getEnvironment(id) {
  return await invoke("get_environment", { id });
}

export async function updateEnvironment(id, name, variables, isGlobal) {
  return await invoke("update_environment", { id, name, variables, isGlobal });
}

export async function deleteEnvironment(id) {
  return await invoke("delete_environment", { id });
}

export async function getActiveEnvironment() {
  return await invoke("get_active_environment");
}

// Variable substitution utility
export function substituteVariables(text, variables) {
  if (!text || !variables) return text;
  
  return text.replace(/\{\{([^}]+)\}\}/g, (match, varName) => {
    const trimmedName = varName.trim();
    return variables[trimmedName] || match;
  });
}