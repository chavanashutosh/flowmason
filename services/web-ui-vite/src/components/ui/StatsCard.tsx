import { ReactNode } from 'react';

interface Trend {
  value: string;
  isPositive: boolean;
}

interface StatsCardProps {
  title: string;
  value: string;
  icon?: ReactNode;
  trend?: Trend;
}

export default function StatsCard({ title, value, icon, trend }: StatsCardProps) {
  return (
    <div className="h-full bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow p-6">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <p className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">{title}</p>
          <p className="text-5xl font-bold text-gray-900">{value}</p>
          {trend && (
            <p className={`text-sm mt-3 font-medium ${trend.isPositive ? 'text-green-600' : 'text-red-600'}`}>
              {trend.isPositive ? '↑' : '↓'} {trend.value}
            </p>
          )}
        </div>
        {icon && (
          <div className="p-3 rounded-lg bg-primary-50 text-primary-600 ml-4 flex-shrink-0">
            {icon}
          </div>
        )}
      </div>
    </div>
  );
}
