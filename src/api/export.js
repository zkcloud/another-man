import { invoke } from "@tauri-apps/api/core";

export async function exportCollection(collectionId) {
  return await invoke("export_collection", { collectionId });
}

export async function importCollection(imported) {
  return await invoke("import_collection", { imported });
}