'use client';

import { useEffect, useState } from 'react';
import { Card, Table, Progress, Badge, Spinner } from 'flowbite-react';
import { BarChart3 } from 'lucide-react';
import { api } from '@/lib/api';

interface UsageStats {
  brick_type: string;
  daily_usage: number;
  daily_limit: number;
  monthly_usage?: number;
  monthly_limit?: number;
}

export default function MeteringPage() {
  const [stats, setStats] = useState<UsageStats[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchStats();
  }, []);

  const fetchStats = async () => {
    try {
      const data = await api.usage.getStats();
      setStats(data);
    } catch (error) {
      console.error('Failed to fetch usage stats:', error);
    } finally {
      setLoading(false);
    }
  };

  const getUsagePercentage = (usage: number, limit: number) => {
    return Math.min((usage / limit) * 100, 100);
  };

  const getProgressColor = (percentage: number) => {
    if (percentage >= 90) return 'failure';
    if (percentage >= 70) return 'warning';
    return 'success';
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <Spinner size="xl" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <BarChart3 className="text-orange-600 dark:text-orange-400" size={32} />
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Usage & Metering</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            Monitor usage and quotas for each brick type
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {stats.map((stat) => {
          const dailyPercentage = getUsagePercentage(stat.daily_usage, stat.daily_limit);
          const monthlyPercentage = stat.monthly_limit && stat.monthly_usage
            ? getUsagePercentage(stat.monthly_usage, stat.monthly_limit)
            : 0;

          return (
            <Card key={stat.brick_type}>
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold text-gray-900 dark:text-white capitalize">
                  {stat.brick_type.replace(/_/g, ' ')}
                </h3>
                <Badge color={getProgressColor(dailyPercentage) as any}>
                  {stat.daily_usage} / {stat.daily_limit}
                </Badge>
              </div>

              <div className="space-y-4">
                <div>
                  <div className="flex justify-between text-sm mb-2">
                    <span className="text-gray-600 dark:text-gray-400">Daily Usage</span>
                    <span className="font-medium text-gray-900 dark:text-white">
                      {stat.daily_usage} / {stat.daily_limit}
                    </span>
                  </div>
                  <Progress
                    progress={dailyPercentage}
                    color={getProgressColor(dailyPercentage) as any}
                    size="lg"
                  />
                </div>

                {stat.monthly_limit && (
                  <div>
                    <div className="flex justify-between text-sm mb-2">
                      <span className="text-gray-600 dark:text-gray-400">Monthly Usage</span>
                      <span className="font-medium text-gray-900 dark:text-white">
                        {stat.monthly_usage || 0} / {stat.monthly_limit}
                      </span>
                    </div>
                    <Progress
                      progress={monthlyPercentage}
                      color={getProgressColor(monthlyPercentage) as any}
                      size="lg"
                    />
                  </div>
                )}
              </div>
            </Card>
          );
        })}
      </div>

      {stats.length === 0 && (
        <Card>
          <div className="text-center py-12">
            <BarChart3 className="h-12 w-12 mx-auto mb-4 text-gray-400" />
            <p className="text-gray-500 dark:text-gray-400">No usage data available</p>
          </div>
        </Card>
      )}
    </div>
  );
}
