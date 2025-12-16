import { useState, useEffect } from 'react';
import { api, User, ApiKey } from '../api/client';
import { ConfirmModal } from '../components/ui';
import { X, Plus, Copy } from '../components/ui/icons';

export default function Settings() {
  const [user, setUser] = useState<User | null>(null);
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [loading, setLoading] = useState(true);
  const [newKeyName, setNewKeyName] = useState('');
  const [creatingKey, setCreatingKey] = useState(false);
  const [newKey, setNewKey] = useState<string | null>(null);
  const [deleteKeyId, setDeleteKeyId] = useState<string | null>(null);
  const [copiedKey, setCopiedKey] = useState<string | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      try {
        const keysData = await api.authListApiKeys();
        setApiKeys(keysData.keys);
      } catch (error) {
        console.error('Failed to load API keys:', error);
      }
      // Try to get user info, but don't fail if endpoint doesn't exist
      try {
        const userData = await api.authGetMe();
        setUser(userData);
      } catch (error) {
        // /me endpoint might not exist yet, that's okay
        console.warn('User info endpoint not available:', error);
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateApiKey = async () => {
    if (!newKeyName.trim()) {
      alert('Please enter a name for the API key');
      return;
    }
    setCreatingKey(true);
    try {
      const response = await api.authCreateApiKey({ name: newKeyName.trim() });
      setNewKey(response.key);
      setNewKeyName('');
      await loadData();
    } catch (error) {
      console.error('Failed to create API key:', error);
      alert('Failed to create API key. Please try again.');
    } finally {
      setCreatingKey(false);
    }
  };

  const handleDeleteApiKey = async () => {
    if (!deleteKeyId) return;
    try {
      await api.authDeleteApiKey(deleteKeyId);
      setDeleteKeyId(null);
      await loadData();
    } catch (error) {
      console.error('Failed to delete API key:', error);
      alert('Failed to delete API key. Please try again.');
    }
  };

  const handleCopyKey = (key: string) => {
    navigator.clipboard.writeText(key);
    setCopiedKey(key);
    setTimeout(() => setCopiedKey(null), 2000);
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">Loading settings...</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Settings</h1>
        <p className="text-gray-600 mt-1">Manage your account and preferences</p>
      </div>

      {/* User Profile Section */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-4">User Profile</h2>
        <div className="space-y-4">
          {user ? (
            <>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Email</label>
                <input
                  type="email"
                  value={user.email}
                  disabled
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg bg-gray-50 text-gray-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">User ID</label>
                <input
                  type="text"
                  value={user.id}
                  disabled
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg bg-gray-50 text-gray-500 font-mono text-sm"
                />
              </div>
            </>
          ) : (
            <div className="text-sm text-gray-500">
              User information will be displayed here once available.
            </div>
          )}
        </div>
      </div>

      {/* API Keys Management */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold text-gray-900">API Keys</h2>
          <button
            onClick={() => {
              setNewKey(null);
              setNewKeyName('');
            }}
            className="flex items-center px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700"
          >
            <Plus size={16} className="mr-2" />
            Create API Key
          </button>
        </div>

        {/* New Key Display */}
        {newKey && (
          <div className="mb-6 p-4 bg-green-50 border border-green-200 rounded-lg">
            <div className="flex items-center justify-between mb-2">
              <h3 className="text-sm font-semibold text-green-900">New API Key Created</h3>
              <button
                onClick={() => setNewKey(null)}
                className="text-green-600 hover:text-green-800"
              >
                <X size={16} />
              </button>
            </div>
            <p className="text-xs text-green-700 mb-2">
              Make sure to copy this key now. You won't be able to see it again!
            </p>
            <div className="flex items-center gap-2">
              <code className="flex-1 px-3 py-2 bg-white border border-green-300 rounded text-sm font-mono break-all">
                {newKey}
              </code>
              <button
                onClick={() => handleCopyKey(newKey)}
                className="flex items-center px-3 py-2 text-sm font-medium text-green-700 bg-green-100 rounded hover:bg-green-200"
              >
                <Copy size={16} className="mr-1" />
                {copiedKey === newKey ? 'Copied!' : 'Copy'}
              </button>
            </div>
          </div>
        )}

        {/* Create New Key Form */}
        {!newKey && (
          <div className="mb-6 p-4 bg-gray-50 rounded-lg">
            <label className="block text-sm font-medium text-gray-700 mb-2">Key Name</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={newKeyName}
                onChange={(e) => setNewKeyName(e.target.value)}
                placeholder="e.g., Production API Key"
                className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
              />
              <button
                onClick={handleCreateApiKey}
                disabled={creatingKey || !newKeyName.trim()}
                className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700 disabled:opacity-50"
              >
                {creatingKey ? 'Creating...' : 'Create'}
              </button>
            </div>
          </div>
        )}

        {/* API Keys List */}
        {apiKeys.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            <p>No API keys created yet</p>
          </div>
        ) : (
          <div className="space-y-3">
            {apiKeys.map((key) => (
              <div
                key={key.id}
                className="flex items-center justify-between p-4 bg-gray-50 rounded-lg border border-gray-200"
              >
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="text-sm font-medium text-gray-900">
                      {key.name || 'Unnamed Key'}
                    </span>
                    <span className="text-xs text-gray-500 font-mono">({key.id.slice(0, 8)}...)</span>
                  </div>
                  <div className="text-xs text-gray-500">
                    Created: {new Date(key.created_at).toLocaleDateString()}
                    {key.last_used_at && (
                      <span className="ml-4">
                        Last used: {new Date(key.last_used_at).toLocaleDateString()}
                      </span>
                    )}
                  </div>
                </div>
                <button
                  onClick={() => setDeleteKeyId(key.id)}
                  className="px-3 py-1.5 text-sm font-medium text-red-600 bg-red-50 rounded-lg hover:bg-red-100"
                >
                  Delete
                </button>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Preferences Section */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-4">Preferences</h2>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Default Flow Settings
            </label>
            <p className="text-sm text-gray-500">
              Flow preferences will be available in a future update.
            </p>
          </div>
        </div>
      </div>

      {/* Danger Zone */}
      <div className="bg-white rounded-lg shadow-sm border border-red-200 p-6">
        <h2 className="text-xl font-semibold text-red-900 mb-4">Danger Zone</h2>
        <div className="space-y-4">
          <div>
            <h3 className="text-sm font-medium text-gray-900 mb-2">Export Data</h3>
            <p className="text-sm text-gray-600 mb-3">
              Download all your flows, executions, and settings as JSON.
            </p>
            <button
              disabled
              className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Export Data (Coming Soon)
            </button>
          </div>
          <div className="pt-4 border-t border-gray-200">
            <h3 className="text-sm font-medium text-red-900 mb-2">Delete Account</h3>
            <p className="text-sm text-gray-600 mb-3">
              Permanently delete your account and all associated data. This action cannot be undone.
            </p>
            <button
              disabled
              className="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Delete Account (Coming Soon)
            </button>
          </div>
        </div>
      </div>

      <ConfirmModal
        show={deleteKeyId !== null}
        title="Delete API Key"
        message="Are you sure you want to delete this API key? This action cannot be undone and any applications using this key will stop working."
        onConfirm={handleDeleteApiKey}
        onCancel={() => setDeleteKeyId(null)}
        confirmText="Delete"
      />
    </div>
  );
}
