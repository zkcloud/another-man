import { invoke } from "@tauri-apps/api/core";
import { getActiveEnvironment, substituteVariables } from "./environments.js";

export async function sendHttpRequest(request) {
  // Get active environment for variable substitution
  let env = null;
  try {
    env = await getActiveEnvironment();
  } catch (err) {
    console.error("Failed to get active environment:", err);
  }
  
  // Substitute variables in URL
  let processedRequest = { ...request };
  if (env?.variables) {
    processedRequest.url = substituteVariables(request.url, env.variables);
    
    // Substitute in headers
    processedRequest.headers = request.headers.map(h => ({
      ...h,
      value: substituteVariables(h.value, env.variables)
    }));
    
    // Substitute in body
    if (request.body) {
      processedRequest.body = substituteVariables(request.body, env.variables);
    }
  }
  
  return await invoke("send_http_request", { request: processedRequest });
}

export const HttpMethods = [
  "GET",
  "POST",
  "PUT",
  "DELETE",
  "PATCH",
  "HEAD",
  "OPTIONS",
];

export function createRequest(method = "GET", url = "") {
  return {
    id: crypto.randomUUID(),
    method,
    url,
    headers: [],
    query_params: [],
    body: null,
  };
}

export function createHeader(key = "", value = "", enabled = true) {
  return { key, value, enabled };
}

export function createParam(key = "", value = "", enabled = true) {
  return { key, value, enabled };
}