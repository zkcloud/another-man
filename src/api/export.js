import { invoke } from "@tauri-apps/api/core";
import { save, open } from "@tauri-apps/plugin-fs";

export async function exportCollection(collectionId) {
  return await invoke("export_collection", { collectionId });
}

export async function importCollection(imported) {
  return await invoke("import_collection", { imported });
}

export async function saveCollectionToFile(collectionId, collectionName) {
  const data = await exportCollection(collectionId);
  const json = JSON.stringify(data, null, 2);
  
  const fileName = `${collectionName.replace(/\s+/g, '-').toLowerCase()}.json`;
  
  await save({
    defaultPath: fileName,
    filters: [{ name: "JSON", extensions: ["json"] }],
  }, { contents: json });
  
  return true;
}

export async function loadCollectionFromFile() {
  const selected = await open({
    multiple: false,
    filters: [{ name: "JSON", extensions: ["json"] }],
  });
  
  if (!selected) return null;
  
  // Read file contents
  const { readFile } = await import("@tauri-apps/plugin-fs");
  const contents = await readFile(selected);
  const text = new TextDecoder().decode(contents);
  const data = JSON.parse(text);
  
  return data;
}