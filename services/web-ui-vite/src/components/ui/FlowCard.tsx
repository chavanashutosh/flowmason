import { ReactNode } from 'react';

interface FlowCardProps {
  title: string;
  children: ReactNode;
  className?: string;
  actions?: ReactNode;
}

export default function FlowCard({ title, children, className = '', actions }: FlowCardProps) {
  return (
    <div className={`bg-white rounded-lg shadow p-6 ${className}`}>
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900">{title}</h3>
        {actions && <div>{actions}</div>}
      </div>
      {children}
    </div>
  );
}
