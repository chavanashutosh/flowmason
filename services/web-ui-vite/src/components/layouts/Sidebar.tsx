import { Link, useLocation } from 'react-router-dom';
import {
  LayoutDashboard,
  Workflow,
  Sparkles,
  Clock,
  Calendar,
  TrendingUp,
  LinkIcon,
  BookOpen,
  Plus,
} from '../ui/icons';

interface MenuItem {
  href: string;
  label: string;
  icon: React.ReactNode;
}

export default function Sidebar() {
  const location = useLocation();

  const menuItems: MenuItem[] = [
    { href: '/', label: 'Dashboard', icon: <LayoutDashboard size={20} /> },
    { href: '/flows', label: 'Flows', icon: <Workflow size={20} /> },
    { href: '/templates', label: 'Templates', icon: <Sparkles size={20} /> },
    { href: '/executions', label: 'Executions', icon: <Clock size={20} /> },
    { href: '/scheduler', label: 'Scheduler', icon: <Calendar size={20} /> },
    { href: '/metering', label: 'Metering', icon: <TrendingUp size={20} /> },
    { href: '/mapping', label: 'Mapping', icon: <LinkIcon size={20} /> },
    { href: '/documentation', label: 'Documentation', icon: <BookOpen size={20} /> },
  ];

  const isActive = (path: string) => {
    if (path === '/') {
      return location.pathname === '/';
    }
    return location.pathname.startsWith(path);
  };

  return (
    <aside
      className="fixed left-0 top-0 z-40 h-screen transition-transform -translate-x-full lg:translate-x-0 bg-white border-r border-gray-200 w-64"
      aria-label="Sidebar navigation"
    >
      <div className="px-6 py-4 border-b border-gray-200">
        <Link to="/">
          <div className="flex items-center">
            <span className="text-lg font-semibold text-gray-900">FlowMason</span>
          </div>
        </Link>
      </div>

      <nav className="px-4 py-4 space-y-1">
        {/* Quick Actions Section */}
        <div className="mb-4 pb-4 border-b border-gray-200">
          <Link
            to="/flows/new"
            className="flex items-center justify-center w-full px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors mb-2"
          >
            <Plus size={16} className="mr-2" />
            <span>Create Flow</span>
          </Link>
          <Link
            to="/templates"
            className="flex items-center justify-center w-full px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          >
            <Sparkles size={16} className="mr-2 text-gray-600" />
            <span>Browse Templates</span>
          </Link>
        </div>

        {/* Main Navigation */}
        {menuItems.map((item) => (
          <Link
            key={item.href}
            to={item.href}
            className={`flex items-center px-4 py-2 text-sm font-medium rounded-lg transition-colors ${
              isActive(item.href)
                ? 'bg-gray-100 text-gray-900'
                : 'text-gray-700 hover:bg-gray-100'
            }`}
          >
            <div className="mr-3 text-gray-500">{item.icon}</div>
            <span>{item.label}</span>
          </Link>
        ))}
      </nav>
    </aside>
  );
}
