import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { api, Flow, Execution, ScheduledFlow } from '../api/client';
import { StatsCard, EmptyState, EmptyStateIcon, StatusBadge, Status, OnboardingPanel } from '../components/ui';
import { Workflow, Clock, Calendar, TrendingUp } from '../components/ui/icons';

interface DashboardStats {
  totalFlows: number;
  totalExecutions: number;
  scheduledFlows: number;
  recentExecutions: Execution[];
}

export default function Dashboard() {
  const [stats, setStats] = useState<DashboardStats>({
    totalFlows: 0,
    totalExecutions: 0,
    scheduledFlows: 0,
    recentExecutions: [],
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [flows, executions, scheduled] = await Promise.all([
          api.flowsList(),
          api.executionsList(),
          api.schedulerListScheduledFlows(),
        ]);

        const recentExecutions = executions.slice(0, 5);

        setStats({
          totalFlows: flows.length,
          totalExecutions: executions.length,
          scheduledFlows: scheduled.length,
          recentExecutions,
        });
      } catch (error) {
        console.error('Failed to fetch dashboard data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  return (
    <div className="space-y-8">
      {/* Title row with action button */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Dashboard</h1>
          <p className="text-sm text-gray-500 mt-1">Overview of your automation platform</p>
        </div>
        <Link to="/flows/new">
          <button className="px-6 py-2.5 bg-primary-600 hover:bg-primary-700 text-white font-medium rounded-lg transition-colors h-10">
            Create Flow
          </button>
        </Link>
      </div>

      <OnboardingPanel />

      {loading ? (
        <div className="flex items-center justify-center h-64">Loading...</div>
      ) : (
        <div className="space-y-8">
          {/* Metrics grid */}
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
            <StatsCard
              title="Total Flows"
              value={stats.totalFlows.toString()}
              icon={<Workflow size={24} className="text-primary-600" />}
            />
            <StatsCard
              title="Total Executions"
              value={stats.totalExecutions.toString()}
              icon={<Clock size={24} className="text-primary-600" />}
            />
            <StatsCard
              title="Scheduled Flows"
              value={stats.scheduledFlows.toString()}
              icon={<Calendar size={24} className="text-primary-600" />}
            />
            <StatsCard
              title="Usage Today"
              value="0"
              icon={<TrendingUp size={24} className="text-primary-600" />}
            />
          </div>

          {/* Recent Executions */}
          <div className="bg-white border border-gray-200 rounded-lg shadow-sm">
            <div className="px-6 py-4 border-b border-gray-200">
              <h2 className="text-xl font-semibold text-gray-900">Recent Executions</h2>
            </div>
            <div className="p-6">
              {stats.recentExecutions.length === 0 ? (
                <EmptyState
                  title="No recent executions"
                  description="Run a flow to see execution history here."
                  actionLabel="Create Flow"
                  actionRoute="/flows/new"
                  icon={EmptyStateIcon.Clock}
                />
              ) : (
                <div className="overflow-x-auto">
                  <table className="min-w-full divide-y divide-gray-200">
                    <thead className="bg-gray-50">
                      <tr>
                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                          Execution ID
                        </th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                          Status
                        </th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                          Created
                        </th>
                      </tr>
                    </thead>
                    <tbody className="bg-white divide-y divide-gray-200">
                      {stats.recentExecutions.map((execution) => (
                        <tr key={execution.execution_id} className="hover:bg-gray-50">
                          <td className="px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900">
                            {execution.execution_id.substring(0, 12)}...
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap">
                            <StatusBadge status={execution.status} />
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                            {execution.created_at}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
