import { useEffect, useState } from 'react';
import { api, ScheduledFlow, Flow } from '../api/client';

export default function Scheduler() {
  const [scheduledFlows, setScheduledFlows] = useState<ScheduledFlow[]>([]);
  const [flows, setFlows] = useState<Flow[]>([]);
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
        api.schedulerListScheduledFlows(),
        api.flowsList(),
      ]);
      setScheduledFlows(scheduled);
      setFlows(allFlows);
    } catch (error) {
      console.error('Failed to fetch scheduler data:', error);
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
      const scheduled = await api.schedulerScheduleFlow(selectedFlowId, cronExpression);
      setScheduledFlows([...scheduledFlows, scheduled]);
      setModalOpen(false);
      setSelectedFlowId('');
      setCronExpression('');
    } catch (err: any) {
      setError(`Failed to schedule flow: ${err.message || 'Unknown error'}`);
    } finally {
      setSaving(false);
    }
  };

  const handleUnschedule = async (flowId: string) => {
    try {
      await api.schedulerUnscheduleFlow(flowId);
      setScheduledFlows(scheduledFlows.filter((s) => s.flow_id !== flowId));
    } catch (error) {
      console.error('Failed to unschedule flow:', error);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Scheduler</h1>
          <p className="text-gray-600 mt-1">Manage scheduled flows</p>
        </div>
        <button
          className="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg"
          onClick={() => setModalOpen(true)}
        >
          Schedule Flow
        </button>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">Loading...</div>
      ) : scheduledFlows.length === 0 ? (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <span className="text-6xl mb-6 block">📅</span>
          <p className="text-gray-500">No scheduled flows</p>
        </div>
      ) : (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Flow ID</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Cron Expression
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {scheduledFlows.map((scheduled) => (
                <tr key={scheduled.flow_id}>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {scheduled.flow_id}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 font-mono">
                    {scheduled.cron_expression}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <button
                      className="text-red-600 hover:text-red-900"
                      onClick={() => handleUnschedule(scheduled.flow_id)}
                    >
                      Unschedule
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {modalOpen && (
        <div
          className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
          onClick={() => setModalOpen(false)}
        >
          <div className="bg-white rounded-lg p-6 max-w-md w-full" onClick={(e) => e.stopPropagation()}>
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Schedule Flow</h3>
            {error && <div className="mb-4 p-3 bg-red-50 text-red-800 rounded">{error}</div>}
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Flow</label>
                <select
                  className="w-full px-3 py-2 border border-gray-300 rounded"
                  value={selectedFlowId}
                  onChange={(e) => setSelectedFlowId(e.target.value)}
                >
                  <option value="">Select a flow...</option>
                  {flows.map((flow) => (
                    <option key={flow.id} value={flow.id}>
                      {flow.name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Cron Expression</label>
                <input
                  type="text"
                  className="w-full px-3 py-2 border border-gray-300 rounded font-mono"
                  placeholder="0 * * * *"
                  value={cronExpression}
                  onChange={(e) => setCronExpression(e.target.value)}
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end gap-3">
              <button
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
                onClick={() => setModalOpen(false)}
              >
                Cancel
              </button>
              <button
                className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700 disabled:opacity-50"
                onClick={handleSchedule}
                disabled={saving}
              >
                {saving ? 'Saving...' : 'Schedule'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
