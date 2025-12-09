'use client';

import { Sidebar } from './Sidebar';
import { TopNav } from './TopNav';

export function AdminLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-screen bg-gray-50 dark:bg-gray-900">
      <Sidebar />
      
      <div className="flex-1 flex flex-col overflow-hidden lg:ml-64">
        <TopNav />
        
        <main className="flex-1 overflow-y-auto pt-16 p-6">
          {children}
        </main>
      </div>
    </div>
  );
}
