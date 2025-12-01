import { useState, useEffect, useCallback } from 'react';
import {
  StoreSource,
  fetchStoreSources,
  addStoreSource,
  removeStoreSource,
  enableStoreSource,
  disableStoreSource,
  updateStoreSourcePriority,
} from '../lib/extensions';

interface UseStoreSourcesReturn {
  sources: StoreSource[];
  loading: boolean;
  error: string | null;
  fetchSources: () => Promise<void>;
  addSource: (source: Omit<StoreSource, 'id'>) => Promise<string>;
  removeSource: (sourceId: string) => Promise<void>;
  enableSource: (sourceId: string) => Promise<void>;
  disableSource: (sourceId: string) => Promise<void>;
  updatePriority: (sourceId: string, priority: number) => Promise<void>;
  getEnabledSources: () => StoreSource[];
}

export function useStoreSources(): UseStoreSourcesReturn {
  const [sources, setSources] = useState<StoreSource[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchSources = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const newSources = await fetchStoreSources();
      // Sort by priority
      newSources.sort((a, b) => a.priority - b.priority);
      setSources(newSources);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to fetch store sources'
      );
    } finally {
      setLoading(false);
    }
  }, []);

  const addSource = useCallback(
    async (source: Omit<StoreSource, 'id'>): Promise<string> => {
      try {
        setError(null);
        const sourceId = await addStoreSource(source);
        await fetchSources(); // Refresh the list
        return sourceId;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to add store source';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [fetchSources]
  );

  const removeSource = useCallback(async (sourceId: string): Promise<void> => {
    try {
      setError(null);
      await removeStoreSource(sourceId);
      setSources((prev) => prev.filter((s) => s.id !== sourceId));
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to remove store source';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  const enableSource = useCallback(async (sourceId: string): Promise<void> => {
    try {
      setError(null);
      await enableStoreSource(sourceId);
      setSources((prev) =>
        prev.map((s) => (s.id === sourceId ? { ...s, enabled: true } : s))
      );
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to enable store source';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  const disableSource = useCallback(async (sourceId: string): Promise<void> => {
    try {
      setError(null);
      await disableStoreSource(sourceId);
      setSources((prev) =>
        prev.map((s) => (s.id === sourceId ? { ...s, enabled: false } : s))
      );
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to disable store source';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  const updatePriority = useCallback(
    async (sourceId: string, priority: number): Promise<void> => {
      try {
        setError(null);
        await updateStoreSourcePriority(sourceId, priority);
        setSources((prev) =>
          prev
            .map((s) => (s.id === sourceId ? { ...s, priority } : s))
            .sort((a, b) => a.priority - b.priority)
        );
      } catch (err) {
        const errorMessage =
          err instanceof Error
            ? err.message
            : 'Failed to update source priority';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const getEnabledSources = useCallback(() => {
    return sources.filter((s) => s.enabled);
  }, [sources]);

  // Initial load
  useEffect(() => {
    fetchSources();
  }, [fetchSources]);

  return {
    sources,
    loading,
    error,
    fetchSources,
    addSource,
    removeSource,
    enableSource,
    disableSource,
    updatePriority,
    getEnabledSources,
  };
}
