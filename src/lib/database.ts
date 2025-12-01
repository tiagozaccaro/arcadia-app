import { invoke } from '@tauri-apps/api/core';

export async function getSetting(key: string): Promise<string> {
  return await invoke('get_setting', { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  await invoke('set_setting', { key, value });
}

export async function getAppData(dataType: string): Promise<string[]> {
  return await invoke('get_app_data', { dataType });
}

export async function saveAppData(
  dataType: string,
  data: string
): Promise<number> {
  return await invoke('save_app_data', { dataType, data });
}

export async function updateAppData(id: number, data: string): Promise<void> {
  await invoke('update_app_data', { id, data });
}

export async function deleteAppData(id: number): Promise<void> {
  await invoke('delete_app_data', { id });
}
