'use client';

import { Card } from 'flowbite-react';
import { ReactNode } from 'react';

interface FlowCardProps {
  title?: string;
  children: ReactNode;
  className?: string;
  actions?: ReactNode;
}

export function FlowCard({ title, children, className = '', actions }: FlowCardProps) {
  return (
    <Card className={className}>
      {(title || actions) && (
        <div className="flex items-center justify-between mb-4">
          {title && (
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white">{title}</h3>
          )}
          {actions && <div>{actions}</div>}
        </div>
      )}
      {children}
    </Card>
  );
}

