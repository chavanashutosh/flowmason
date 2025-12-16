import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { api, BrickConfig } from '../api/client';
import { FlowBuilder } from '../components/flow-builder';

export default function NewFlow() {
  const navigate = useNavigate();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [bricks, setBricks] = useState<BrickConfig[]>([]);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) {
      alert('Please enter a flow name');
      return;
    }
    setLoading(true);
    try {
      const flow = await api.flowsCreate({
        name,
        description: description || null,
        bricks,
      });
      navigate(`/flows/${flow.id}`);
    } catch (error) {
      console.error('Failed to create flow:', error);
      alert('Failed to create flow. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Create New Flow</h1>
        <p className="text-gray-600 mt-1">Build a new automation flow</p>
      </div>

      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-2">
              Flow Name
            </label>
            <input
              type="text"
              id="name"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <div>
            <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-2">
              Description
            </label>
            <textarea
              id="description"
              rows={3}
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
        </form>
      </div>

      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Flow Builder</h2>
        <p className="text-sm text-gray-600 mb-4">
          Drag bricks from the palette to build your flow, or click to add them. Configure each
          brick by expanding its configuration panel.
        </p>
        <div className="h-[600px]">
          <FlowBuilder bricks={bricks} onBricksChange={setBricks} />
        </div>
      </div>

      <div className="flex justify-end space-x-3">
        <button
          type="button"
          onClick={() => navigate('/flows')}
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
        >
          Cancel
        </button>
        <button
          type="button"
          onClick={handleSubmit}
          disabled={loading || !name.trim()}
          className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700 disabled:opacity-50"
        >
          {loading ? 'Creating...' : 'Create Flow'}
        </button>
      </div>
    </div>
  );
}
