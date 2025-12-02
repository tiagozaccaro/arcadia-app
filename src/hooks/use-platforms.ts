import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

export interface Platform {
  id: number;
  name: string;
  description?: string;
  icon_path?: string;
  created_at: string;
  updated_at: string;
}

interface CreatePlatformParams extends Record<string, unknown> {
  name: string;
  description?: string;
  icon_path?: string;
}

interface UpdatePlatformParams extends Record<string, unknown> {
  id: number;
  name: string;
  description?: string;
  icon_path?: string;
}

interface UsePlatformsReturn {
  createPlatform: (params: CreatePlatformParams) => Promise<number>;
  getPlatforms: () => Promise<Platform[]>;
  updatePlatform: (params: UpdatePlatformParams) => Promise<void>;
  deletePlatform: (id: number) => Promise<void>;
}

export function usePlatforms(): UsePlatformsReturn {
  const createPlatform = useCallback(
    async (params: CreatePlatformParams): Promise<number> => {
      try {
        return await invoke<number>('create_platform_command', params);
      } catch (error) {
        throw new Error(`Failed to create platform: ${error}`);
      }
    },
    []
  );

  const getPlatforms = useCallback(async (): Promise<Platform[]> => {
    try {
      return await invoke<Platform[]>('get_platforms_command');
    } catch (error) {
      throw new Error(`Failed to get platforms: ${error}`);
    }
  }, []);

  const updatePlatform = useCallback(
    async (params: UpdatePlatformParams): Promise<void> => {
      try {
        await invoke('update_platform_command', params);
      } catch (error) {
        throw new Error(`Failed to update platform: ${error}`);
      }
    },
    []
  );

  const deletePlatform = useCallback(async (id: number): Promise<void> => {
    try {
      await invoke('delete_platform_command', { id });
    } catch (error) {
      throw new Error(`Failed to delete platform: ${error}`);
    }
  }, []);

  return { createPlatform, getPlatforms, updatePlatform, deletePlatform };
}
