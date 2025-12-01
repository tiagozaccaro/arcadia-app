'use client';

import { useState, useEffect } from 'react';
import { useExtensionStore } from '@/hooks/use-extension-store';
import { useStoreSources } from '@/hooks/use-store-sources';
import { ExtensionCard } from '@/components/extension-card';
import { ExtensionDetailsModal } from '@/components/extension-details-modal';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@/components/ui/sheet';
import { Label } from '@/components/ui/label';
import {
  StoreExtensionDetails,
  ExtensionType,
  SortOption,
  StoreSource,
  StoreSourceType,
  getStoreSourceTypeDisplayName,
} from '@/lib/extensions';
import { Search, X, Plus, Trash2, Power, PowerOff } from 'lucide-react';

export default function ExtensionStorePage() {
  const {
    extensions,
    loading,
    error,
    hasMore,
    filters,
    sort,
    setFilters,
    setSort,
    loadMore,
    getExtensionDetails,
    installExtension,
  } = useExtensionStore();

  const [selectedExtension, setSelectedExtension] =
    useState<StoreExtensionDetails | null>(null);
  const [modalOpen, setModalOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedType, setSelectedType] = useState<ExtensionType | 'all'>(
    'all'
  );
  const [selectedSource, setSelectedSource] = useState<string>('all');
  const {
    sources,
    loading: sourcesLoading,
    error: sourcesError,
    addSource,
    removeSource,
    enableSource,
    disableSource,
  } = useStoreSources();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [newSource, setNewSource] = useState({
    name: '',
    type: StoreSourceType.Community,
    url: '',
    enabled: true,
    priority: 0,
  });
  const [addSourceLoading, setAddSourceLoading] = useState(false);
  const [addSourceError, setAddSourceError] = useState<string | null>(null);

  // Apply search filter with debounce
  useEffect(() => {
    const timer = setTimeout(() => {
      setFilters({
        ...filters,
        search: searchQuery || undefined,
      });
    }, 300);

    return () => clearTimeout(timer);
  }, [searchQuery, setFilters]);

  // Apply type filter
  useEffect(() => {
    setFilters({
      ...filters,
      extension_type: selectedType === 'all' ? undefined : selectedType,
    });
  }, [selectedType, setFilters]);

  // Apply source filter
  useEffect(() => {
    setFilters({
      ...filters,
      source_ids: selectedSource === 'all' ? undefined : [selectedSource],
    });
  }, [selectedSource, setFilters]);

  const handleCardClick = async (extensionId: string) => {
    try {
      const details = await getExtensionDetails(extensionId);
      setSelectedExtension(details);
      setModalOpen(true);
    } catch (error) {
      console.error('Failed to load extension details:', error);
    }
  };

  const handleInstall = async (extensionId: string) => {
    try {
      await installExtension(extensionId);
    } catch (error) {
      console.error('Failed to install extension:', error);
    }
  };

  const clearFilters = () => {
    setSearchQuery('');
    setSelectedType('all');
    setSelectedSource('all');
    setFilters({});
  };

  const hasActiveFilters =
    searchQuery || selectedType !== 'all' || selectedSource !== 'all';

  const handleAddSource = async () => {
    if (!newSource.name.trim() || !newSource.url.trim()) {
      setAddSourceError('Name and URL are required');
      return;
    }
    setAddSourceLoading(true);
    setAddSourceError(null);
    try {
      await addSource(newSource);
      setNewSource({
        name: '',
        type: StoreSourceType.Community,
        url: '',
        enabled: true,
        priority: 0,
      });
      setDialogOpen(false);
    } catch (error) {
      console.error('Failed to add source:', error);
      setAddSourceError('Failed to add source. Please try again.');
    } finally {
      setAddSourceLoading(false);
    }
  };

  const handleRemoveSource = async (sourceId: string) => {
    if (confirm('Are you sure you want to remove this source?')) {
      try {
        await removeSource(sourceId);
      } catch (error) {
        console.error('Failed to remove source:', error);
      }
    }
  };

  const handleToggleSource = async (source: StoreSource) => {
    try {
      if (source.enabled) {
        await disableSource(source.id);
      } else {
        await enableSource(source.id);
      }
    } catch (error) {
      console.error('Failed to toggle source:', error);
    }
  };

  return (
    <div className='@container/main flex flex-1 flex-col gap-2'>
      <div className='flex flex-col gap-6 p-6'>
        <Tabs defaultValue='browse' className='w-full'>
          <TabsList className='grid w-full grid-cols-2'>
            <TabsTrigger value='browse'>Browse</TabsTrigger>
            <TabsTrigger value='sources'>Sources</TabsTrigger>
          </TabsList>
          <TabsContent value='browse'>
            {/* Header */}
            <div className='flex flex-col gap-4'>
              <div>
                <h1 className='text-3xl font-bold'>Extension Store</h1>
                <p className='text-muted-foreground'>
                  Discover and install extensions to enhance your experience
                </p>
              </div>

              {/* Search and Filters */}
              <div className='flex flex-col gap-4 sm:flex-row sm:items-center'>
                <div className='relative flex-1'>
                  <Search className='absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground' />
                  <Input
                    placeholder='Search extensions...'
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    className='pl-9'
                  />
                </div>

                <div className='flex gap-2'>
                  <Select
                    value={selectedType}
                    onValueChange={(value) =>
                      setSelectedType(value as ExtensionType | 'all')
                    }
                  >
                    <SelectTrigger className='w-40'>
                      <SelectValue placeholder='Type' />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value='all'>All Types</SelectItem>
                      <SelectItem value={ExtensionType.Theme}>
                        UI Theme
                      </SelectItem>
                      <SelectItem value={ExtensionType.DataSource}>
                        Data Source
                      </SelectItem>
                      <SelectItem value={ExtensionType.GameLibrary}>
                        Game Library
                      </SelectItem>
                    </SelectContent>
                  </Select>

                  <Select
                    value={selectedSource}
                    onValueChange={setSelectedSource}
                  >
                    <SelectTrigger className='w-40'>
                      <SelectValue placeholder='Source' />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value='all'>All Sources</SelectItem>
                      {sources.map((source) => (
                        <SelectItem key={source.id} value={source.id}>
                          {source.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>

                  <Select
                    value={sort}
                    onValueChange={(value) => setSort(value as SortOption)}
                  >
                    <SelectTrigger className='w-32'>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value={SortOption.Name}>Name</SelectItem>
                      <SelectItem value={SortOption.DownloadCount}>
                        Downloads
                      </SelectItem>
                      <SelectItem value={SortOption.Rating}>Rating</SelectItem>
                      <SelectItem value={SortOption.Newest}>Newest</SelectItem>
                    </SelectContent>
                  </Select>

                  {hasActiveFilters && (
                    <Button
                      variant='outline'
                      size='icon'
                      onClick={clearFilters}
                    >
                      <X className='h-4 w-4' />
                    </Button>
                  )}
                </div>

                {/* Active Filters */}
                {hasActiveFilters && (
                  <div className='flex flex-wrap gap-2'>
                    {searchQuery && (
                      <Badge variant='secondary' className='gap-1'>
                        Search: {searchQuery}
                        <Button
                          variant='ghost'
                          size='icon'
                          className='h-4 w-4 p-0 hover:bg-transparent'
                          onClick={() => setSearchQuery('')}
                        >
                          <X className='h-3 w-3' />
                        </Button>
                      </Badge>
                    )}
                    {selectedType !== 'all' && (
                      <Badge variant='secondary' className='gap-1'>
                        Type: {selectedType.replace('_', ' ')}
                        <Button
                          variant='ghost'
                          size='icon'
                          className='h-4 w-4 p-0 hover:bg-transparent'
                          onClick={() => setSelectedType('all')}
                        >
                          <X className='h-3 w-3' />
                        </Button>
                      </Badge>
                    )}
                    {selectedSource !== 'all' && (
                      <Badge variant='secondary' className='gap-1'>
                        Source:{' '}
                        {sources.find((s) => s.id === selectedSource)?.name ||
                          selectedSource}
                        <Button
                          variant='ghost'
                          size='icon'
                          className='h-4 w-4 p-0 hover:bg-transparent'
                          onClick={() => setSelectedSource('all')}
                        >
                          <X className='h-3 w-3' />
                        </Button>
                      </Badge>
                    )}
                  </div>
                )}
              </div>

              {/* Content */}
              <div className='flex-1'>
                {error && (
                  <div className='rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive'>
                    {error}
                  </div>
                )}

                {loading && extensions.length === 0 ? (
                  <div className='grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4'>
                    {Array.from({ length: 8 }).map((_, i) => (
                      <div
                        key={i}
                        className='h-48 animate-pulse rounded-xl bg-muted'
                      />
                    ))}
                  </div>
                ) : extensions.length === 0 ? (
                  <div className='flex flex-col items-center justify-center py-12 text-center'>
                    <div className='text-6xl'>🔍</div>
                    <h3 className='mt-4 text-lg font-semibold'>
                      No extensions found
                    </h3>
                    <p className='text-muted-foreground'>
                      Try adjusting your search or filters
                    </p>
                    {hasActiveFilters && (
                      <Button
                        variant='outline'
                        className='mt-4'
                        onClick={clearFilters}
                      >
                        Clear filters
                      </Button>
                    )}
                  </div>
                ) : (
                  <>
                    <div className='grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4'>
                      {extensions.map((extension) => (
                        <ExtensionCard
                          key={extension.id}
                          extension={extension}
                          onClick={() => handleCardClick(extension.id)}
                          onInstall={handleInstall}
                        />
                      ))}
                    </div>

                    {/* Load More */}
                    {hasMore && (
                      <div className='mt-8 flex justify-center'>
                        <Button
                          variant='outline'
                          onClick={loadMore}
                          disabled={loading}
                        >
                          {loading ? 'Loading...' : 'Load More'}
                        </Button>
                      </div>
                    )}
                  </>
                )}
              </div>
            </div>
          </TabsContent>

          <TabsContent value='sources'>
            <div className='p-6'>
              {sourcesLoading ? (
                <div className='text-center'>Loading sources...</div>
              ) : (
                <div className='flex flex-col gap-6'>
                  <div className='flex items-center justify-between'>
                    <div>
                      <h2 className='text-2xl font-bold'>Store Sources</h2>

                      <p className='text-muted-foreground'>
                        Manage extension store sources
                      </p>
                    </div>

                    <Sheet open={dialogOpen} onOpenChange={setDialogOpen}>
                      <SheetTrigger asChild>
                        <Button>
                          <Plus className='mr-2 h-4 w-4' />
                          Add Source
                        </Button>
                      </SheetTrigger>

                      <SheetContent>
                        <SheetHeader>
                          <SheetTitle>Add New Store Source</SheetTitle>

                          <SheetDescription>
                            Configure a new extension store source.
                          </SheetDescription>
                        </SheetHeader>

                        <div className='p-6'>
                          <div className='grid gap-4 py-4'>
                            <div className='grid grid-cols-4 items-center gap-4'>
                              <Label htmlFor='name' className='text-right'>
                                Name
                              </Label>

                              <Input
                                id='name'
                                value={newSource.name}
                                onChange={(e) =>
                                  setNewSource({
                                    ...newSource,
                                    name: e.target.value,
                                  })
                                }
                                className='col-span-3'
                              />
                            </div>

                            <div className='grid grid-cols-4 items-center gap-4'>
                              <Label htmlFor='type' className='text-right'>
                                Type
                              </Label>

                              <Select
                                value={newSource.type}
                                onValueChange={(value) =>
                                  setNewSource({
                                    ...newSource,

                                    type: value as StoreSourceType,
                                  })
                                }
                              >
                                <SelectTrigger className='col-span-3'>
                                  <SelectValue />
                                </SelectTrigger>

                                <SelectContent>
                                  <SelectItem value={StoreSourceType.Community}>
                                    Community
                                  </SelectItem>

                                  <SelectItem
                                    value={StoreSourceType.ThirdParty}
                                  >
                                    Custom
                                  </SelectItem>
                                </SelectContent>
                              </Select>
                            </div>

                            <div className='grid grid-cols-4 items-center gap-4'>
                              <Label htmlFor='url' className='text-right'>
                                URL
                              </Label>

                              <Input
                                id='url'
                                value={newSource.url}
                                onChange={(e) =>
                                  setNewSource({
                                    ...newSource,
                                    url: e.target.value,
                                  })
                                }
                                className='col-span-3'
                              />
                            </div>

                            {addSourceError && (
                              <div className='text-destructive text-sm'>
                                {addSourceError}
                              </div>
                            )}
                          </div>

                          <SheetFooter>
                            <Button
                              onClick={handleAddSource}
                              disabled={addSourceLoading}
                            >
                              {addSourceLoading ? 'Adding...' : 'Add Source'}
                            </Button>
                          </SheetFooter>
                        </div>
                      </SheetContent>
                    </Sheet>
                  </div>

                  {sourcesError && (
                    <div className='rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive'>
                      {sourcesError}
                    </div>
                  )}

                  <div className='grid gap-4'>
                    {sources.map((source) => (
                      <Card key={source.id}>
                        <CardHeader className='pb-3'>
                          <div className='flex items-center justify-between'>
                            <div className='flex items-center gap-3'>
                              <CardTitle className='text-lg'>
                                {source.name}
                              </CardTitle>

                              <Badge variant='secondary'>
                                {getStoreSourceTypeDisplayName(source.type)}
                              </Badge>

                              <Badge
                                variant={
                                  source.enabled ? 'default' : 'secondary'
                                }
                              >
                                {source.enabled ? 'Enabled' : 'Disabled'}
                              </Badge>
                            </div>

                            <div className='flex items-center gap-2'>
                              <Button
                                variant='outline'
                                size='sm'
                                onClick={() => handleToggleSource(source)}
                              >
                                {source.enabled ? (
                                  <>
                                    <PowerOff className='mr-2 h-4 w-4' />
                                    Disable
                                  </>
                                ) : (
                                  <>
                                    <Power className='mr-2 h-4 w-4' />
                                    Enable
                                  </>
                                )}
                              </Button>

                              <Button
                                variant='outline'
                                size='sm'
                                onClick={() => handleRemoveSource(source.id)}
                              >
                                <Trash2 className='h-4 w-4' />
                              </Button>
                            </div>
                          </div>
                        </CardHeader>

                        <CardContent>
                          <div className='text-sm text-muted-foreground'>
                            <p>URL: {source.url}</p>

                            <p>Priority: {source.priority}</p>
                          </div>
                        </CardContent>
                      </Card>
                    ))}
                  </div>

                  {sources.length === 0 && (
                    <div className='flex flex-col items-center justify-center py-12 text-center'>
                      <div className='text-6xl'>🔍</div>

                      <h3 className='mt-4 text-lg font-semibold'>
                        No sources configured
                      </h3>

                      <p className='text-muted-foreground'>
                        Add your first extension store source to get started.
                      </p>
                    </div>
                  )}
                </div>
              )}
            </div>
          </TabsContent>
        </Tabs>
      </div>
      {/* Details Modal */}
      <ExtensionDetailsModal
        extension={selectedExtension}
        open={modalOpen}
        onOpenChange={setModalOpen}
        onInstall={handleInstall}
      />
    </div>
  );
}
