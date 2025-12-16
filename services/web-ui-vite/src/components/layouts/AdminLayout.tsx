import { Outlet } from 'react-router-dom';
import Sidebar from './Sidebar';
import TopNav from './TopNav';

export default function AdminLayout() {
  return (
    <div className="flex h-screen bg-gray-50">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden lg:ml-64">
        <TopNav />
        <main className="flex-1 overflow-y-auto pt-16 px-8 py-8">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
