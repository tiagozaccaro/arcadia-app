import { useState } from 'react';
import {
  IconPlus,
  IconTrash,
  IconPlayerPlay,
  IconPlayerPause,
  IconDotsVertical,
  IconInfoCircle,
} from '@tabler/icons-react';
import { toast } from 'sonner';

import { useExtensions } from '@/hooks/use-extensions';
import {
  ExtensionInfo,
  getExtensionTypeDisplayNameFromString,
} from '@/lib/extensions';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@/components/ui/sheet';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';

export function ExtensionManager() {
  const {
    extensions,
    loading,
    error,
    installExtension,
    uninstallExtension,
    enableExtension,
    disableExtension,
    refreshExtensions,
  } = useExtensions();

  const [installDialogOpen, setInstallDialogOpen] = useState(false);
  const [manifestPath, setManifestPath] = useState('');

  const handleInstall = async () => {
    if (!manifestPath.trim()) {
      toast.error('Please enter a manifest path');
      return;
    }

    try {
      await installExtension(manifestPath);
      toast.success('Extension installed successfully');
      setInstallDialogOpen(false);
      setManifestPath('');
    } catch (err) {
      toast.error('Failed to install extension');
    }
  };

  const handleUninstall = async (
    extensionId: string,
    extensionName: string
  ) => {
    try {
      await uninstallExtension(extensionId);
      toast.success(`Extension "${extensionName}" uninstalled successfully`);
    } catch (err) {
      toast.error('Failed to uninstall extension');
    }
  };

  const handleToggleEnabled = async (extension: ExtensionInfo) => {
    try {
      if (extension.enabled) {
        await disableExtension(extension.id);
        toast.success(`Extension "${extension.name}" disabled`);
      } else {
        await enableExtension(extension.id);
        toast.success(`Extension "${extension.name}" enabled`);
      }
    } catch (err) {
      toast.error(
        `Failed to ${extension.enabled ? 'disable' : 'enable'} extension`
      );
    }
  };

  if (loading) {
    return (
      <div className='flex items-center justify-center p-8'>
        <div className='text-muted-foreground'>Loading extensions...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className='flex flex-col items-center justify-center p-8 gap-4'>
        <div className='text-destructive'>
          Error loading extensions: {error}
        </div>
        <Button onClick={refreshExtensions} variant='outline'>
          Retry
        </Button>
      </div>
    );
  }

  return (
    <div className='flex flex-col gap-6 p-6'>
      <div className='flex items-center justify-between'>
        <div>
          <h1 className='text-2xl font-semibold'>Extension Manager</h1>
          <p className='text-muted-foreground'>
            Manage installed extensions and install new ones
          </p>
        </div>
        <div className='flex items-center gap-2'>
          <Button onClick={refreshExtensions} variant='outline' size='sm'>
            Refresh
          </Button>
          <Sheet open={installDialogOpen} onOpenChange={setInstallDialogOpen}>
            <SheetTrigger asChild>
              <Button size='sm'>
                <IconPlus className='size-4' />
                Install Extension
              </Button>
            </SheetTrigger>
            <SheetContent>
              <SheetHeader>
                <SheetTitle>Install Extension</SheetTitle>
                <SheetDescription>
                  Enter the path to the extension manifest file (.json) to
                  install a new extension.
                </SheetDescription>
              </SheetHeader>
              <div className='grid gap-4 py-4'>
                <div className='grid gap-2'>
                  <Label htmlFor='manifest-path'>Manifest Path</Label>
                  <Input
                    id='manifest-path'
                    placeholder='/path/to/extension/manifest.json'
                    value={manifestPath}
                    onChange={(e) => setManifestPath(e.target.value)}
                  />
                </div>
              </div>
              <SheetFooter>
                <Button
                  variant='outline'
                  onClick={() => setInstallDialogOpen(false)}
                >
                  Cancel
                </Button>
                <Button onClick={handleInstall}>Install</Button>
              </SheetFooter>
            </SheetContent>
          </Sheet>
        </div>
      </div>

      {extensions.length === 0 ? (
        <Card>
          <CardContent className='flex flex-col items-center justify-center py-12'>
            <IconInfoCircle className='size-12 text-muted-foreground mb-4' />
            <h3 className='text-lg font-medium mb-2'>
              No extensions installed
            </h3>
            <p className='text-muted-foreground text-center mb-4'>
              Get started by installing your first extension.
            </p>
            <Button onClick={() => setInstallDialogOpen(true)}>
              <IconPlus className='size-4' />
              Install Extension
            </Button>
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <CardTitle>Installed Extensions</CardTitle>
            <CardDescription>
              {extensions.length} extension{extensions.length !== 1 ? 's' : ''}{' '}
              installed
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Name</TableHead>
                  <TableHead>Version</TableHead>
                  <TableHead>Type</TableHead>
                  <TableHead>Author</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead className='w-12'></TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {extensions.map((extension) => (
                  <TableRow key={extension.id}>
                    <TableCell>
                      <div className='flex flex-col'>
                        <span className='font-medium'>{extension.name}</span>
                        {extension.description && (
                          <span className='text-sm text-muted-foreground'>
                            {extension.description}
                          </span>
                        )}
                      </div>
                    </TableCell>
                    <TableCell>
                      <Badge variant='outline'>{extension.version}</Badge>
                    </TableCell>
                    <TableCell>
                      <Badge variant='secondary'>
                        {getExtensionTypeDisplayNameFromString(
                          extension.extension_type
                        )}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      {extension.author || (
                        <span className='text-muted-foreground'>Unknown</span>
                      )}
                    </TableCell>
                    <TableCell>
                      <Badge
                        variant={extension.enabled ? 'default' : 'secondary'}
                      >
                        {extension.enabled ? 'Enabled' : 'Disabled'}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant='ghost' size='icon'>
                            <IconDotsVertical className='size-4' />
                            <span className='sr-only'>Open menu</span>
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align='end'>
                          <DropdownMenuItem
                            onClick={() => handleToggleEnabled(extension)}
                          >
                            {extension.enabled ? (
                              <>
                                <IconPlayerPause className='size-4' />
                                Disable
                              </>
                            ) : (
                              <>
                                <IconPlayerPlay className='size-4' />
                                Enable
                              </>
                            )}
                          </DropdownMenuItem>
                          <DropdownMenuSeparator />
                          <DropdownMenuItem
                            variant='destructive'
                            onClick={() => {
                              if (
                                window.confirm(
                                  `Are you sure you want to uninstall "${extension.name}"? This action cannot be undone.`
                                )
                              ) {
                                handleUninstall(extension.id, extension.name);
                              }
                            }}
                          >
                            <IconTrash className='size-4' />
                            Uninstall
                          </DropdownMenuItem>
                        </DropdownMenuContent>
                      </DropdownMenu>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
