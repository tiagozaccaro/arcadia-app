import ExtensionsPage from '@/app/extensions/page';
import ExtensionStorePage from '@/app/extensions/store/page';
import TestGamesPlatformsPage from '@/app/test-games-platforms/page';
import TestSettingsPage from '@/app/test-settings/page';
import { AppSidebar } from '@/components/app-sidebar';
import { SiteHeader } from '@/components/site-header';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { useEffect } from 'react';
import { BrowserRouter, Route, Routes, useLocation } from 'react-router-dom';
import './App.css';

// Route to title mapping
const routeTitles: Record<string, string> = {
  '/': 'Games & Platforms',
  '/extensions': 'Extensions',
  '/extensions/store': 'Extension Store',
  '/test-settings': 'Settings',
  '/test-games-platforms': 'Games & Platforms',
};

function AppContent() {
  const location = useLocation();
  const currentTitle = routeTitles[location.pathname] || 'Arcadia';

  useEffect(() => {
    console.log('Route changed to:', location.pathname);
    document.title = currentTitle;
  }, [location, currentTitle]);

  return (
    <SidebarProvider
      style={
        {
          '--sidebar-width': 'calc(var(--spacing) * 72)',
        } as React.CSSProperties
      }
    >
      <AppSidebar variant='inset' />
      <SidebarInset>
        <SiteHeader title={currentTitle} />
        <div className='flex flex-1 flex-col'>
          <Routes>
            <Route path='/' element={<TestGamesPlatformsPage />} />
            <Route path='/extensions' element={<ExtensionsPage />} />
            <Route path='/extensions/store' element={<ExtensionStorePage />} />
            <Route path='/test-settings' element={<TestSettingsPage />} />
            <Route
              path='/test-games-platforms'
              element={<TestGamesPlatformsPage />}
            />
          </Routes>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}

export default function App() {
  console.log('App component rendering');
  return (
    <BrowserRouter>
      <AppContent />
    </BrowserRouter>
  );
}
