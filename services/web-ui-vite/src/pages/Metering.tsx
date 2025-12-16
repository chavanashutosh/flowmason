import { useEffect, useState } from 'react';
import { api } from '../api/client';
import { TrendingUp } from '../components/ui/icons';

interface UsageStat {
  brick_type: string;
  daily_usage: number;
  daily_limit: number;
  monthly_usage?: number;
  monthly_limit?: number;
}

export default function Metering() {
  const [stats, setStats] = useState<UsageStat[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchStats();
  }, []);

  const fetchStats = async () => {
    try {
      const data = await api.usageGetStats();
      if (Array.isArray(data)) {
        setStats(data as UsageStat[]);
      }
    } catch (error) {
      console.error('Failed to fetch usage stats:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <TrendingUp size={28} className="text-gray-500" />
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Usage & Metering</h1>
          <p className="text-gray-600 mt-1">Monitor usage and quotas for each brick type</p>
        </div>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">Loading...</div>
      ) : stats.length === 0 ? (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <div className="flex justify-center mb-6">
            <TrendingUp size={56} className="text-gray-300" />
          </div>
          <p className="text-gray-500">No usage data available</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {stats.map((stat) => (
            <UsageCard key={stat.brick_type} stat={stat} />
          ))}
        </div>
      )}
    </div>
  );
}

function UsageCard({ stat }: { stat: UsageStat }) {
  const dailyPercentage = Math.min((stat.daily_usage / stat.daily_limit) * 100, 100);
  const monthlyPercentage =
    stat.monthly_limit && stat.monthly_usage
      ? Math.min((stat.monthly_usage / stat.monthly_limit) * 100, 100)
      : 0;

  const progressColor =
    dailyPercentage >= 90 ? 'bg-red-600' : dailyPercentage >= 70 ? 'bg-yellow-600' : 'bg-green-600';

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900 capitalize">
          {stat.brick_type.replace('_', ' ')}
        </h3>
        <span className="px-2 py-1 text-xs font-semibold rounded bg-gray-100 text-gray-800">
          {stat.daily_usage} / {stat.daily_limit}
        </span>
      </div>

      <div className="space-y-6">
        <div>
          <div className="flex justify-between text-sm mb-3">
            <span className="text-gray-600">Daily Usage</span>
            <span className="font-medium text-gray-900">
              {stat.daily_usage} / {stat.daily_limit}
            </span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5">
            <div
              className={`${progressColor} h-2.5 rounded-full`}
              style={{ width: `${dailyPercentage}%` }}
            />
          </div>
        </div>

        {stat.monthly_limit && (
          <div>
            <div className="flex justify-between text-sm mb-3">
              <span className="text-gray-600">Monthly Usage</span>
              <span className="font-medium text-gray-900">
                {stat.monthly_usage || 0} / {stat.monthly_limit}
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2.5">
              <div
                className={`${progressColor} h-2.5 rounded-full`}
                style={{ width: `${monthlyPercentage}%` }}
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
