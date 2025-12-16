import { useState } from 'react';
import { Link } from 'react-router-dom';
import { User, Settings, LogOut } from '../ui/icons';

export default function TopNav() {
  const [menuOpen, setMenuOpen] = useState(false);

  const toggleMenu = () => {
    setMenuOpen(!menuOpen);
  };

  const closeMenu = () => {
    setMenuOpen(false);
  };

  return (
    <nav className="fixed top-0 right-0 left-64 z-30 bg-white border-b border-gray-200 h-16 flex items-center px-8">
      <div className="flex-1 flex items-center justify-between">
        <div className="flex items-center">
          <h2 className="text-xl font-semibold text-gray-900">FlowMason</h2>
        </div>
        <div className="flex items-center space-x-4">
          <div className="relative">
            <button
              onClick={toggleMenu}
              className="flex items-center justify-center w-10 h-10 rounded-full bg-gray-100 hover:bg-gray-200 transition-colors text-gray-700"
            >
              <User size={20} />
            </button>
            {menuOpen && (
              <div
                id="user-menu-dropdown"
                className="absolute right-0 top-12 mt-2 w-48 bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-50"
                onClick={(e) => e.stopPropagation()}
              >
                <Link
                  to="/"
                  className="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  onClick={closeMenu}
                >
                  <User size={16} className="mr-3" />
                  <span>Profile</span>
                </Link>
                <Link
                  to="/settings"
                  className="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  onClick={closeMenu}
                >
                  <Settings size={16} className="mr-3" />
                  <span>Settings</span>
                </Link>
                <div className="border-t border-gray-200 my-1" />
                <button
                  className="w-full flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 text-left"
                  onClick={closeMenu}
                >
                  <LogOut size={16} className="mr-3" />
                  <span>Logout</span>
                </button>
              </div>
            )}
          </div>
        </div>
      </div>
      {menuOpen && (
        <div
          className="fixed inset-0 z-40"
          onClick={closeMenu}
        />
      )}
    </nav>
  );
}
