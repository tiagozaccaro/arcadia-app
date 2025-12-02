import { useAppSettings } from '@/hooks/use-app-settings';
import { useExtensionStore } from '@/hooks/use-extension-store';
import { useEffect, useState } from 'react';

export default function TestSettingsPage() {
  const { getSetting, setSetting } = useAppSettings();
  const {
    getExtensionSetting,
    setExtensionSetting,
    listExtensionSettings,
    deleteExtensionSetting,
  } = useExtensionStore();

  const [appKey, setAppKey] = useState('test_app_key');
  const [appValue, setAppValue] = useState('test_app_value');
  const [appResult, setAppResult] = useState('');

  const [extId, setExtId] = useState('test_extension');
  const [extKey, setExtKey] = useState('test_ext_key');
  const [extValue, setExtValue] = useState('test_ext_value');
  const [extResult, setExtResult] = useState('');
  const [extList, setExtList] = useState<Array<{ key: string; value: string }>>(
    []
  );

  // Automatic tests on mount
  useEffect(() => {
    const runTests = async () => {
      console.log('Running automatic settings tests...');

      // Test app settings CRUD
      try {
        // Create
        await setSetting('auto_test_key', 'auto_test_value');
        console.log('App setting set successfully');
        // Read
        let value = await getSetting('auto_test_key');
        console.log('App setting retrieved:', value);
        // Update
        await setSetting('auto_test_key', 'updated_value');
        value = await getSetting('auto_test_key');
        console.log('App setting updated:', value);
        setAppResult(`Auto test: CRUD successful, final value: ${value}`);
      } catch (error) {
        console.error('App settings test failed:', error);
        setAppResult(`Auto test failed: ${error}`);
      }

      // Test extension settings CRUD
      try {
        const extId = 'auto_test_ext';
        const key1 = 'auto_test_ext_key1';
        const key2 = 'auto_test_ext_key2';

        // Create multiple
        await setExtensionSetting(extId, key1, 'value1');
        await setExtensionSetting(extId, key2, 'value2');
        console.log('Extension settings set successfully');

        // Read
        const extValue1 = await getExtensionSetting(extId, key1);
        const extValue2 = await getExtensionSetting(extId, key2);
        console.log('Extension settings retrieved:', extValue1, extValue2);

        // List
        const list = await listExtensionSettings(extId);
        console.log('Extension settings listed:', list);

        // Delete
        await deleteExtensionSetting(extId, key1);
        const listAfterDelete = await listExtensionSettings(extId);
        console.log('Extension settings after delete:', listAfterDelete);

        setExtResult(
          `Auto test: CRUD successful, remaining: ${listAfterDelete.length}`
        );
        setExtList(listAfterDelete);
      } catch (error) {
        console.error('Extension settings test failed:', error);
        setExtResult(`Auto test failed: ${error}`);
      }
    };

    runTests();
  }, [
    setSetting,
    getSetting,
    setExtensionSetting,
    getExtensionSetting,
    listExtensionSettings,
    deleteExtensionSetting,
  ]);

  const testAppSet = async () => {
    try {
      await setSetting(appKey, appValue);
      setAppResult(`Set ${appKey} = ${appValue}`);
    } catch (error) {
      setAppResult(`Error setting: ${error}`);
    }
  };

  const testAppGet = async () => {
    try {
      const value = await getSetting(appKey);
      setAppResult(`Got ${appKey} = ${value}`);
    } catch (error) {
      setAppResult(`Error getting: ${error}`);
    }
  };

  const testExtSet = async () => {
    try {
      await setExtensionSetting(extId, extKey, extValue);
      setExtResult(`Set ${extId}:${extKey} = ${extValue}`);
    } catch (error) {
      setExtResult(`Error setting: ${error}`);
    }
  };

  const testExtGet = async () => {
    try {
      const value = await getExtensionSetting(extId, extKey);
      setExtResult(`Got ${extId}:${extKey} = ${value}`);
    } catch (error) {
      setExtResult(`Error getting: ${error}`);
    }
  };

  const testExtList = async () => {
    try {
      const settings = await listExtensionSettings(extId);
      setExtList(settings);
      setExtResult(`Listed ${settings.length} settings for ${extId}`);
    } catch (error) {
      setExtResult(`Error listing: ${error}`);
    }
  };

  const testExtDelete = async () => {
    try {
      await deleteExtensionSetting(extId, extKey);
      setExtResult(`Deleted ${extId}:${extKey}`);
    } catch (error) {
      setExtResult(`Error deleting: ${error}`);
    }
  };

  return (
    <div className='p-6 space-y-8'>
      <h1 className='text-2xl font-bold'>Settings Test Page</h1>

      <div className='space-y-4'>
        <h2 className='text-xl font-semibold'>App Settings</h2>
        <div className='flex gap-2'>
          <input
            type='text'
            value={appKey}
            onChange={(e) => setAppKey(e.target.value)}
            placeholder='Key'
            className='border p-2'
          />
          <input
            type='text'
            value={appValue}
            onChange={(e) => setAppValue(e.target.value)}
            placeholder='Value'
            className='border p-2'
          />
          <button
            onClick={testAppSet}
            className='bg-blue-500 text-white px-4 py-2'
          >
            Set
          </button>
          <button
            onClick={testAppGet}
            className='bg-green-500 text-white px-4 py-2'
          >
            Get
          </button>
        </div>
        <div className='text-sm text-gray-600'>{appResult}</div>
      </div>

      <div className='space-y-4'>
        <h2 className='text-xl font-semibold'>Extension Settings</h2>
        <div className='flex gap-2'>
          <input
            type='text'
            value={extId}
            onChange={(e) => setExtId(e.target.value)}
            placeholder='Extension ID'
            className='border p-2'
          />
          <input
            type='text'
            value={extKey}
            onChange={(e) => setExtKey(e.target.value)}
            placeholder='Key'
            className='border p-2'
          />
          <input
            type='text'
            value={extValue}
            onChange={(e) => setExtValue(e.target.value)}
            placeholder='Value'
            className='border p-2'
          />
          <button
            onClick={testExtSet}
            className='bg-blue-500 text-white px-4 py-2'
          >
            Set
          </button>
          <button
            onClick={testExtGet}
            className='bg-green-500 text-white px-4 py-2'
          >
            Get
          </button>
          <button
            onClick={testExtList}
            className='bg-purple-500 text-white px-4 py-2'
          >
            List
          </button>
          <button
            onClick={testExtDelete}
            className='bg-red-500 text-white px-4 py-2'
          >
            Delete
          </button>
        </div>
        <div className='text-sm text-gray-600'>{extResult}</div>
        {extList.length > 0 && (
          <div className='text-sm'>
            <strong>Settings:</strong>
            <ul>
              {extList.map((setting, i) => (
                <li key={i}>
                  {setting.key}: {setting.value}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
}
