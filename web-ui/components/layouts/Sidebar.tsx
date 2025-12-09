'use client';

import { usePathname } from 'next/navigation';
import Link from 'next/link';
import { Sidebar as FlowbiteSidebar } from 'flowbite-react';
import {
  LayoutDashboard,
  GitBranch,
  Sparkles,
  Clock,
  Calendar,
  BarChart3,
  GitMerge,
  Menu,
} from 'lucide-react';
import { useState } from 'react';

export function Sidebar() {
  const pathname = usePathname();
  const [isCollapsed, setIsCollapsed] = useState(false);

  const menuItems = [
    {
      href: '/',
      label: 'Dashboard',
      icon: LayoutDashboard,
    },
    {
      href: '/flows',
      label: 'Flows',
      icon: GitBranch,
    },
    {
      href: '/templates',
      label: 'Templates',
      icon: Sparkles,
    },
    {
      href: '/executions',
      label: 'Executions',
      icon: Clock,
    },
    {
      href: '/scheduler',
      label: 'Scheduler',
      icon: Calendar,
    },
    {
      href: '/metering',
      label: 'Metering',
      icon: BarChart3,
    },
    {
      href: '/mapping',
      label: 'Mapping',
      icon: GitMerge,
    },
  ];

  return (
    <FlowbiteSidebar
      aria-label="Sidebar navigation"
      className="fixed left-0 top-0 z-40 h-screen transition-transform -translate-x-full lg:translate-x-0"
    >
      <FlowbiteSidebar.Logo href="/" img="/favicon.ico" imgAlt="FlowMason logo">
        <span className="ml-3 text-xl font-semibold text-gray-800 dark:text-white">
          FlowMason
        </span>
      </FlowbiteSidebar.Logo>
      <FlowbiteSidebar.Items>
        <FlowbiteSidebar.ItemGroup>
          {menuItems.map((item) => {
            const Icon = item.icon;
            const isActive = pathname === item.href || 
              (item.href !== '/' && pathname?.startsWith(item.href));
            
            return (
              <FlowbiteSidebar.Item
                key={item.href}
                href={item.href}
                icon={Icon}
                active={isActive}
                as={Link}
              >
                {item.label}
              </FlowbiteSidebar.Item>
            );
          })}
        </FlowbiteSidebar.ItemGroup>
      </FlowbiteSidebar.Items>
    </FlowbiteSidebar>
  );
}

