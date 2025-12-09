'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { Card, Button, Label, TextInput, Textarea, Select, Modal, Alert, Spinner } from 'flowbite-react';
import { Plus, X, Trash2, Save } from 'lucide-react';
import { api } from '@/lib/api';

interface BrickSchema {
  brick_type: string;
  name: string;
  config_schema: any;
}

interface BrickConfig {
  brick_type: string;
  config: any;
}

export default function NewFlowPage() {
  const router = useRouter();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [bricks, setBricks] = useState<BrickSchema[]>([]);
  const [selectedBricks, setSelectedBricks] = useState<BrickConfig[]>([]);
  const [configuringBrick, setConfiguringBrick] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchBricks();
  }, []);

  const fetchBricks = async () => {
    try {
      const response = await api.bricks.list();
      setBricks(response.bricks);
    } catch (err) {
      console.error('Failed to fetch bricks:', err);
      setError('Failed to load available bricks');
    } finally {
      setLoading(false);
    }
  };

  const addBrick = (brickType: string) => {
    const brick = bricks.find(b => b.name === brickType);
    if (!brick) return;

    const defaultConfig = generateDefaultConfig(brick.config_schema);
    setSelectedBricks([...selectedBricks, { brick_type: brickType, config: defaultConfig }]);
    setConfiguringBrick(selectedBricks.length);
  };

  const removeBrick = (index: number) => {
    setSelectedBricks(selectedBricks.filter((_, i) => i !== index));
    if (configuringBrick === index) {
      setConfiguringBrick(null);
    } else if (configuringBrick !== null && configuringBrick > index) {
      setConfiguringBrick(configuringBrick - 1);
    }
  };

  const updateBrickConfig = (index: number, config: any) => {
    const updated = [...selectedBricks];
    updated[index] = { ...updated[index], config };
    setSelectedBricks(updated);
  };

  const generateDefaultConfig = (schema: any): any => {
    if (!schema.properties) return {};
    
    const config: any = {};
    Object.keys(schema.properties).forEach(key => {
      const prop = schema.properties[key];
      if (prop.default !== undefined) {
        config[key] = prop.default;
      } else if (prop.type === 'string') {
        config[key] = '';
      } else if (prop.type === 'number') {
        config[key] = 0;
      } else if (prop.type === 'boolean') {
        config[key] = false;
      } else if (prop.type === 'array') {
        config[key] = [];
      } else if (prop.type === 'object') {
        config[key] = {};
      }
    });
    return config;
  };

  const renderConfigField = (key: string, prop: any, value: any, onChange: (value: any) => void) => {
    if (prop.type === 'string' && prop.enum) {
      return (
        <div key={key} className="mb-4">
          <Label htmlFor={key} value={prop.title || key} />
          <Select id={key} value={value || ''} onChange={(e) => onChange(e.target.value)}>
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
        <div key={key} className="mb-4">
          <Label htmlFor={key} value={prop.title || key} />
          <TextInput
            id={key}
            type="text"
            value={value || ''}
            onChange={(e) => onChange(e.target.value)}
            placeholder={prop.description}
          />
        </div>
      );
    }

    if (prop.type === 'number') {
      return (
        <div key={key} className="mb-4">
          <Label htmlFor={key} value={prop.title || key} />
          <TextInput
            id={key}
            type="number"
            value={value || ''}
            onChange={(e) => onChange(Number(e.target.value))}
            placeholder={prop.description}
          />
        </div>
      );
    }

    if (prop.type === 'boolean') {
      return (
        <div key={key} className="mb-4 flex items-center gap-2">
          <input
            type="checkbox"
            id={key}
            checked={value || false}
            onChange={(e) => onChange(e.target.checked)}
            className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
          />
          <Label htmlFor={key} value={prop.title || key} />
        </div>
      );
    }

    return null;
  };

  const handleSave = async () => {
    if (!name.trim()) {
      setError('Flow name is required');
      return;
    }

    if (selectedBricks.length === 0) {
      setError('At least one brick is required');
      return;
    }

    setError(null);
    setSaving(true);

    try {
      const flow = await api.flows.create({
        name,
        description: description || undefined,
        bricks: selectedBricks.map(b => ({
          brick_type: b.brick_type as any,
          config: b.config,
        })),
      });
      
      router.push(`/flows/${flow.id}`);
    } catch (err: any) {
      console.error('Failed to create flow:', err);
      setError(err.message || 'Failed to create flow');
    } finally {
      setSaving(false);
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
      <div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Create New Flow</h1>
        <p className="text-gray-600 dark:text-gray-400 mt-1">
          Build your automation flow by adding and configuring bricks
        </p>
      </div>

      {error && (
        <Alert color="failure" onDismiss={() => setError(null)}>
          {error}
        </Alert>
      )}

      <Card>
        <div className="space-y-4">
          <div>
            <Label htmlFor="name" value="Flow Name *" />
            <TextInput
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="My Automation Flow"
              required
            />
          </div>

          <div>
            <Label htmlFor="description" value="Description" />
            <Textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Describe what this flow does..."
              rows={3}
            />
          </div>
        </div>
      </Card>

      <Card>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold text-gray-900 dark:text-white">Bricks</h2>
          <Modal show={configuringBrick === null && selectedBricks.length === 0} onClose={() => {}}>
            <Modal.Header>Add Brick</Modal.Header>
            <Modal.Body>
              <div className="space-y-2 max-h-64 overflow-y-auto">
                {bricks.map((brick) => (
                  <Button
                    key={brick.name}
                    color="light"
                    className="w-full justify-start"
                    onClick={() => {
                      addBrick(brick.name);
                    }}
                  >
                    {brick.name.replace(/_/g, ' ')}
                  </Button>
                ))}
              </div>
            </Modal.Body>
          </Modal>
          <Button color="light" icon={Plus} onClick={() => setConfiguringBrick(null)}>
            Add Brick
          </Button>
        </div>

        {selectedBricks.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <p>No bricks added yet. Click "Add Brick" to get started.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {selectedBricks.map((brickConfig, index) => {
              const brick = bricks.find(b => b.name === brickConfig.brick_type);
              if (!brick) return null;

              return (
                <Card key={index}>
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-white capitalize">
                      {brickConfig.brick_type.replace(/_/g, ' ')}
                    </h3>
                    <Button
                      size="xs"
                      color="failure"
                      icon={Trash2}
                      onClick={() => removeBrick(index)}
                    >
                      Remove
                    </Button>
                  </div>

                  {configuringBrick === index && brick.config_schema.properties && (
                    <div className="space-y-4">
                      {Object.keys(brick.config_schema.properties).map(key => {
                        const prop = brick.config_schema.properties[key];
                        return renderConfigField(
                          key,
                          prop,
                          brickConfig.config[key],
                          (value) => {
                            const updated = { ...brickConfig.config };
                            updated[key] = value;
                            updateBrickConfig(index, updated);
                          }
                        );
                      })}
                    </div>
                  )}

                  {configuringBrick !== index && (
                    <Button
                      color="light"
                      onClick={() => setConfiguringBrick(index)}
                    >
                      Configure
                    </Button>
                  )}
                </Card>
              );
            })}
          </div>
        )}
      </Card>

      <div className="flex justify-end gap-3">
        <Button color="gray" onClick={() => router.back()}>
          Cancel
        </Button>
        <Button
          gradientDuoTone="purpleToBlue"
          icon={Save}
          onClick={handleSave}
          disabled={saving || !name.trim() || selectedBricks.length === 0}
        >
          {saving ? 'Saving...' : 'Save Flow'}
        </Button>
      </div>
    </div>
  );
}
