import { useState } from 'react';
import { LinkIcon, Plus, X, ArrowRight, ArrowLeft, ArrowRightLeft } from '../components/ui/icons';

type MappingDirection = 'forward' | 'backward' | 'bidirectional';
type MergeStrategy = 'concat' | 'merge_object' | 'array' | 'first' | 'last';
type SplitStrategy = 'copy' | 'extract' | 'transform_each';

interface MappingRule {
  sourcePaths: string[];
  targetPaths: string[];
  direction: MappingDirection;
  mergeStrategy?: MergeStrategy;
  splitStrategy?: SplitStrategy;
}

export default function Mapping() {
  const [inputJson, setInputJson] = useState(`{
  "user": {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
  },
  "company": {
    "name": "Acme Corp"
  }
}`);
  const [outputJson, setOutputJson] = useState('{}');
  const [mappingRules, setMappingRules] = useState<MappingRule[]>([]);

  const addRule = () => {
    setMappingRules([
      ...mappingRules,
      {
        sourcePaths: [''],
        targetPaths: [''],
        direction: 'forward',
      },
    ]);
  };

  const updateRule = (
    index: number,
    field: 'sourcePaths' | 'targetPaths' | 'direction' | 'mergeStrategy' | 'splitStrategy',
    value: any
  ) => {
    const newRules = [...mappingRules];
    if (field === 'sourcePaths' || field === 'targetPaths') {
      newRules[index] = { ...newRules[index], [field]: value };
    } else {
      newRules[index] = { ...newRules[index], [field]: value };
    }
    setMappingRules(newRules);
  };

  const addSourcePath = (ruleIndex: number) => {
    const newRules = [...mappingRules];
    newRules[ruleIndex].sourcePaths.push('');
    setMappingRules(newRules);
  };

  const removeSourcePath = (ruleIndex: number, pathIndex: number) => {
    const newRules = [...mappingRules];
    newRules[ruleIndex].sourcePaths.splice(pathIndex, 1);
    if (newRules[ruleIndex].sourcePaths.length === 0) {
      newRules[ruleIndex].sourcePaths.push('');
    }
    setMappingRules(newRules);
  };

  const updateSourcePath = (ruleIndex: number, pathIndex: number, value: string) => {
    const newRules = [...mappingRules];
    newRules[ruleIndex].sourcePaths[pathIndex] = value;
    setMappingRules(newRules);
  };

  const addTargetPath = (ruleIndex: number) => {
    const newRules = [...mappingRules];
    newRules[ruleIndex].targetPaths.push('');
    setMappingRules(newRules);
  };

  const removeTargetPath = (ruleIndex: number, pathIndex: number) => {
    const newRules = [...mappingRules];
    newRules[ruleIndex].targetPaths.splice(pathIndex, 1);
    if (newRules[ruleIndex].targetPaths.length === 0) {
      newRules[ruleIndex].targetPaths.push('');
    }
    setMappingRules(newRules);
  };

  const updateTargetPath = (ruleIndex: number, pathIndex: number, value: string) => {
    const newRules = [...mappingRules];
    newRules[ruleIndex].targetPaths[pathIndex] = value;
    setMappingRules(newRules);
  };

  const removeRule = (index: number) => {
    setMappingRules(mappingRules.filter((_, i) => i !== index));
  };

  const applyMapping = () => {
    try {
      const input = JSON.parse(inputJson);
      const output: Record<string, any> = {};

      for (const rule of mappingRules) {
        const validSourcePaths = rule.sourcePaths.filter((p) => p.trim());
        const validTargetPaths = rule.targetPaths.filter((p) => p.trim());

        if (validSourcePaths.length === 0 || validTargetPaths.length === 0) continue;

        // Handle different mapping scenarios
        if (validSourcePaths.length === 1 && validTargetPaths.length === 1) {
          // Single to single
          const value = getNestedValue(input, validSourcePaths[0]);
          if (value !== undefined) {
            setNestedValue(output, validTargetPaths[0], value);
          }
        } else if (validSourcePaths.length > 1 && validTargetPaths.length === 1) {
          // Multiple sources to single target (merge)
          const values = validSourcePaths
            .map((path) => getNestedValue(input, path))
            .filter((v) => v !== undefined);

          if (values.length > 0) {
            const merged = mergeValues(values, rule.mergeStrategy);
            setNestedValue(output, validTargetPaths[0], merged);
          }
        } else if (validSourcePaths.length === 1 && validTargetPaths.length > 1) {
          // Single source to multiple targets (split)
          const value = getNestedValue(input, validSourcePaths[0]);
          if (value !== undefined) {
            splitValue(output, validTargetPaths, value, rule.splitStrategy);
          }
        } else {
          // Multiple to multiple (1:1 mapping)
          validSourcePaths.forEach((sourcePath, idx) => {
            if (idx < validTargetPaths.length) {
              const value = getNestedValue(input, sourcePath);
              if (value !== undefined) {
                setNestedValue(output, validTargetPaths[idx], value);
              }
            }
          });
        }

        // Handle backward and bidirectional
        if (rule.direction === 'backward' || rule.direction === 'bidirectional') {
          // Reverse mapping
          if (validSourcePaths.length === 1 && validTargetPaths.length === 1) {
            const value = getNestedValue(output, validTargetPaths[0]);
            if (value !== undefined) {
              setNestedValue(input as any, validSourcePaths[0], value);
            }
          }
        }
      }

      setOutputJson(JSON.stringify(output, null, 2));
    } catch (error) {
      console.error('Invalid JSON:', error);
      alert('Invalid JSON input. Please check your input format.');
    }
  };

  const mergeValues = (values: any[], strategy?: MergeStrategy): any => {
    switch (strategy) {
      case 'concat':
        return values.map((v) => String(v)).join(' ');
      case 'merge_object':
        return values.reduce((acc, v) => ({ ...acc, ...(typeof v === 'object' ? v : {}) }), {});
      case 'array':
        return values;
      case 'first':
        return values[0];
      case 'last':
        return values[values.length - 1];
      default:
        return values.length > 0 ? values[0] : null;
    }
  };

  const splitValue = (
    output: Record<string, any>,
    targetPaths: string[],
    value: any,
    strategy?: SplitStrategy
  ) => {
    switch (strategy) {
      case 'extract':
        if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
          const keys = Object.keys(value);
          targetPaths.forEach((targetPath, idx) => {
            if (idx < keys.length) {
              setNestedValue(output, targetPath, value[keys[idx]]);
            }
          });
        } else {
          // Copy to all if not object
          targetPaths.forEach((targetPath) => {
            setNestedValue(output, targetPath, value);
          });
        }
        break;
      case 'transform_each':
        // For now, just copy (transform would need per-target config)
        targetPaths.forEach((targetPath) => {
          setNestedValue(output, targetPath, value);
        });
        break;
      default:
        // Copy to all targets
        targetPaths.forEach((targetPath) => {
          setNestedValue(output, targetPath, value);
        });
    }
  };

  const getNestedValue = (obj: any, path: string): any => {
    return path.split('.').reduce((current, key) => current?.[key], obj);
  };

  const setNestedValue = (obj: Record<string, any>, path: string, value: any) => {
    const keys = path.split('.');
    const lastKey = keys.pop()!;
    const target = keys.reduce((current, key) => {
      if (!current[key]) {
        current[key] = {};
      }
      return current[key];
    }, obj);
    target[lastKey] = value;
  };

  const getDirectionIcon = (direction: MappingDirection) => {
    switch (direction) {
      case 'forward':
        return <ArrowRight size={16} />;
      case 'backward':
        return <ArrowLeft size={16} />;
      case 'bidirectional':
        return <ArrowRightLeft size={16} />;
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <LinkIcon size={32} className="text-primary-600" />
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Field Mapping Builder</h1>
          <p className="text-gray-600 mt-1">Build dynamic multi-directional field mappings between data sources</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Input JSON</h2>
          <textarea
            className="w-full font-mono text-sm border border-gray-300 rounded p-3"
            rows={12}
            value={inputJson}
            onChange={(e) => setInputJson(e.target.value)}
            placeholder="Enter input JSON..."
          />
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-6">Output JSON</h2>
          <textarea
            className="w-full font-mono text-sm bg-gray-50 border border-gray-300 rounded p-3"
            rows={12}
            readOnly
            value={outputJson}
            placeholder="Output will appear here..."
          />
        </div>
      </div>

      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-semibold text-gray-900">Mapping Rules</h2>
          <button
            className="flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200"
            onClick={addRule}
          >
            <Plus size={16} className="mr-2" />
            Add Rule
          </button>
        </div>

        {mappingRules.length === 0 ? (
          <div className="text-center py-12 text-gray-500">
            <p>No mapping rules yet. Click "Add Rule" to create one.</p>
          </div>
        ) : (
          <div className="space-y-6">
            {mappingRules.map((rule, index) => (
              <div key={index} className="bg-gray-50 rounded-lg p-4 border border-gray-200">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-medium text-gray-700">Rule {index + 1}</span>
                    <div className="flex items-center gap-1 px-2 py-1 bg-white rounded border border-gray-300">
                      {getDirectionIcon(rule.direction)}
                      <select
                        value={rule.direction}
                        onChange={(e) =>
                          updateRule(index, 'direction', e.target.value as MappingDirection)
                        }
                        className="text-xs font-medium text-gray-700 bg-transparent border-0 focus:ring-0"
                      >
                        <option value="forward">Forward</option>
                        <option value="backward">Backward</option>
                        <option value="bidirectional">Bidirectional</option>
                      </select>
                    </div>
                  </div>
                  <button
                    className="px-3 py-1.5 text-sm font-medium text-white bg-red-600 rounded-lg hover:bg-red-700"
                    onClick={() => removeRule(index)}
                  >
                    <X size={16} />
                  </button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* Source Paths */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Source Paths
                      {rule.sourcePaths.length > 1 && (
                        <span className="ml-2 text-xs text-gray-500">
                          ({rule.sourcePaths.length} paths)
                        </span>
                      )}
                    </label>
                    <div className="space-y-2">
                      {rule.sourcePaths.map((path, pathIdx) => (
                        <div key={pathIdx} className="flex gap-2">
                          <input
                            type="text"
                            className="flex-1 px-3 py-2 border border-gray-300 rounded text-sm"
                            value={path}
                            onChange={(e) => updateSourcePath(index, pathIdx, e.target.value)}
                            placeholder="user.name"
                          />
                          {rule.sourcePaths.length > 1 && (
                            <button
                              onClick={() => removeSourcePath(index, pathIdx)}
                              className="px-2 py-1 text-red-600 hover:bg-red-50 rounded"
                            >
                              <X size={14} />
                            </button>
                          )}
                        </div>
                      ))}
                      <button
                        onClick={() => addSourcePath(index)}
                        className="flex items-center text-xs text-primary-600 hover:text-primary-700"
                      >
                        <Plus size={14} className="mr-1" />
                        Add Source Path
                      </button>
                    </div>
                    {rule.sourcePaths.length > 1 && rule.targetPaths.length === 1 && (
                      <div className="mt-2">
                        <label className="block text-xs font-medium text-gray-600 mb-1">
                          Merge Strategy
                        </label>
                        <select
                          value={rule.mergeStrategy || 'concat'}
                          onChange={(e) =>
                            updateRule(index, 'mergeStrategy', e.target.value as MergeStrategy)
                          }
                          className="w-full px-2 py-1 text-xs border border-gray-300 rounded"
                        >
                          <option value="concat">Concatenate</option>
                          <option value="merge_object">Merge Objects</option>
                          <option value="array">Combine as Array</option>
                          <option value="first">Use First</option>
                          <option value="last">Use Last</option>
                        </select>
                      </div>
                    )}
                  </div>

                  {/* Target Paths */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Target Paths
                      {rule.targetPaths.length > 1 && (
                        <span className="ml-2 text-xs text-gray-500">
                          ({rule.targetPaths.length} paths)
                        </span>
                      )}
                    </label>
                    <div className="space-y-2">
                      {rule.targetPaths.map((path, pathIdx) => (
                        <div key={pathIdx} className="flex gap-2">
                          <input
                            type="text"
                            className="flex-1 px-3 py-2 border border-gray-300 rounded text-sm"
                            value={path}
                            onChange={(e) => updateTargetPath(index, pathIdx, e.target.value)}
                            placeholder="customer.name"
                          />
                          {rule.targetPaths.length > 1 && (
                            <button
                              onClick={() => removeTargetPath(index, pathIdx)}
                              className="px-2 py-1 text-red-600 hover:bg-red-50 rounded"
                            >
                              <X size={14} />
                            </button>
                          )}
                        </div>
                      ))}
                      <button
                        onClick={() => addTargetPath(index)}
                        className="flex items-center text-xs text-primary-600 hover:text-primary-700"
                      >
                        <Plus size={14} className="mr-1" />
                        Add Target Path
                      </button>
                    </div>
                    {rule.sourcePaths.length === 1 && rule.targetPaths.length > 1 && (
                      <div className="mt-2">
                        <label className="block text-xs font-medium text-gray-600 mb-1">
                          Split Strategy
                        </label>
                        <select
                          value={rule.splitStrategy || 'copy'}
                          onChange={(e) =>
                            updateRule(index, 'splitStrategy', e.target.value as SplitStrategy)
                          }
                          className="w-full px-2 py-1 text-xs border border-gray-300 rounded"
                        >
                          <option value="copy">Copy to All</option>
                          <option value="extract">Extract Nested</option>
                          <option value="transform_each">Transform Each</option>
                        </select>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        <div className="mt-6 flex justify-end">
          <button
            className="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700"
            onClick={applyMapping}
          >
            Apply Mapping
          </button>
        </div>
      </div>
    </div>
  );
}
