'use client';

import { useEffect, useState } from 'react';
import { Card, Table, Button, Modal, Badge, Spinner, Label } from 'flowbite-react';
import { Clock, CheckCircle, XCircle, Loader, Eye, RefreshCw } from 'lucide-react';
import { api } from '@/lib/api';
import { StatusBadge } from '@/components/ui/StatusBadge';

interface Execution {
  execution_id: string;
  flow_id: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  started_at: string;
  completed_at?: string;
  input_payload: any;
  output_payload?: any;
  error?: string;
}

export default function ExecutionsPage() {
  const [executions, setExecutions] = useState<Execution[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedExecution, setSelectedExecution] = useState<Execution | null>(null);
  const [detailModalOpen, setDetailModalOpen] = useState(false);

  useEffect(() => {
    fetchExecutions();
  }, []);

  const fetchExecutions = async () => {
    try {
      const data = await api.executions.list();
      setExecutions(data);
    } catch (error) {
      console.error('Failed to fetch executions:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  const getDuration = (started: string, completed?: string) => {
    if (!completed) return 'Running...';
    const start = new Date(started).getTime();
    const end = new Date(completed).getTime();
    const ms = end - start;
    if (ms < 1000) return `${ms}ms`;
    if (ms < 60000) return `${(ms / 1000).toFixed(2)}s`;
    return `${(ms / 60000).toFixed(2)}m`;
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
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Execution History</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            View and monitor flow execution history
          </p>
        </div>
        <Button color="light" icon={RefreshCw} onClick={fetchExecutions}>
          Refresh
        </Button>
      </div>

      <Card>
        {executions.length === 0 ? (
          <div className="text-center py-12">
            <Clock className="h-12 w-12 mx-auto mb-4 text-gray-400" />
            <p className="text-gray-500 dark:text-gray-400 mb-4">No executions yet</p>
            <p className="text-sm text-gray-400">
              Create and run a flow to see execution history
            </p>
          </div>
        ) : (
          <Table hoverable>
            <Table.Head>
              <Table.HeadCell>Execution ID</Table.HeadCell>
              <Table.HeadCell>Flow ID</Table.HeadCell>
              <Table.HeadCell>Status</Table.HeadCell>
              <Table.HeadCell>Started</Table.HeadCell>
              <Table.HeadCell>Duration</Table.HeadCell>
              <Table.HeadCell>
                <span className="sr-only">Actions</span>
              </Table.HeadCell>
            </Table.Head>
            <Table.Body className="divide-y">
              {executions.map((execution) => (
                <Table.Row key={execution.execution_id} className="bg-white dark:border-gray-700 dark:bg-gray-800">
                  <Table.Cell className="font-mono text-sm">
                    {execution.execution_id.substring(0, 8)}...
                  </Table.Cell>
                  <Table.Cell className="font-mono text-sm">
                    {execution.flow_id.substring(0, 8)}...
                  </Table.Cell>
                  <Table.Cell>
                    <StatusBadge status={execution.status} />
                  </Table.Cell>
                  <Table.Cell className="text-sm text-gray-600 dark:text-gray-400">
                    {formatDate(execution.started_at)}
                  </Table.Cell>
                  <Table.Cell className="text-sm text-gray-600 dark:text-gray-400">
                    {getDuration(execution.started_at, execution.completed_at)}
                  </Table.Cell>
                  <Table.Cell>
                    <Button
                      size="xs"
                      color="light"
                      icon={Eye}
                      onClick={() => {
                        setSelectedExecution(execution);
                        setDetailModalOpen(true);
                      }}
                    >
                      View
                    </Button>
                  </Table.Cell>
                </Table.Row>
              ))}
            </Table.Body>
          </Table>
        )}
      </Card>

      <Modal show={detailModalOpen} onClose={() => setDetailModalOpen(false)} size="xl">
        <Modal.Header>Execution Details</Modal.Header>
        <Modal.Body>
          {selectedExecution && (
            <div className="space-y-4">
              <div>
                <Label className="text-sm font-medium text-gray-700 dark:text-gray-300">Execution ID</Label>
                <p className="text-sm font-mono text-gray-900 dark:text-white break-all">
                  {selectedExecution.execution_id}
                </p>
              </div>

              <div>
                <Label className="text-sm font-medium text-gray-700 dark:text-gray-300">Status</Label>
                <div className="mt-1">
                  <StatusBadge status={selectedExecution.status} />
                </div>
              </div>

              <div>
                <Label className="text-sm font-medium text-gray-700 dark:text-gray-300">Started At</Label>
                <p className="text-sm text-gray-900 dark:text-white">{formatDate(selectedExecution.started_at)}</p>
              </div>

              {selectedExecution.completed_at && (
                <div>
                  <Label className="text-sm font-medium text-gray-700 dark:text-gray-300">Completed At</Label>
                  <p className="text-sm text-gray-900 dark:text-white">{formatDate(selectedExecution.completed_at)}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    Duration: {getDuration(selectedExecution.started_at, selectedExecution.completed_at)}
                  </p>
                </div>
              )}

              <div>
                <Label className="text-sm font-medium text-gray-700 dark:text-gray-300">Input Payload</Label>
                <pre className="text-xs bg-gray-50 dark:bg-gray-800 p-3 rounded mt-1 overflow-auto max-h-40">
                  {JSON.stringify(selectedExecution.input_payload, null, 2)}
                </pre>
              </div>

              {selectedExecution.output_payload && (
                <div>
                  <Label className="text-sm font-medium text-gray-700 dark:text-gray-300">Output Payload</Label>
                  <pre className="text-xs bg-gray-50 dark:bg-gray-800 p-3 rounded mt-1 overflow-auto max-h-40">
                    {JSON.stringify(selectedExecution.output_payload, null, 2)}
                  </pre>
                </div>
              )}

              {selectedExecution.error && (
                <div>
                  <Label className="text-sm font-medium text-red-700 dark:text-red-300">Error</Label>
                  <p className="text-sm text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 p-3 rounded mt-1">
                    {selectedExecution.error}
                  </p>
                </div>
              )}
            </div>
          )}
        </Modal.Body>
        <Modal.Footer>
          <Button color="gray" onClick={() => setDetailModalOpen(false)}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
}
