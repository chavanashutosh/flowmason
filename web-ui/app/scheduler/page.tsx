'use client';

import { useEffect, useState } from 'react';
import { Card, Table, Button, Modal, Label, TextInput, Alert, Spinner, Badge, Select } from 'flowbite-react';
import { Calendar, Plus, Trash2, Clock } from 'lucide-react';
import { api } from '@/lib/api';

interface ScheduledFlow {
  flow_id: string;
  cron_expression: string;
}

export default function SchedulerPage() {
  const [scheduledFlows, setScheduledFlows] = useState<any[]>([]);
  const [flows, setFlows] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [modalOpen, setModalOpen] = useState(false);
  const [selectedFlowId, setSelectedFlowId] = useState('');
  const [cronExpression, setCronExpression] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    try {
      const [scheduled, allFlows] = await Promise.all([
        api.scheduler.listScheduledFlows().catch(() => ({ flows: [] })),
        api.flows.list().catch(() => []),
      ]);
      setScheduledFlows(scheduled.flows || []);
      setFlows(allFlows);
    } catch (error) {
      console.error('Failed to fetch data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSchedule = async () => {
    if (!selectedFlowId || !cronExpression.trim()) {
      setError('Please select a flow and enter a cron expression');
      return;
    }

    setSaving(true);
    setError(null);

    try {
      await api.scheduler.scheduleFlow(selectedFlowId, cronExpression);
      setModalOpen(false);
      setSelectedFlowId('');
      setCronExpression('');
      fetchData();
    } catch (err: any) {
      setError(err.message || 'Failed to schedule flow');
    } finally {
      setSaving(false);
    }
  };

  const handleUnschedule = async (flowId: string) => {
    try {
      await api.scheduler.unscheduleFlow(flowId);
      fetchData();
    } catch (error) {
      console.error('Failed to unschedule flow:', error);
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
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Scheduler</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            Schedule flows to run automatically using cron expressions
          </p>
        </div>
        <Button gradientDuoTone="purpleToBlue" icon={Plus} onClick={() => setModalOpen(true)}>
          Schedule Flow
        </Button>
      </div>

      <Card>
        {scheduledFlows.length === 0 ? (
          <div className="text-center py-12">
            <Calendar className="h-12 w-12 mx-auto mb-4 text-gray-400" />
            <p className="text-gray-500 dark:text-gray-400 mb-4">No scheduled flows</p>
            <Button gradientDuoTone="purpleToBlue" icon={Plus} onClick={() => setModalOpen(true)}>
              Schedule Your First Flow
            </Button>
          </div>
        ) : (
          <Table hoverable>
            <Table.Head>
              <Table.HeadCell>Flow</Table.HeadCell>
              <Table.HeadCell>Cron Expression</Table.HeadCell>
              <Table.HeadCell>Status</Table.HeadCell>
              <Table.HeadCell>
                <span className="sr-only">Actions</span>
              </Table.HeadCell>
            </Table.Head>
            <Table.Body className="divide-y">
              {scheduledFlows.map((scheduled) => {
                const flow = flows.find(f => f.id === scheduled.flow_id);
                return (
                  <Table.Row key={scheduled.flow_id} className="bg-white dark:border-gray-700 dark:bg-gray-800">
                    <Table.Cell className="font-medium text-gray-900 dark:text-white">
                      {flow?.name || scheduled.flow_id.substring(0, 8)}
                    </Table.Cell>
                    <Table.Cell className="font-mono text-sm">
                      {scheduled.cron_expression || 'N/A'}
                    </Table.Cell>
                    <Table.Cell>
                      <Badge color="success">Active</Badge>
                    </Table.Cell>
                    <Table.Cell>
                      <Button
                        size="xs"
                        color="failure"
                        icon={Trash2}
                        onClick={() => handleUnschedule(scheduled.flow_id)}
                      >
                        Unschedule
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                );
              })}
            </Table.Body>
          </Table>
        )}
      </Card>

      <Modal show={modalOpen} onClose={() => setModalOpen(false)}>
        <Modal.Header>Schedule Flow</Modal.Header>
        <Modal.Body>
          <div className="space-y-4">
            {error && (
              <Alert color="failure" onDismiss={() => setError(null)}>
                {error}
              </Alert>
            )}

            <div>
              <Label htmlFor="flow" value="Select Flow" />
              <Select
                id="flow"
                value={selectedFlowId}
                onChange={(e) => setSelectedFlowId(e.target.value)}
                required
              >
                <option value="">Choose a flow</option>
                {flows.map((flow) => (
                  <option key={flow.id} value={flow.id}>
                    {flow.name}
                  </option>
                ))}
              </Select>
            </div>

            <div>
              <Label htmlFor="cron" value="Cron Expression" />
              <TextInput
                id="cron"
                type="text"
                value={cronExpression}
                onChange={(e) => setCronExpression(e.target.value)}
                placeholder="0 0 * * * (daily at midnight)"
                helperText="Example: 0 0 * * * runs daily at midnight"
              />
            </div>

            <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
              <p className="text-sm font-medium text-blue-900 dark:text-blue-300 mb-2">Cron Examples:</p>
              <ul className="text-xs text-blue-800 dark:text-blue-400 space-y-1">
                <li><code>0 0 * * *</code> - Daily at midnight</li>
                <li><code>0 */6 * * *</code> - Every 6 hours</li>
                <li><code>0 9 * * 1-5</code> - Weekdays at 9 AM</li>
                <li><code>*/15 * * * *</code> - Every 15 minutes</li>
              </ul>
            </div>
          </div>
        </Modal.Body>
        <Modal.Footer>
          <Button color="gray" onClick={() => setModalOpen(false)}>
            Cancel
          </Button>
          <Button
            gradientDuoTone="purpleToBlue"
            onClick={handleSchedule}
            disabled={saving || !selectedFlowId || !cronExpression.trim()}
          >
            {saving ? 'Scheduling...' : 'Schedule'}
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
}
