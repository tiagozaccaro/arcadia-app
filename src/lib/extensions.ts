import { invoke } from '@tauri-apps/api/core';

// Extension Types
export enum ExtensionType {
  Theme = 'theme',
  DataSource = 'data_source',
  GameLibrary = 'game_library',
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
