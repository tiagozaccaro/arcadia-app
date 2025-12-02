import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { useExtensionStore } from '@/hooks/use-extension-store';
import { useExtensions } from '@/hooks/use-extensions';
import { useStoreSources } from '@/hooks/use-store-sources';
import {
  StoreExtension,
  getExtensionTypeDisplayNameFromString,
} from '@/lib/extensions';
import { Database, Download, Palette, Puzzle, Star, User } from 'lucide-react';

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
  const displaySourceName = source?.id === 'default' ? 'Arcadia' : source?.name;

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

  const getExtensionIcon = () => {
    const iconClass = 'h-8 w-8';
    if (extension.icon) {
      switch (extension.icon) {
        case 'steam':
          return <Database className={`${iconClass} text-blue-600`} />;
        default:
          return <Puzzle className={`${iconClass} text-gray-500`} />;
      }
    }
    // Fallback to type-based icons
    switch (extension.extension_type) {
      case 'GameLibrary':
        return <Puzzle className={`${iconClass} text-blue-500`} />;
      case 'DataSource':
        return <Database className={`${iconClass} text-green-500`} />;
      case 'Theme':
        return <Palette className={`${iconClass} text-purple-500`} />;
      default:
        return <Puzzle className={`${iconClass} text-gray-500`} />;
    }
  };

  return (
    <Card
      className='cursor-pointer transition-all hover:shadow-md overflow-hidden'
      onClick={onClick}
    >
      <CardHeader className='pb-3'>
        <div className='flex flex-col items-center gap-2'>
          <div className='shrink-0'>{getExtensionIcon()}</div>
          <div className='flex-1 min-w-0 text-center'>
            <CardTitle className='text-lg' style={{ wordBreak: 'break-word' }}>
              {extension.name}
            </CardTitle>
            <CardDescription
              className='text-sm text-muted-foreground mt-1 overflow-hidden'
              style={{
                display: '-webkit-box',
                WebkitLineClamp: 2,
                WebkitBoxOrient: 'vertical',
                // Fallback for browsers that don't support WebkitLineClamp
                maxHeight: '3em', // Approximately 2 lines
                lineHeight: '1.5em',
                wordBreak: 'break-word',
              }}
            >
              {extension.description}
            </CardDescription>
          </div>
          <div className='flex gap-1'>
            <Badge variant='secondary'>
              {getExtensionTypeDisplayNameFromString(extension.extension_type)}
            </Badge>
            {source && <Badge variant='outline'>{displaySourceName}</Badge>}
          </div>
        </div>
      </CardHeader>

      <CardContent className='pt-0'>
        <div className='flex items-center gap-4 text-sm text-muted-foreground mb-3'>
          <div className='flex items-center gap-1'>
            <User className='h-4 w-4' />
            <span className='truncate'>
              {extension.author || 'Arcadia Team'}
            </span>
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

        <div className='flex items-end justify-between gap-2'>
          <div className='flex flex-wrap gap-1 min-w-0 flex-1 overflow-hidden'>
            {extension.tags.slice(0, 3).map((tag) => (
              <Badge
                key={tag}
                variant='outline'
                className='text-xs truncate max-w-24'
              >
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
