import { invoke } from '@tauri-apps/api/core';

// Extension Types
export enum ExtensionType {
  Theme = 'Theme',
  DataSource = 'DataSource',
  GameLibrary = 'GameLibrary',
}

// Menu Item for extensions
export interface MenuItem {
  title: string;
  url: string;
  icon?: string; // Icon name, e.g., 'Puzzle', 'Bot', etc.
  items?: {
    title: string;
    url: string;
  }[];
}

// Extension Manifest
export interface ExtensionManifest {
  name: string;
  version: string;
  author?: string;
  description?: string;
  type: ExtensionType;
  entryPoint: string;
  permissions: string[];
  dependencies?: Record<string, string>;
  hooks?: string[];
  apis?: ExtensionApis;
  menuItems?: MenuItem[];
}

// Extension APIs
export interface ExtensionApis {
  provided?: string[];
  required?: string[];
}

// Extension Info (returned from backend)
export interface ExtensionInfo {
  id: string;
  name: string;
  version: string;
  author?: string;
  description?: string;
  extension_type: string;
  enabled: boolean;
}

// Extension Permission
export interface ExtensionPermission {
  extension_id: string;
  permission: string;
  granted: boolean;
}

// Extension Setting
export interface ExtensionSetting {
  extension_id: string;
  key: string;
  value?: string;
}

// Store Extension Types
export interface StoreExtension {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  extension_type: ExtensionType;
  source_id: string;
  icon?: string;
  download_count: number;
  rating: number;
  tags: string[];
}

export interface StoreExtensionDetails {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  extension_type: ExtensionType;
  source_id: string;
  download_count: number;
  rating: number;
  tags: string[];
  manifest_url: string;
  package_url: string;
  checksum: string;
  readme: string;
  screenshots: string[];
  dependencies: Record<string, string>;
}

export enum StoreSourceType {
  Official = 'official',
  Community = 'community',
  ThirdParty = 'third_party',
}

export interface StoreSource {
  id: string;
  name: string;
  type: StoreSourceType;
  base_url: string;
  enabled: boolean;
  priority: number;
}

export interface StoreFilters {
  extension_type?: ExtensionType;
  tags?: string[];
  search?: string;
  source_ids?: string[];
}

export enum SortOption {
  Name = 'Name',
  DownloadCount = 'DownloadCount',
  Rating = 'Rating',
  Newest = 'Newest',
}

// Tauri Command Wrappers

/**
 * Install an extension from a manifest path
 * @param manifestPath Path to the extension manifest file
 * @returns The extension ID
 */
export async function installExtension(manifestPath: string): Promise<string> {
  return await invoke('install_extension', { manifestPath });
}

/**
 * Uninstall an extension
 * @param extensionId The extension ID to uninstall
 */
export async function uninstallExtension(extensionId: string): Promise<void> {
  return await invoke('uninstall_extension', { extensionId });
}

/**
 * Enable an extension
 * @param extensionId The extension ID to enable
 */
export async function enableExtension(extensionId: string): Promise<void> {
  return await invoke('enable_extension', { extensionId });
}

/**
 * Disable an extension
 * @param extensionId The extension ID to disable
 */
export async function disableExtension(extensionId: string): Promise<void> {
  return await invoke('disable_extension', { extensionId });
}

/**
 * List all installed extensions
 * @returns Array of extension info
 */
export async function listExtensions(): Promise<ExtensionInfo[]> {
  return await invoke('list_extensions');
}

/**
 * Call an API on an extension
 * @param extensionId The extension ID
 * @param api The API method name
 * @param params Parameters to pass to the API
 * @returns The API response
 */
export async function callExtensionApi(
  extensionId: string,
  api: string,
  params: any = {}
): Promise<any> {
  return await invoke('call_extension_api', { extensionId, api, params });
}

// Utility functions

/**
 * Get extension type from string
 */
export function getExtensionTypeFromString(type: string): ExtensionType {
  switch (type) {
    case 'theme':
      return ExtensionType.Theme;
    case 'data_source':
      return ExtensionType.DataSource;
    case 'game_library':
      return ExtensionType.GameLibrary;
    default:
      return ExtensionType.Theme;
  }
}

/**
 * Get display name for extension type
 */
export function getExtensionTypeDisplayName(type: ExtensionType): string {
  switch (type) {
    case ExtensionType.Theme:
      return 'UI Theme';
    case ExtensionType.DataSource:
      return 'Data Source';
    case ExtensionType.GameLibrary:
      return 'Game Library';
  }
}

/**
 * Get display name for extension type from string
 */
export function getExtensionTypeDisplayNameFromString(type: string): string {
  switch (type) {
    case 'theme':
      return 'UI Theme';
    case 'data_source':
      return 'Data Source';
    case 'game_library':
      return 'Game Library';
    default:
      return 'Unknown';
  }
}

/**
 * Get display name for store source type
 */
export function getStoreSourceTypeDisplayName(type: StoreSourceType): string {
  switch (type) {
    case StoreSourceType.Official:
      return 'Official';
    case StoreSourceType.Community:
      return 'Community';
    case StoreSourceType.ThirdParty:
      return 'Third Party';
  }
}

/**
 * Get menu items from all enabled extensions
 * @returns Array of menu items
 */
export async function getExtensionMenuItems(): Promise<MenuItem[]> {
  return await invoke('get_extension_menu_items');
}

/**
 * Check if a permission is valid
 */
export function isValidPermission(permission: string): boolean {
  const validPermissions = [
    'filesystem',
    'network',
    'database',
    'ui',
    'native',
  ];
  return validPermissions.includes(permission);
}

/**
 * Compare two semantic version strings
 * Returns: -1 if v1 < v2, 0 if v1 == v2, 1 if v1 > v2
 */
export function compareVersions(v1: string, v2: string): number {
  const parts1 = v1.split('.').map(Number);
  const parts2 = v2.split('.').map(Number);

  const maxLength = Math.max(parts1.length, parts2.length);

  for (let i = 0; i < maxLength; i++) {
    const part1 = parts1[i] || 0;
    const part2 = parts2[i] || 0;

    if (part1 < part2) return -1;
    if (part1 > part2) return 1;
  }

  return 0;
}

/**
 * Check if version1 is greater than version2 (for update availability)
 */
export function isVersionGreater(v1: string, v2: string): boolean {
  return compareVersions(v1, v2) > 0;
}

/**
 * Fetch extensions from the store
 * @param filters Filters to apply
 * @param sort Sort option
 * @param page Page number (0-based)
 * @param limit Number of items per page
 * @returns Array of store extensions
 */
export async function fetchStoreExtensions(
  filters: StoreFilters,
  sort: SortOption,
  page: number,
  limit: number
): Promise<StoreExtension[]> {
  console.log('extensions.ts: fetchStoreExtensions called with', {
    filters,
    sort,
    page,
    limit,
  });
  try {
    const result = await invoke<StoreExtension[]>('fetch_store_extensions', {
      filters,
      sort,
      page,
      limit,
    });
    console.log('extensions.ts: fetchStoreExtensions received', result);
    return result;
  } catch (err) {
    console.error('extensions.ts: fetchStoreExtensions error:', err);
    throw err;
  }
}

/**
 * Fetch detailed information about a store extension
 * @param extensionId The extension ID
 * @returns Detailed extension information
 */
export async function fetchExtensionDetails(
  extensionId: string
): Promise<StoreExtensionDetails> {
  return await invoke('fetch_extension_details', {
    sourceId: 'default',
    extensionId,
  });
}

/**
 * Install an extension from the store
 * @param extensionId The extension ID to install
 * @returns The installed extension ID
 */
export async function installFromStore(extensionId: string): Promise<string> {
  return await invoke('install_from_store', { extensionId });
}

// Store Source Management

/**
 * Fetch all store sources
 * @returns Array of store sources
 */
export async function fetchStoreSources(): Promise<StoreSource[]> {
  console.log(
    'extensions.ts: fetchStoreSources calling invoke list_store_sources'
  );
  try {
    const result = await invoke<StoreSource[]>('list_store_sources');
    console.log('extensions.ts: fetchStoreSources received:', result);
    return result;
  } catch (err) {
    console.error('extensions.ts: fetchStoreSources error:', err);
    throw err;
  }
}

/**
 * Add a new store source
 * @param source The store source to add
 * @returns The added store source ID
 */
export async function addStoreSource(
  source: Omit<StoreSource, 'id'>
): Promise<string> {
  return await invoke('add_store_source', { source });
}

/**
 * Remove a store source
 * @param sourceId The source ID to remove
 */
export async function removeStoreSource(sourceId: string): Promise<void> {
  return await invoke('remove_store_source', { sourceId });
}

/**
 * Enable a store source
 * @param sourceId The source ID to enable
 */
export async function enableStoreSource(sourceId: string): Promise<void> {
  return await invoke('enable_store_source', { sourceId });
}

/**
 * Disable a store source
 * @param sourceId The source ID to disable
 */
export async function disableStoreSource(sourceId: string): Promise<void> {
  return await invoke('disable_store_source', { sourceId });
}

/**
 * Update store source priority
 * @param sourceId The source ID
 * @param priority The new priority
 */
export async function updateStoreSourcePriority(
  sourceId: string,
  priority: number
): Promise<void> {
  return await invoke('update_store_source_priority', { sourceId, priority });
}
