import { useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { api, Flow } from '../api/client';
import { StatusBadge, Status, ConfirmModal } from '../components/ui';
import { Play, Edit } from '../components/ui/icons';

export default function Flows() {
  const [flows, setFlows] = useState<Flow[]>([]);
  const [loading, setLoading] = useState(true);
  const [deleteModalOpen, setDeleteModalOpen] = useState(false);
  const [selectedFlowId, setSelectedFlowId] = useState<string>('');
  const navigate = useNavigate();

  useEffect(() => {
    fetchFlows();
  }, []);

  const fetchFlows = async () => {
    try {
      const data = await api.flowsList();
      setFlows(data);
    } catch (error) {
      console.error('Failed to fetch flows:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRunFlow = async (flowId: string) => {
    try {
      await api.executionsExecute(flowId, {});
      fetchFlows(); // Refresh flows list
    } catch (error) {
      console.error('Failed to run flow:', error);
    }
  };

  const handleDeleteClick = (flowId: string) => {
    setSelectedFlowId(flowId);
    setDeleteModalOpen(true);
  };

  const handleDeleteConfirm = async () => {
    try {
      await api.flowsDelete(selectedFlowId);
      setFlows(flows.filter((f) => f.id !== selectedFlowId));
      setDeleteModalOpen(false);
    } catch (error) {
      console.error('Failed to delete flow:', error);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Flows</h1>
          <p className="text-gray-600 mt-1">Manage your automation flows</p>
        </div>
        <Link to="/flows/new">
          <button className="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg">
            Create Flow
          </button>
        </Link>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">Loading...</div>
      ) : flows.length === 0 ? (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <p className="text-gray-600 font-medium mb-2">No flows created yet</p>
          <p className="text-sm text-gray-500 mb-6">Get started by creating a new flow</p>
          <Link to="/flows/new">
            <button className="px-4 py-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg">
              Create New Flow
            </button>
          </Link>
        </div>
      ) : (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Created</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {flows.map((flow) => (
                <tr key={flow.id}>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div>
                      <div className="text-sm font-medium text-gray-900">{flow.name}</div>
                      {flow.description && (
                        <div className="text-sm text-gray-500">{flow.description}</div>
                      )}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <StatusBadge status={flow.active ? Status.Active : Status.Inactive} />
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{flow.created_at}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                    <div className="flex gap-2">
                      <button
                        className="flex items-center px-3 py-1.5 text-sm text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded transition-colors"
                        onClick={() => handleRunFlow(flow.id)}
                      >
                        <Play size={14} className="mr-1" />
                        <span>Run</span>
                      </button>
                      <Link
                        to={`/flows/${flow.id}`}
                        className="flex items-center px-3 py-1.5 text-sm text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded transition-colors"
                      >
                        <Edit size={14} className="mr-1" />
                        <span>Edit</span>
                      </Link>
                      <button
                        className="flex items-center px-3 py-1.5 text-sm text-red-600 hover:text-red-900 hover:bg-red-50 rounded transition-colors"
                        onClick={() => handleDeleteClick(flow.id)}
                      >
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <ConfirmModal
        show={deleteModalOpen}
        title="Delete Flow"
        message="Are you sure you want to delete this flow? This action cannot be undone."
        onConfirm={handleDeleteConfirm}
        onCancel={() => setDeleteModalOpen(false)}
        confirmText="Delete"
      />
    </div>
  );
}
