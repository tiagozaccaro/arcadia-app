import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

export interface Game {
  id: number;
  name: string;
  platform_id: number;
  description?: string;
  developer?: string;
  publisher?: string;
  release_date?: string;
  cover_image_path?: string;
  executable_path?: string;
  working_directory?: string;
  arguments?: string;
  is_favorite: boolean;
  playtime_minutes: number;
  last_played?: string;
  created_at: string;
  updated_at: string;
}

interface CreateGameParams extends Record<string, unknown> {
  name: string;
  platform_id: number;
  description?: string;
  developer?: string;
  publisher?: string;
  release_date?: string;
  cover_image_path?: string;
  executable_path?: string;
  working_directory?: string;
  arguments?: string;
}

interface UpdateGameParams extends Record<string, unknown> {
  id: number;
  name: string;
  platform_id: number;
  description?: string;
  developer?: string;
  publisher?: string;
  release_date?: string;
  cover_image_path?: string;
  executable_path?: string;
  working_directory?: string;
  arguments?: string;
}

interface UseGamesReturn {
  createGame: (params: CreateGameParams) => Promise<number>;
  getGames: () => Promise<Game[]>;
  getGamesByPlatform: (platform_id: number) => Promise<Game[]>;
  updateGame: (params: UpdateGameParams) => Promise<void>;
  deleteGame: (id: number) => Promise<void>;
}

export function useGames(): UseGamesReturn {
  const createGame = useCallback(
    async (params: CreateGameParams): Promise<number> => {
      try {
        return await invoke<number>('create_game_command', params);
      } catch (error) {
        throw new Error(`Failed to create game: ${error}`);
      }
    },
    []
  );

  const getGames = useCallback(async (): Promise<Game[]> => {
    try {
      return await invoke<Game[]>('get_games_command');
    } catch (error) {
      throw new Error(`Failed to get games: ${error}`);
    }
  }, []);

  const getGamesByPlatform = useCallback(
    async (platform_id: number): Promise<Game[]> => {
      try {
        return await invoke<Game[]>('get_games_by_platform_command', {
          platform_id,
        });
      } catch (error) {
        throw new Error(`Failed to get games by platform: ${error}`);
      }
    },
    []
  );

  const updateGame = useCallback(
    async (params: UpdateGameParams): Promise<void> => {
      try {
        await invoke('update_game_command', params);
      } catch (error) {
        throw new Error(`Failed to update game: ${error}`);
      }
    },
    []
  );

  const deleteGame = useCallback(async (id: number): Promise<void> => {
    try {
      await invoke('delete_game_command', { id });
    } catch (error) {
      throw new Error(`Failed to delete game: ${error}`);
    }
  }, []);

  return { createGame, getGames, getGamesByPlatform, updateGame, deleteGame };
}
