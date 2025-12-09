'use client';

import { useEffect, useState } from 'react';
import { Card, Table, Button, Spinner } from 'flowbite-react';
import { 
  GitBranch, 
  Clock, 
  Calendar, 
  BarChart3,
  Plus,
  ArrowUpRight,
} from 'lucide-react';
import Link from 'next/link';
import { api } from '@/lib/api';
import { StatsCard } from '@/components/ui/StatsCard';
import { StatusBadge } from '@/components/ui/StatusBadge';

export default function Dashboard() {
  const [stats, setStats] = useState({
    totalFlows: 0,
    totalExecutions: 0,
    scheduledFlows: 0,
    recentExecutions: [] as any[],
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchDashboardData();
  }, []);

  const fetchDashboardData = async () => {
    try {
      const [flows, executions, scheduled] = await Promise.all([
        api.flows.list().catch(() => []),
        api.executions.list().catch(() => []),
        api.scheduler.listScheduledFlows().catch(() => ({ flows: [] })),
      ]);

      setStats({
        totalFlows: flows.length || 0,
        totalExecutions: executions.length || 0,
        scheduledFlows: scheduled.flows?.length || 0,
        recentExecutions: executions.slice(0, 5) || [],
      });
    } catch (error) {
      console.error('Failed to fetch dashboard data:', error);
    } finally {
      setLoading(false);
    }
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
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Dashboard</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            Overview of your automation platform
          </p>
        </div>
        <Link href="/flows/new">
          <Button gradientDuoTone="purpleToBlue" icon={Plus}>
            Create Flow
          </Button>
        </Link>
      </div>

      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <StatsCard
          title="Total Flows"
          value={stats.totalFlows}
          icon={GitBranch}
          color="blue"
        />
        <StatsCard
          title="Total Executions"
          value={stats.totalExecutions}
          icon={Clock}
          color="green"
        />
        <StatsCard
          title="Scheduled Flows"
          value={stats.scheduledFlows}
          icon={Calendar}
          color="purple"
        />
        <StatsCard
          title="Usage Today"
          value="0"
          icon={BarChart3}
          color="yellow"
        />
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <Card>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-bold text-gray-900 dark:text-white">Recent Executions</h2>
            <Link href="/executions">
              <Button size="xs" color="light">View All</Button>
            </Link>
          </div>
          {stats.recentExecutions.length === 0 ? (
            <div className="text-center py-8 text-gray-500 dark:text-gray-400">
              <Clock className="h-12 w-12 mx-auto mb-2 opacity-50" />
              <p>No executions yet</p>
              <Link href="/flows" className="text-blue-600 hover:underline mt-2 inline-block">
                Create your first flow
              </Link>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <Table hoverable>
                <Table.Head>
                  <Table.HeadCell>Execution ID</Table.HeadCell>
                  <Table.HeadCell>Status</Table.HeadCell>
                  <Table.HeadCell>Started</Table.HeadCell>
                  <Table.HeadCell>
                    <span className="sr-only">View</span>
                  </Table.HeadCell>
                </Table.Head>
                <Table.Body className="divide-y">
                  {stats.recentExecutions.map((execution) => (
                    <Table.Row key={execution.execution_id} className="bg-white dark:border-gray-700 dark:bg-gray-800">
                      <Table.Cell className="font-mono text-sm">
                        {execution.execution_id.substring(0, 8)}...
                      </Table.Cell>
                      <Table.Cell>
                        <StatusBadge status={execution.status} />
                      </Table.Cell>
                      <Table.Cell className="text-sm text-gray-600 dark:text-gray-400">
                        {new Date(execution.started_at).toLocaleString()}
                      </Table.Cell>
                      <Table.Cell>
                        <Link href={`/executions`}>
                          <Button size="xs" color="light" icon={ArrowUpRight}>
                            View
                          </Button>
                        </Link>
                      </Table.Cell>
                    </Table.Row>
                  ))}
                </Table.Body>
              </Table>
            </div>
          )}
        </Card>

        <Card>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-bold text-gray-900 dark:text-white">Quick Actions</h2>
          </div>
          <div className="space-y-3">
            <Link href="/flows/new" className="block">
              <Button gradientDuoTone="purpleToBlue" className="w-full justify-start" icon={Plus}>
                Create New Flow
              </Button>
            </Link>
            <Link href="/templates" className="block">
              <Button color="light" className="w-full justify-start">
                Browse Templates
              </Button>
            </Link>
            <Link href="/scheduler" className="block">
              <Button color="light" className="w-full justify-start">
                Schedule a Flow
              </Button>
            </Link>
            <Link href="/metering" className="block">
              <Button color="light" className="w-full justify-start">
                View Usage Stats
              </Button>
            </Link>
          </div>
        </Card>
      </div>
    </div>
  );
}
