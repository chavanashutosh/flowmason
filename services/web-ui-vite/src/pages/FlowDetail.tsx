import { useEffect, useState } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { api, Flow } from '../api/client';
import { StatusBadge, Status, ConfirmModal } from '../components/ui';
import { Play, Edit, ArrowLeft } from '../components/ui/icons';

export default function FlowDetail() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [flow, setFlow] = useState<Flow | null>(null);
  const [loading, setLoading] = useState(true);
  const [deleteModalOpen, setDeleteModalOpen] = useState(false);
  const [running, setRunning] = useState(false);

  useEffect(() => {
    if (id) {
      fetchFlow();
    }
  }, [id]);

  const fetchFlow = async () => {
    if (!id) return;
    try {
      const data = await api.flowsGet(id);
      setFlow(data);
    } catch (error) {
      console.error('Failed to fetch flow:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!id) return;
    try {
      await api.flowsDelete(id);
      navigate('/flows');
    } catch (error) {
      console.error('Failed to delete flow:', error);
    }
  };

  const handleRunFlow = async () => {
    if (!id) return;
    setRunning(true);
    try {
      await api.executionsExecute(id, {});
      navigate('/executions');
    } catch (error) {
      console.error('Failed to run flow:', error);
    } finally {
      setRunning(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link
            to="/flows"
            className="flex items-center px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
          >
            <ArrowLeft size={16} className="mr-2" />
            <span>Back</span>
          </Link>
          <div>
            <h1 className="text-3xl font-bold text-gray-900">{flow?.name || 'Flow Detail'}</h1>
          </div>
        </div>
        {flow && (
          <div className="flex items-center gap-2">
            <button
              className="flex items-center px-4 py-2 text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 rounded-lg transition-colors"
              disabled={running}
              onClick={handleRunFlow}
            >
              <Play size={16} className="mr-2" />
              {running ? 'Running...' : 'Run Flow'}
            </button>
            <Link
              to={`/flows/${id}/edit`}
              className="flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              <Edit size={16} className="mr-2" />
              <span>Edit</span>
            </Link>
            <button
              className="flex items-center px-4 py-2 text-sm font-medium text-red-600 bg-white border border-red-300 rounded-lg hover:bg-red-50 transition-colors"
              onClick={() => setDeleteModalOpen(true)}
            >
              Delete
            </button>
          </div>
        )}
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">Loading...</div>
      ) : flow ? (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="text-sm font-medium text-gray-500 mb-2">Description</h3>
              {flow.description ? (
                <p className="text-gray-900">{flow.description}</p>
              ) : (
                <p className="text-gray-400 italic">No description</p>
              )}
            </div>
            <div>
              <h3 className="text-sm font-medium text-gray-500 mb-2">Status</h3>
              <StatusBadge status={flow.active ? Status.Active : Status.Inactive} />
            </div>
            <div>
              <h3 className="text-sm font-medium text-gray-500 mb-2">Flow ID</h3>
              <p className="text-sm font-mono text-gray-900">{id}</p>
            </div>
            <div>
              <h3 className="text-sm font-medium text-gray-500 mb-2">Created</h3>
              <p className="text-sm text-gray-900">{flow.created_at}</p>
            </div>
          </div>
          <div className="mt-6 pt-6 border-t border-gray-200">
            <h3 className="text-sm font-medium text-gray-500 mb-4">Bricks</h3>
            {flow.bricks.length === 0 ? (
              <p className="text-gray-400 italic">No bricks configured</p>
            ) : (
              <div className="space-y-2">
                {flow.bricks.map((brick, idx) => (
                  <div key={idx} className="flex items-center justify-between p-3 bg-gray-50 rounded">
                    <div>
                      <span className="text-sm font-medium text-gray-900">{brick.brick_type}</span>
                    </div>
                    <span className="text-xs text-gray-500">Brick {idx + 1}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      ) : (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <p className="text-gray-500">Flow not found</p>
          <Link
            to="/flows"
            className="inline-block mt-4 px-4 py-2 text-sm font-medium text-primary-600 hover:text-primary-700"
          >
            Back to Flows
          </Link>
        </div>
      )}

      <ConfirmModal
        show={deleteModalOpen}
        title="Delete Flow"
        message="Are you sure you want to delete this flow? This action cannot be undone."
        onConfirm={handleDelete}
        onCancel={() => setDeleteModalOpen(false)}
        confirmText="Delete"
      />
    </div>
  );
}
