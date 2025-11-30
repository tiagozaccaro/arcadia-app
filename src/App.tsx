import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { AppSidebar } from '@/components/app-sidebar';
import { SiteHeader } from '@/components/site-header';
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar';
import DashboardPage from '@/app/dashboard/page';
import ExtensionsPage from '@/app/extensions/page';
import './App.css';

export default function App() {
  return (
    <BrowserRouter>
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
            </Routes>
          </div>
        </SidebarInset>
      </SidebarProvider>
    </BrowserRouter>
  );
}
