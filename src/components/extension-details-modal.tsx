import { useState } from 'react';
import {
  StoreExtensionDetails,
  getExtensionTypeDisplayNameFromString,
} from '@/lib/extensions';
import { useExtensions } from '@/hooks/use-extensions';
import { useExtensionStore } from '@/hooks/use-extension-store';
import { useStoreSources } from '@/hooks/use-store-sources';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from '@/components/ui/sheet';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Star, Download, User, Shield, Package, Globe } from 'lucide-react';

interface ExtensionDetailsModalProps {
  extension: StoreExtensionDetails | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onInstall: (id: string) => Promise<void>;
}

export function ExtensionDetailsModal({
  extension,
  open,
  onOpenChange,
  onInstall,
}: ExtensionDetailsModalProps) {
  const { getExtensionById } = useExtensions();
  const { isUpdateAvailable } = useExtensionStore();
  const { sources } = useStoreSources();
  const [installing, setInstalling] = useState(false);
  const source = extension
    ? sources.find((s) => s.id === extension.source_id)
    : null;

  const installedExtension = extension ? getExtensionById(extension.id) : null;
  const updateAvailable = extension ? isUpdateAvailable(extension) : false;

  const handleInstall = async () => {
    if (!extension) return;

    setInstalling(true);
    try {
      await onInstall(extension.id);
    } catch (error) {
      console.error('Failed to install extension:', error);
    } finally {
      setInstalling(false);
    }
  };

  const formatDownloads = (count: number) => {
    if (count >= 1000000) {
      return `${(count / 1000000).toFixed(1)}M`;
    }
    if (count >= 1000) {
      return `${(count / 1000).toFixed(1)}K`;
    }
    return count.toString();
  };

  if (!extension) return null;

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent className='w-full sm:max-w-2xl'>
        <SheetHeader>
          <div className='flex items-start justify-between'>
            <div className='flex-1 min-w-0'>
              <SheetTitle className='text-2xl'>{extension.name}</SheetTitle>
              <SheetDescription className='text-base mt-2'>
                {extension.description}
              </SheetDescription>
            </div>
            <Badge variant='secondary' className='ml-4 shrink-0'>
              {getExtensionTypeDisplayNameFromString(extension.extension_type)}
            </Badge>
          </div>
        </SheetHeader>

        <div className='flex-1 mt-6 overflow-y-auto'>
          <div className='space-y-6 pr-6'>
            {/* Stats */}
            <div className='flex items-center gap-6 text-sm'>
              <div className='flex items-center gap-2'>
                <User className='h-4 w-4 text-muted-foreground' />
                <span>{extension.author}</span>
              </div>
              <div className='flex items-center gap-2'>
                <Star className='h-4 w-4 text-muted-foreground fill-current' />
                <span>{extension.rating.toFixed(1)}</span>
              </div>
              <div className='flex items-center gap-2'>
                <Download className='h-4 w-4 text-muted-foreground' />
                <span>{formatDownloads(extension.download_count)}</span>
              </div>
              <div className='flex items-center gap-2'>
                <Package className='h-4 w-4 text-muted-foreground' />
                <span>v{extension.version}</span>
              </div>
              <div className='flex items-center gap-2'>
                <Globe className='h-4 w-4 text-muted-foreground' />
                <span>{source?.name || 'Unknown'}</span>
              </div>
            </div>

            {/* Tags */}
            {extension.tags.length > 0 && (
              <div>
                <h3 className='text-sm font-medium mb-2'>Tags</h3>
                <div className='flex flex-wrap gap-2'>
                  {extension.tags.map((tag) => (
                    <Badge key={tag} variant='outline'>
                      {tag}
                    </Badge>
                  ))}
                </div>
              </div>
            )}

            {/* Screenshots */}
            {extension.screenshots.length > 0 && (
              <div>
                <h3 className='text-sm font-medium mb-3'>Screenshots</h3>
                <div className='grid grid-cols-1 gap-4'>
                  {extension.screenshots.map((screenshot, index) => (
                    <img
                      key={index}
                      src={screenshot}
                      alt={`Screenshot ${index + 1}`}
                      className='w-full rounded-lg border'
                    />
                  ))}
                </div>
              </div>
            )}

            {/* Permissions */}
            {extension.manifest_url && (
              <div>
                <h3 className='text-sm font-medium mb-3 flex items-center gap-2'>
                  <Shield className='h-4 w-4' />
                  Permissions
                </h3>
                <div className='text-sm text-muted-foreground'>
                  This extension requires access to system resources. Please
                  review the permissions carefully before installing.
                </div>
              </div>
            )}

            {/* Dependencies */}
            {Object.keys(extension.dependencies).length > 0 && (
              <div>
                <h3 className='text-sm font-medium mb-3'>Dependencies</h3>
                <div className='space-y-2'>
                  {Object.entries(extension.dependencies).map(
                    ([name, version]) => (
                      <div
                        key={name}
                        className='flex items-center justify-between text-sm'
                      >
                        <span>{name}</span>
                        <Badge variant='outline'>{version}</Badge>
                      </div>
                    )
                  )}
                </div>
              </div>
            )}

            {/* README */}
            {extension.readme && (
              <div>
                <h3 className='text-sm font-medium mb-3'>About</h3>
                <div
                  className='prose prose-sm max-w-none text-sm'
                  dangerouslySetInnerHTML={{ __html: extension.readme }}
                />
              </div>
            )}
          </div>
        </div>

        <Separator className='my-6' />

        <div className='flex items-center justify-between'>
          <div className='text-sm text-muted-foreground'>
            {updateAvailable
              ? 'A newer version is available'
              : installedExtension
              ? 'This extension is already installed'
              : 'Ready to install'}
          </div>
          <Button
            onClick={handleInstall}
            disabled={(!!installedExtension && !updateAvailable) || installing}
            className='min-w-24'
          >
            {installing
              ? 'Installing...'
              : updateAvailable
              ? 'Update'
              : installedExtension
              ? 'Installed'
              : 'Install'}
          </Button>
        </div>
      </SheetContent>
    </Sheet>
  );
}
