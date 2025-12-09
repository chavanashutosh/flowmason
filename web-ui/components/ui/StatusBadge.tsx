'use client';

import { Badge } from 'flowbite-react';

interface StatusBadgeProps {
  status: 'completed' | 'failed' | 'running' | 'pending' | 'active' | 'inactive';
}

export function StatusBadge({ status }: StatusBadgeProps) {
  const statusConfig = {
    completed: { color: 'success', label: 'Completed' },
    failed: { color: 'failure', label: 'Failed' },
    running: { color: 'info', label: 'Running' },
    pending: { color: 'warning', label: 'Pending' },
    active: { color: 'success', label: 'Active' },
    inactive: { color: 'gray', label: 'Inactive' },
  };

  const config = statusConfig[status] || statusConfig.pending;

  return <Badge color={config.color as any}>{config.label}</Badge>;
}

