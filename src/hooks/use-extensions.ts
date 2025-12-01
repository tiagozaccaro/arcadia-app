import { useState, useEffect, useCallback } from 'react';
import {
  ExtensionInfo,
  MenuItem,
  listExtensions,
  installExtension,
  uninstallExtension,
  enableExtension,
  disableExtension,
  callExtensionApi,
  getExtensionMenuItems,
} from '../lib/extensions';

interface UseExtensionsReturn {
  extensions: ExtensionInfo[];
  menuItems: MenuItem[];
  loading: boolean;
  error: string | null;
  refreshExtensions: () => Promise<void>;
  installExtension: (manifestPath: string) => Promise<string>;
  uninstallExtension: (extensionId: string) => Promise<void>;
  enableExtension: (extensionId: string) => Promise<void>;
  disableExtension: (extensionId: string) => Promise<void>;
  callExtensionApi: (
    extensionId: string,
    api: string,
    params?: any
  ) => Promise<any>;
  getExtensionById: (id: string) => ExtensionInfo | undefined;
  getExtensionsByType: (type: string) => ExtensionInfo[];
  getEnabledExtensions: () => ExtensionInfo[];
}

export function useExtensions(): UseExtensionsReturn {
  const [extensions, setExtensions] = useState<ExtensionInfo[]>([]);
  const [menuItems, setMenuItems] = useState<MenuItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refreshExtensions = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const extensionList = await listExtensions();
      const menuItemList = await getExtensionMenuItems();
      setExtensions(extensionList);
      setMenuItems(menuItemList);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : 'Failed to load extensions'
      );
    } finally {
      setLoading(false);
    }
  }, []);

  const handleInstallExtension = useCallback(
    async (manifestPath: string): Promise<string> => {
      try {
        setError(null);
        const extensionId = await installExtension(manifestPath);
        await refreshExtensions(); // Refresh the list after installation
        return extensionId;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to install extension';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [refreshExtensions]
  );

  const handleUninstallExtension = useCallback(
    async (extensionId: string): Promise<void> => {
      try {
        setError(null);
        await uninstallExtension(extensionId);
        await refreshExtensions(); // Refresh the list after uninstallation
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to uninstall extension';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    [refreshExtensions]
  );

  const handleEnableExtension = useCallback(
    async (extensionId: string): Promise<void> => {
      try {
        setError(null);
        await enableExtension(extensionId);
        // Update local state optimistically
        setExtensions((prev) =>
          prev.map((ext) =>
            ext.id === extensionId ? { ...ext, enabled: true } : ext
          )
        );
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to enable extension';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const handleDisableExtension = useCallback(
    async (extensionId: string): Promise<void> => {
      try {
        setError(null);
        await disableExtension(extensionId);
        // Update local state optimistically
        setExtensions((prev) =>
          prev.map((ext) =>
            ext.id === extensionId ? { ...ext, enabled: false } : ext
          )
        );
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to disable extension';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const handleCallExtensionApi = useCallback(
    async (
      extensionId: string,
      api: string,
      params: any = {}
    ): Promise<any> => {
      try {
        setError(null);
        return await callExtensionApi(extensionId, api, params);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to call extension API';
        setError(errorMessage);
        throw new Error(errorMessage);
      }
    },
    []
  );

  const getExtensionById = useCallback(
    (id: string): ExtensionInfo | undefined => {
      return extensions.find((ext) => ext.id === id);
    },
    [extensions]
  );

  const getExtensionsByType = useCallback(
    (type: string): ExtensionInfo[] => {
      return extensions.filter((ext) => ext.extension_type === type);
    },
    [extensions]
  );

  const getEnabledExtensions = useCallback((): ExtensionInfo[] => {
    return extensions.filter((ext) => ext.enabled);
  }, [extensions]);

  // Load extensions on mount
  useEffect(() => {
    refreshExtensions();
  }, [refreshExtensions]);

  return {
    extensions,
    menuItems,
    loading,
    error,
    refreshExtensions,
    installExtension: handleInstallExtension,
    uninstallExtension: handleUninstallExtension,
    enableExtension: handleEnableExtension,
    disableExtension: handleDisableExtension,
    callExtensionApi: handleCallExtensionApi,
    getExtensionById,
    getExtensionsByType,
    getEnabledExtensions,
  };
}
