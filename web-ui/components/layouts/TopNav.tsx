'use client';

import { Navbar, Dropdown, Avatar, TextInput } from 'flowbite-react';
import { Bell, Search, Moon, Sun, User, Settings, LogOut } from 'lucide-react';
import { useState, useEffect } from 'react';

export function TopNav() {
  const [darkMode, setDarkMode] = useState(false);

  useEffect(() => {
    const isDark = document.documentElement.classList.contains('dark');
    setDarkMode(isDark);
  }, []);

  const toggleDarkMode = () => {
    if (darkMode) {
      document.documentElement.classList.remove('dark');
      setDarkMode(false);
      localStorage.setItem('theme', 'light');
    } else {
      document.documentElement.classList.add('dark');
      setDarkMode(true);
      localStorage.setItem('theme', 'dark');
    }
  };

  useEffect(() => {
    const theme = localStorage.getItem('theme');
    if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.classList.add('dark');
      setDarkMode(true);
    }
  }, []);

  return (
    <Navbar fluid className="fixed top-0 z-30 w-full border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
      <div className="flex w-full items-center justify-between px-4 py-3">
        <div className="flex items-center gap-4">
          <Navbar.Toggle className="lg:hidden" />
          <TextInput
            type="search"
            placeholder="Search..."
            icon={Search}
            className="hidden md:block w-64"
          />
        </div>
        
        <div className="flex items-center gap-4">
          <button
            onClick={toggleDarkMode}
            className="rounded-lg p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
          >
            {darkMode ? <Sun className="h-5 w-5" /> : <Moon className="h-5 w-5" />}
          </button>

          <Dropdown
            label=""
            dismissOnClick={false}
            renderTrigger={() => (
              <button className="rounded-lg p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">
                <Bell className="h-5 w-5" />
              </button>
            )}
          >
            <Dropdown.Header>
              <span className="block text-sm">Notifications</span>
            </Dropdown.Header>
            <Dropdown.Item>No new notifications</Dropdown.Item>
          </Dropdown>

          <Dropdown
            label=""
            dismissOnClick={false}
            renderTrigger={() => (
              <Avatar
                alt="User settings"
                img="https://flowbite.com/docs/images/people/profile-picture-5.jpg"
                rounded
                size="sm"
              />
            )}
          >
            <Dropdown.Header>
              <span className="block text-sm">Bonnie Green</span>
              <span className="block truncate text-sm font-medium">name@flowmason.com</span>
            </Dropdown.Header>
            <Dropdown.Item icon={User}>Profile</Dropdown.Item>
            <Dropdown.Item icon={Settings}>Settings</Dropdown.Item>
            <Dropdown.Divider />
            <Dropdown.Item icon={LogOut}>Sign out</Dropdown.Item>
          </Dropdown>
        </div>
      </div>
    </Navbar>
  );
}

