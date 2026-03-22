import { invoke } from "@tauri-apps/api/core";

export async function sendHttpRequest(request) {
  return await invoke("send_http_request", { request });
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