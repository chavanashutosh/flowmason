import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { api, Execution, ExecutionData } from '../api/client';
import { StatusBadge, Status } from '../components/ui';
import { ChevronDown, ChevronRight, Clock, Download, X } from '../components/ui/icons';

export default function Executions() {
  const [executions, setExecutions] = useState<Execution[]>([]);
  const [loading, setLoading] = useState(true);
  const [expandedExecution, setExpandedExecution] = useState<string | null>(null);
  const [executionData, setExecutionData] = useState<ExecutionData[]>([]);
  const [loadingData, setLoadingData] = useState(false);
  const [dataFilter, setDataFilter] = useState<'all' | 'fetched' | 'intermediate'>('all');

  useEffect(() => {
    fetchExecutions();
  }, []);

  const fetchExecutions = async () => {
    setLoading(true);
    try {
      const data = await api.executionsList();
      setExecutions(data);
    } catch (error) {
      console.error('Failed to fetch executions:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleExpandExecution = async (executionId: string) => {
    if (expandedExecution === executionId) {
      setExpandedExecution(null);
      setExecutionData([]);
      return;
    }

    setExpandedExecution(executionId);
    setLoadingData(true);
    try {
      let data: ExecutionData[];
      switch (dataFilter) {
        case 'fetched':
          data = await api.executionsGetFetchedData(executionId);
          break;
        case 'intermediate':
          data = await api.executionsGetIntermediateData(executionId);
          break;
        default:
          data = await api.executionsGetData(executionId);
      }
      setExecutionData(data);
    } catch (error) {
      console.error('Failed to fetch execution data:', error);
      setExecutionData([]);
    } finally {
      setLoadingData(false);
    }
  };

  const handleFilterChange = async (filter: 'all' | 'fetched' | 'intermediate') => {
    setDataFilter(filter);
    if (expandedExecution) {
      setLoadingData(true);
      try {
        let data: ExecutionData[];
        switch (filter) {
          case 'fetched':
            data = await api.executionsGetFetchedData(expandedExecution);
            break;
          case 'intermediate':
            data = await api.executionsGetIntermediateData(expandedExecution);
            break;
          default:
            data = await api.executionsGetData(expandedExecution);
        }
        setExecutionData(data);
      } catch (error) {
        console.error('Failed to fetch execution data:', error);
        setExecutionData([]);
      } finally {
        setLoadingData(false);
      }
    }
  };

  const downloadData = (data: ExecutionData) => {
    const blob = new Blob([JSON.stringify(data.data_value, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${data.data_key}_${data.id}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const groupedData = executionData.reduce((acc, item) => {
    if (!acc[item.brick_index]) {
      acc[item.brick_index] = [];
    }
    acc[item.brick_index].push(item);
    return acc;
  }, {} as Record<number, ExecutionData[]>);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Execution History</h1>
          <p className="text-gray-600 mt-1">View and monitor flow execution history</p>
        </div>
        <button
          className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
          onClick={fetchExecutions}
        >
          Refresh
        </button>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">Loading...</div>
      ) : executions.length === 0 ? (
        <div className="bg-white rounded-lg shadow p-12 text-center">
          <div className="flex justify-center mb-6">
            <Clock size={56} className="text-gray-300" />
          </div>
          <p className="text-gray-500">No executions yet</p>
        </div>
      ) : (
        <div className="space-y-4">
          {executions.map((execution) => (
            <div key={execution.execution_id} className="bg-white rounded-lg shadow overflow-hidden">
              <div
                className="hover:bg-gray-50 cursor-pointer"
                onClick={() => handleExpandExecution(execution.execution_id)}
              >
                <div className="px-6 py-4 flex items-center justify-between">
                  <div className="flex items-center gap-4 flex-1">
                    <div className="flex items-center gap-2">
                      {expandedExecution === execution.execution_id ? (
                        <ChevronDown size={20} className="text-gray-400" />
                      ) : (
                        <ChevronRight size={20} className="text-gray-400" />
                      )}
                    </div>
                    <div className="flex-1 grid grid-cols-4 gap-4">
                      <div>
                        <div className="text-xs text-gray-500 mb-1">Execution ID</div>
                        <div className="text-sm font-mono text-gray-900">
                          {execution.execution_id.substring(0, 8)}...
                        </div>
                      </div>
                      <div>
                        <div className="text-xs text-gray-500 mb-1">Flow ID</div>
                        <Link
                          to={`/flows/${execution.flow_id}`}
                          className="text-sm text-primary-600 hover:text-primary-900 hover:underline"
                          onClick={(e) => e.stopPropagation()}
                        >
                          {execution.flow_id.substring(0, 8)}...
                        </Link>
                      </div>
                      <div>
                        <div className="text-xs text-gray-500 mb-1">Status</div>
                        <StatusBadge
                          status={
                            execution.status === 'completed'
                              ? Status.Completed
                              : execution.status === 'failed'
                              ? Status.Failed
                              : execution.status === 'running'
                              ? Status.Running
                              : Status.Pending
                          }
                        />
                      </div>
                      <div>
                        <div className="text-xs text-gray-500 mb-1">Started</div>
                        <div className="text-sm text-gray-500">{execution.created_at}</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {expandedExecution === execution.execution_id && (
                <div className="border-t border-gray-200 p-6 bg-gray-50">
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-semibold text-gray-900">Execution Data</h3>
                    <div className="flex items-center gap-2">
                      <select
                        value={dataFilter}
                        onChange={(e) =>
                          handleFilterChange(e.target.value as 'all' | 'fetched' | 'intermediate')
                        }
                        className="px-3 py-1.5 text-sm border border-gray-300 rounded-lg bg-white"
                      >
                        <option value="all">All Data</option>
                        <option value="fetched">Fetched Only</option>
                        <option value="intermediate">Intermediate Only</option>
                      </select>
                    </div>
                  </div>

                  {loadingData ? (
                    <div className="text-center py-8 text-gray-500">Loading data...</div>
                  ) : executionData.length === 0 ? (
                    <div className="text-center py-8 text-gray-500">No execution data available</div>
                  ) : (
                    <div className="space-y-4">
                      {Object.entries(groupedData)
                        .sort(([a], [b]) => Number(a) - Number(b))
                        .map(([brickIndex, items]) => (
                          <div key={brickIndex} className="bg-white rounded-lg border border-gray-200 p-4">
                            <div className="flex items-center justify-between mb-3">
                              <h4 className="font-semibold text-gray-900">
                                Brick {brickIndex}: {items[0]?.brick_type || 'Unknown'}
                              </h4>
                              <span className="text-xs text-gray-500">
                                {items.length} data point{items.length !== 1 ? 's' : ''}
                              </span>
                            </div>
                            <div className="space-y-3">
                              {items.map((item) => (
                                <div
                                  key={item.id}
                                  className="bg-gray-50 rounded-lg p-3 border border-gray-200"
                                >
                                  <div className="flex items-center justify-between mb-2">
                                    <div className="flex items-center gap-2">
                                      <span className="text-xs font-medium text-gray-700 px-2 py-1 bg-white rounded border border-gray-300">
                                        {item.data_type}
                                      </span>
                                      <span className="text-xs text-gray-600 font-mono">
                                        {item.data_key}
                                      </span>
                                    </div>
                                    <button
                                      onClick={() => downloadData(item)}
                                      className="flex items-center gap-1 px-2 py-1 text-xs text-primary-600 hover:text-primary-700 hover:bg-primary-50 rounded"
                                    >
                                      <Download size={14} />
                                      Download
                                    </button>
                                  </div>
                                  <div className="mt-2">
                                    <details className="cursor-pointer">
                                      <summary className="text-xs text-gray-600 hover:text-gray-900">
                                        View JSON
                                      </summary>
                                      <pre className="mt-2 p-3 bg-white rounded border border-gray-300 text-xs font-mono overflow-auto max-h-64">
                                        {JSON.stringify(item.data_value, null, 2)}
                                      </pre>
                                    </details>
                                  </div>
                                  <div className="text-xs text-gray-500 mt-2">
                                    {new Date(item.timestamp).toLocaleString()}
                                  </div>
                                </div>
                              ))}
                            </div>
                          </div>
                        ))}
                    </div>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
