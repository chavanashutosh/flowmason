'use client';

import { useEffect, useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import ReactFlow, {
  Node,
  Edge,
  Background,
  Controls,
  addEdge,
  Connection,
  useNodesState,
  useEdgesState,
} from 'react-flow-renderer';
import 'react-flow-renderer/dist/style.css';
import { Card, Button, Modal, Label, TextInput, Textarea, Select, Alert, Spinner, Badge } from 'flowbite-react';
import { Plus, Trash2, Settings, Save, CheckCircle, XCircle, Loader, Clock, Play } from 'lucide-react';
import { api } from '@/lib/api';
import Link from 'next/link';
import { StatusBadge } from '@/components/ui/StatusBadge';

interface Flow {
  id: string;
  name: string;
  description?: string;
  bricks: Array<{
    brick_type: string;
    config: any;
  }>;
}

interface BrickSchema {
  brick_type: string;
  name: string;
  config_schema: any;
}

export default function FlowEditorPage() {
  const params = useParams();
  const router = useRouter();
  const [flow, setFlow] = useState<Flow | null>(null);
  const [nodes, setNodes] = useNodesState([]);
  const [edges, setEdges] = useEdgesState([]);
  const [loading, setLoading] = useState(true);
  const [availableBricks, setAvailableBricks] = useState<BrickSchema[]>([]);
  const [showAddBrick, setShowAddBrick] = useState(false);
  const [editingBrick, setEditingBrick] = useState<number | null>(null);
  const [saving, setSaving] = useState(false);
  const [flowExecutions, setFlowExecutions] = useState<any[]>([]);
  const [showExecutions, setShowExecutions] = useState(false);
  const [executionResult, setExecutionResult] = useState<any>(null);
  const [executing, setExecuting] = useState(false);
  const [configModalOpen, setConfigModalOpen] = useState(false);

  useEffect(() => {
    if (params.id) {
      fetchFlow(params.id as string);
      fetchAvailableBricks();
      fetchFlowExecutions(params.id as string);
    }
  }, [params.id]);

  const fetchAvailableBricks = async () => {
    try {
      const response = await api.bricks.list();
      setAvailableBricks(response.bricks);
    } catch (error) {
      console.error('Failed to fetch bricks:', error);
    }
  };

  const fetchFlow = async (id: string) => {
    try {
      const data = await api.flows.get(id);
      setFlow(data);

      const flowNodes: Node[] = data.bricks.map(
        (brick: any, index: number) => ({
          id: `brick-${index}`,
          type: 'default',
          position: { x: index * 200, y: 100 },
          data: {
            label: brick.brick_type.replace(/_/g, ' '),
            brick: brick,
            index: index,
          },
        })
      );

      const flowEdges: Edge[] = [];
      for (let i = 0; i < flowNodes.length - 1; i++) {
        flowEdges.push({
          id: `edge-${i}`,
          source: flowNodes[i].id,
          target: flowNodes[i + 1].id,
        });
      }

      setNodes(flowNodes);
      setEdges(flowEdges);
    } catch (error) {
      console.error('Failed to fetch flow:', error);
    } finally {
      setLoading(false);
    }
  };

  const fetchFlowExecutions = async (flowId: string) => {
    try {
      const executions = await api.executions.listByFlow(flowId);
      setFlowExecutions(executions.slice(0, 5));
    } catch (error) {
      console.error('Failed to fetch flow executions:', error);
    }
  };

  const onConnect = (params: Connection) => {
    setEdges((eds) => addEdge(params, eds));
  };

  const addBrick = (brickName: string) => {
    const brick = availableBricks.find(b => b.name === brickName);
    if (!brick || !flow) return;

    const newBrick = {
      brick_type: brickName,
      config: {},
    };

    const updatedFlow = {
      ...flow,
      bricks: [...flow.bricks, newBrick],
    };

    const newNode: Node = {
      id: `brick-${flow.bricks.length}`,
      type: 'default',
      position: { x: flow.bricks.length * 200, y: 100 },
      data: {
        label: brickName.replace(/_/g, ' '),
        brick: newBrick,
        index: flow.bricks.length,
      },
    };

    setNodes([...nodes, newNode]);
    setFlow(updatedFlow);
    setShowAddBrick(false);
  };

  const removeBrick = (index: number) => {
    if (!flow) return;
    const updatedBricks = flow.bricks.filter((_, i) => i !== index);
    const updatedFlow = { ...flow, bricks: updatedBricks };
    setFlow(updatedFlow);

    const updatedNodes = nodes.filter((_, i) => i !== index).map((node, i) => ({
      ...node,
      id: `brick-${i}`,
      position: { x: i * 200, y: 100 },
      data: { ...node.data, index: i },
    }));

    const updatedEdges = updatedNodes.length > 1
      ? updatedNodes.slice(0, -1).map((_, i) => ({
          id: `edge-${i}`,
          source: updatedNodes[i].id,
          target: updatedNodes[i + 1].id,
        }))
      : [];

    setNodes(updatedNodes);
    setEdges(updatedEdges);
  };

  const handleSave = async () => {
    if (!flow) return;
    setSaving(true);
    try {
      await api.flows.update(flow.id, {
        name: flow.name,
        description: flow.description,
        bricks: flow.bricks.map(b => ({
          brick_type: b.brick_type as any,
          config: b.config,
        })),
      });
      alert('Flow saved successfully!');
    } catch (error) {
      console.error('Failed to save flow:', error);
      alert('Failed to save flow');
    } finally {
      setSaving(false);
    }
  };

  const handleRunFlow = async () => {
    if (!flow) return;
    setExecuting(true);
    setExecutionResult(null);
    try {
      const result = await api.executions.execute(flow.id, {});
      setExecutionResult(result);
      await fetchFlowExecutions(flow.id);
    } catch (error: any) {
      console.error('Failed to run flow:', error);
      setExecutionResult({
        status: 'failed',
        error: error.message || 'Failed to run flow',
      });
    } finally {
      setExecuting(false);
    }
  };

  const renderBrickConfig = (index: number) => {
    const brick = flow?.bricks[index];
    if (!brick) return null;
    const schema = availableBricks.find(b => b.name === brick.brick_type)?.config_schema;
    if (!schema) return null;

    return (
      <Modal show={configModalOpen && editingBrick === index} onClose={() => setConfigModalOpen(false)}>
        <Modal.Header>Configure {brick.brick_type.replace(/_/g, ' ')}</Modal.Header>
        <Modal.Body>
          <div className="space-y-4">
            {schema.properties && Object.keys(schema.properties).map(key => {
              const prop = schema.properties[key];
              const value = brick.config[key];

              if (prop.type === 'string' && prop.enum) {
                return (
                  <div key={key}>
                    <Label htmlFor={key} value={prop.title || key} />
                    <Select
                      id={key}
                      value={value || ''}
                      onChange={(e) => {
                        if (!flow) return;
                        const updated = { ...flow };
                        updated.bricks[index] = {
                          ...updated.bricks[index],
                          config: {
                            ...updated.bricks[index].config,
                            [key]: e.target.value,
                          },
                        };
                        setFlow(updated);
                      }}
                    >
                      <option value="">Select {prop.title || key}</option>
                      {prop.enum.map((option: string) => (
                        <option key={option} value={option}>{option}</option>
                      ))}
                    </Select>
                  </div>
                );
              }

              if (prop.type === 'string') {
                return (
                  <div key={key}>
                    <Label htmlFor={key} value={prop.title || key} />
                    <TextInput
                      id={key}
                      type="text"
                      value={value || ''}
                      onChange={(e) => {
                        if (!flow) return;
                        const updated = { ...flow };
                        updated.bricks[index] = {
                          ...updated.bricks[index],
                          config: {
                            ...updated.bricks[index].config,
                            [key]: e.target.value,
                          },
                        };
                        setFlow(updated);
                      }}
                      placeholder={prop.description}
                    />
                  </div>
                );
              }

              return null;
            })}
          </div>
        </Modal.Body>
        <Modal.Footer>
          <Button color="gray" onClick={() => setConfigModalOpen(false)}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    );
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <Spinner size="xl" />
      </div>
    );
  }

  if (!flow) {
    return (
      <Alert color="failure">
        Flow not found
      </Alert>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">{flow.name}</h1>
          {flow.description && (
            <p className="text-gray-600 dark:text-gray-400 mt-1">{flow.description}</p>
          )}
        </div>
        <div className="flex gap-2">
          <Button color="light" icon={Clock} onClick={() => setShowExecutions(!showExecutions)}>
            History ({flowExecutions.length})
          </Button>
          <Button color="light" icon={Plus} onClick={() => setShowAddBrick(true)}>
            Add Brick
          </Button>
          <Button color="light" icon={Save} onClick={handleSave} disabled={saving}>
            {saving ? 'Saving...' : 'Save'}
          </Button>
          <Button
            gradientDuoTone="greenToBlue"
            icon={executing ? undefined : Play}
            onClick={handleRunFlow}
            disabled={executing}
          >
            {executing ? (
              <>
                <Spinner size="sm" className="mr-2" />
                Running...
              </>
            ) : (
              'Run Flow'
            )}
          </Button>
        </div>
      </div>

      <Card>
        <div className="h-[600px] relative">
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onConnect={onConnect}
            onNodesChange={(changes) => {
              setNodes((nds) => {
                const updated = [...nds];
                changes.forEach((change: any) => {
                  if (change.type === 'position' && change.position) {
                    const node = updated.find(n => n.id === change.id);
                    if (node) {
                      node.position = change.position;
                    }
                  }
                });
                return updated;
              });
            }}
            onEdgesChange={setEdges}
            onNodeClick={(_, node) => {
              const index = node.data.index;
              setEditingBrick(index);
              setConfigModalOpen(true);
            }}
            fitView
          >
            <Background />
            <Controls />
          </ReactFlow>

          {showAddBrick && (
            <Card className="absolute top-4 right-4 z-10 w-64">
              <div className="flex justify-between items-center mb-2">
                <h3 className="font-semibold">Add Brick</h3>
                <Button size="xs" color="light" onClick={() => setShowAddBrick(false)}>×</Button>
              </div>
              <div className="max-h-64 overflow-y-auto space-y-1">
                {availableBricks.map((brick) => (
                  <Button
                    key={brick.name}
                    color="light"
                    className="w-full justify-start"
                    onClick={() => addBrick(brick.name)}
                  >
                    {brick.name.replace(/_/g, ' ')}
                  </Button>
                ))}
              </div>
            </Card>
          )}

          {showExecutions && flowExecutions.length > 0 && (
            <Card className="absolute top-4 left-4 z-10 w-80">
              <div className="flex justify-between items-center mb-2">
                <h3 className="font-semibold">Recent Executions</h3>
                <Button size="xs" color="light" onClick={() => setShowExecutions(false)}>×</Button>
              </div>
              <div className="space-y-2 max-h-64 overflow-y-auto">
                {flowExecutions.map((exec) => (
                  <div
                    key={exec.execution_id}
                    className="p-2 border rounded hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer"
                    onClick={() => router.push(`/executions`)}
                  >
                    <div className="flex items-center justify-between mb-1">
                      <span className="text-xs font-mono">{exec.execution_id.substring(0, 8)}...</span>
                      <StatusBadge status={exec.status} />
                    </div>
                    <div className="text-xs text-gray-500">
                      {new Date(exec.started_at).toLocaleString()}
                    </div>
                  </div>
                ))}
              </div>
              <Link href="/executions" className="mt-2 block">
                <Button size="xs" color="light" className="w-full">View All</Button>
              </Link>
            </Card>
          )}
        </div>
      </Card>

      {editingBrick !== null && renderBrickConfig(editingBrick)}

      {executionResult && (
        <Modal show={!!executionResult} onClose={() => setExecutionResult(null)}>
          <Modal.Header>Execution Result</Modal.Header>
          <Modal.Body>
            {executionResult.status === 'completed' ? (
              <div>
                <div className="flex items-center gap-2 mb-4 text-green-600">
                  <CheckCircle size={20} />
                  <span className="font-medium">Execution Successful</span>
                </div>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                  Execution ID: <span className="font-mono">{executionResult.execution_id?.substring(0, 8)}...</span>
                </p>
                {executionResult.output_payload && (
                  <div>
                    <Label className="text-sm font-medium">Output:</Label>
                    <pre className="text-xs bg-gray-50 dark:bg-gray-800 p-3 rounded mt-1 overflow-auto max-h-48">
                      {JSON.stringify(executionResult.output_payload, null, 2)}
                    </pre>
                  </div>
                )}
              </div>
            ) : (
              <div>
                <div className="flex items-center gap-2 mb-4 text-red-600">
                  <XCircle size={20} />
                  <span className="font-medium">Execution Failed</span>
                </div>
                <p className="text-sm text-red-600 bg-red-50 dark:bg-red-900/20 p-3 rounded">
                  {executionResult.error || 'Unknown error occurred'}
                </p>
              </div>
            )}
          </Modal.Body>
          <Modal.Footer>
            <Button color="gray" onClick={() => setExecutionResult(null)}>Close</Button>
            {executionResult.status === 'completed' && (
              <Link href="/executions">
                <Button gradientDuoTone="purpleToBlue">View All Executions</Button>
              </Link>
            )}
          </Modal.Footer>
        </Modal>
      )}
    </div>
  );
}
