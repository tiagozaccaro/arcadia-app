import { ExtensionManager } from '@/components/extension-manager';

export default function ExtensionsPage() {
  return (
    <div className='@container/main flex flex-1 flex-col gap-2'>
      <ExtensionManager />
    </div>
  );
}
