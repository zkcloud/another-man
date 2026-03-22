import { invoke } from "@tauri-apps/api/core";

// Collection operations
export async function createCollection(input) {
  return await invoke("create_collection", { input });
}

export async function getCollections() {
  return await invoke("get_collections");
}

export async function updateCollection(id, name, description) {
  return await invoke("update_collection", { id, name, description });
}

export async function deleteCollection(id) {
  return await invoke("delete_collection", { id });
}

// Saved Request operations
export async function createSavedRequest(input) {
  return await invoke("create_saved_request", { input });
}

export async function getRequestsByCollection(collectionId) {
  return await invoke("get_requests_by_collection", { collectionId });
}

export async function deleteSavedRequest(id) {
  return await invoke("delete_saved_request", { id });
}