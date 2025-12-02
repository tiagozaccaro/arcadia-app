import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import {
  SortOption,
  StoreExtension,
  StoreExtensionDetails,
  StoreFilters,
  fetchExtensionDetails,
  fetchStoreExtensions,
  installFromStore,
  isVersionGreater,
} from '../lib/extensions';
import { useExtensions } from './use-extensions';
import { useStoreSources } from './use-store-sources';

interface UseExtensionStoreReturn {
  extensions: StoreExtension[];
  loading: boolean;
  error: string | null;
  hasMore: boolean;
  currentPage: number;
  filters: StoreFilters;
  sort: SortOption;
  fetchExtensions: (reset?: boolean) => Promise<void>;
  setFilters: (filters: Partial<StoreFilters>) => void;
  setSort: (sort: SortOption) => void;
  loadMore: () => Promise<void>;
  getExtensionDetails: (id: string) => Promise<StoreExtensionDetails>;
  installExtension: (id: string) => Promise<string>;
  isUpdateAvailable: (extension: StoreExtension) => boolean;
  getExtensionSetting: (extensionId: string, key: string) => Promise<string>;
  setExtensionSetting: (
    extensionId: string,
    key: string,
    value: string
  ) => Promise<void>;
  listExtensionSettings: (
    extensionId: string
  ) => Promise<Array<{ key: string; value: string }>>;
  deleteExtensionSetting: (extensionId: string, key: string) => Promise<void>;
}

const PAGE_SIZE = 20;

export function useExtensionStore(): UseExtensionStoreReturn {
  console.log('useExtensionStore hook initialized');
  const { getExtensionById } = useExtensions();
  const { getEnabledSources } = useStoreSources();
  const [extensions, setExtensions] = useState<StoreExtension[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [hasMore, setHasMore] = useState(true);
  const [currentPage, setCurrentPage] = useState(0);
  const [filters, setFiltersState] = useState<StoreFilters>({});
  const [sort, setSortState] = useState<SortOption>(SortOption.Name);

  const fetchExtensions = useCallback(
    async (reset = false) => {
      try {
        setLoading(true);
        setError(null);

        const page = reset ? 0 : currentPage;
        // Prepare filters with source_ids if not specified
        const effectiveFilters = { ...filters };
        if (!effectiveFilters.source_ids) {
          const enabledSources = getEnabledSources();
          effectiveFilters.source_ids = enabledSources.map((s) => s.id);
        }
        console.log('Calling fetchStoreExtensions with', {
          effectiveFilters,
          sort,
          page,
          PAGE_SIZE,
        });
        const newExtensions = await fetchStoreExtensions(
          effectiveFilters,
          sort,
          page,
          PAGE_SIZE
        );
        console.log('Received extensions:', newExtensions);
        console.log('Number of extensions received:', newExtensions.length);
        if (newExtensions.length === 0) {
          console.log(
            'No extensions received. Checking filters:',
            effectiveFilters
          );
          console.log('Enabled sources:', getEnabledSources());
        }

        if (reset) {
          setExtensions(newExtensions);
          setCurrentPage(1);
          setHasMore(newExtensions.length === PAGE_SIZE);
        } else {
          setExtensions((prev) => [...prev, ...newExtensions]);
          setCurrentPage((prev) => prev + 1);
          setHasMore(newExtensions.length === PAGE_SIZE);
        }
      } catch (err) {
        console.error('Error fetching extensions:', err);
        setError(
          err instanceof Error ? err.message : 'Failed to fetch extensions'
        );
      } finally {
        setLoading(false);
      }
    },
    [currentPage, filters, sort, getEnabledSources]
  );

  const setFilters = useCallback((newFilters: Partial<StoreFilters>) => {
    setFiltersState((prev) => ({ ...prev, ...newFilters }));
    setCurrentPage(0);
    setExtensions([]);
    setHasMore(true);
  }, []);

  const setSort = useCallback((newSort: SortOption) => {
    setSortState(newSort);
    setCurrentPage(0);
    setExtensions([]);
    setHasMore(true);
  }, []);

  const loadMore = useCallback(async () => {
    if (!loading && hasMore) {
      await fetchExtensions(false);
    }
  }, [loading, hasMore, fetchExtensions]);

  const getExtensionDetails = useCallback(
    async (id: string): Promise<StoreExtensionDetails> => {
      try {
        setError(null);
        return await fetchExtensionDetails(id);
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : 'Failed to fetch extension details';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const isUpdateAvailable = useCallback(
    (storeExtension: StoreExtension): boolean => {
      const installedExtension = getExtensionById(storeExtension.id);
      if (!installedExtension) return false;
      return isVersionGreater(
        storeExtension.version,
        installedExtension.version
      );
    },
    [getExtensionById]
  );

  const installExtension = useCallback(
    async (id: string): Promise<string> => {
      try {
        setError(null);
        const extensionId = await installFromStore(id);
        // Refresh the extensions list to reflect the installation
        await fetchExtensions(true);
        return extensionId;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to install extension';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [fetchExtensions]
  );

  const getExtensionSetting = useCallback(
    async (extensionId: string, key: string): Promise<string> => {
      try {
        setError(null);
        return await invoke<string>('get_extension_setting', {
          extensionId,
          key,
        });
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : 'Failed to get extension setting';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const setExtensionSetting = useCallback(
    async (extensionId: string, key: string, value: string): Promise<void> => {
      try {
        setError(null);
        await invoke('set_extension_setting', { extensionId, key, value });
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : 'Failed to set extension setting';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const listExtensionSettings = useCallback(
    async (
      extensionId: string
    ): Promise<Array<{ key: string; value: string }>> => {
      try {
        setError(null);
        const settings = await invoke<[string, string][]>(
          'list_extension_settings',
          { extensionId }
        );
        return settings.map(([key, value]) => ({ key, value }));
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : 'Failed to list extension settings';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const deleteExtensionSetting = useCallback(
    async (extensionId: string, key: string): Promise<void> => {
      try {
        setError(null);
        await invoke('delete_extension_setting', { extensionId, key });
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : 'Failed to delete extension setting';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  // Initial load
  useEffect(() => {
    fetchExtensions(true);
  }, [fetchExtensions]); // Include fetchExtensions in dependencies

  // Refetch when filters or sort change
  useEffect(() => {
    if (currentPage === 0) {
      fetchExtensions(true);
    }
  }, [filters, sort, fetchExtensions, currentPage]);

  return {
    extensions,
    loading,
    error,
    hasMore,
    currentPage,
    filters,
    sort,
    fetchExtensions,
    setFilters,
    setSort,
    loadMore,
    getExtensionDetails,
    installExtension,
    isUpdateAvailable,
    getExtensionSetting,
    setExtensionSetting,
    listExtensionSettings,
    deleteExtensionSetting,
  };
}
