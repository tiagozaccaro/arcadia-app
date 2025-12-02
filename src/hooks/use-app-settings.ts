import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

interface UseAppSettingsReturn {
  getSetting: (key: string) => Promise<string>;
  setSetting: (key: string, value: string) => Promise<void>;
}

export function useAppSettings(): UseAppSettingsReturn {
  const getSetting = useCallback(async (key: string): Promise<string> => {
    try {
      return await invoke<string>('get_setting', { key });
    } catch (error) {
      throw new Error(`Failed to get setting: ${error}`);
    }
  }, []);

  const setSetting = useCallback(
    async (key: string, value: string): Promise<void> => {
      try {
        await invoke('set_setting', { key, value });
      } catch (error) {
        throw new Error(`Failed to set setting: ${error}`);
      }
    },
    []
  );

  return { getSetting, setSetting };
}
