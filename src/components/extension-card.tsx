import {
  StoreExtension,
  getExtensionTypeDisplayNameFromString,
} from '@/lib/extensions';
import { useExtensions } from '@/hooks/use-extensions';
import { useExtensionStore } from '@/hooks/use-extension-store';
import { useStoreSources } from '@/hooks/use-store-sources';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Star, Download, User } from 'lucide-react';

interface ExtensionCardProps {
  extension: StoreExtension;
  onClick: () => void;
  onInstall: (id: string) => Promise<void>;
}

export function ExtensionCard({
  extension,
  onClick,
  onInstall,
}: ExtensionCardProps) {
  const { getExtensionById } = useExtensions();
  const { isUpdateAvailable } = useExtensionStore();
  const { sources } = useStoreSources();
  const installedExtension = getExtensionById(extension.id);
  const updateAvailable = isUpdateAvailable(extension);
  const source = sources.find((s) => s.id === extension.source_id);

  const handleInstall = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await onInstall(extension.id);
    } catch (error) {
      console.error('Failed to install extension:', error);
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

  return (
    <Card
      className='cursor-pointer transition-all hover:shadow-md'
      onClick={onClick}
    >
      <CardHeader className='pb-3'>
        <div className='flex items-start gap-2'>
          <div className='flex-1 min-w-0'>
            <CardTitle className='text-lg truncate'>{extension.name}</CardTitle>
            <CardDescription className='text-sm text-muted-foreground mt-1'>
              {extension.description}
            </CardDescription>
          </div>
          <div className='flex flex-col gap-1 shrink-0'>
            <Badge variant='secondary'>
              {getExtensionTypeDisplayNameFromString(extension.extension_type)}
            </Badge>
            {source && <Badge variant='outline'>{source.name}</Badge>}
          </div>
        </div>
      </CardHeader>

      <CardContent className='pt-0'>
        <div className='flex items-center gap-4 text-sm text-muted-foreground mb-3'>
          <div className='flex items-center gap-1'>
            <User className='h-4 w-4' />
            <span className='truncate'>{extension.author}</span>
          </div>
          <div className='flex items-center gap-1'>
            <Star className='h-4 w-4 fill-current' />
            <span>{extension.rating.toFixed(1)}</span>
          </div>
          <div className='flex items-center gap-1'>
            <Download className='h-4 w-4' />
            <span>{formatDownloads(extension.download_count)}</span>
          </div>
        </div>

        <div className='flex items-center justify-between'>
          <div className='flex flex-wrap gap-1'>
            {extension.tags.slice(0, 3).map((tag) => (
              <Badge key={tag} variant='outline' className='text-xs'>
                {tag}
              </Badge>
            ))}
            {extension.tags.length > 3 && (
              <Badge variant='outline' className='text-xs'>
                +{extension.tags.length - 3}
              </Badge>
            )}
          </div>

          <Button
            size='sm'
            onClick={handleInstall}
            disabled={!!installedExtension && !updateAvailable}
            className='shrink-0'
          >
            {updateAvailable
              ? 'Update'
              : installedExtension
              ? 'Installed'
              : 'Install'}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
