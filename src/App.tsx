import DashboardPage from '@/app/dashboard/page';
import ExtensionsPage from '@/app/extensions/page';
import ExtensionStorePage from '@/app/extensions/store/page';
import { AppSidebar } from '@/components/app-sidebar';
import { SiteHeader } from '@/components/site-header';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import { useEffect } from 'react';
import { BrowserRouter, Route, Routes, useLocation } from 'react-router-dom';
import './App.css';

function AppContent() {
  const location = useLocation();

  useEffect(() => {
    console.log('Route changed to:', location.pathname);
  }, [location]);

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
        <SiteHeader />
        <div className='flex flex-1 flex-col'>
          <Routes>
            <Route path='/' element={<DashboardPage />} />
            <Route path='/extensions' element={<ExtensionsPage />} />
            <Route path='/extensions/store' element={<ExtensionStorePage />} />
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
