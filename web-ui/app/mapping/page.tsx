'use client';

import { useState } from 'react';
import { Card, Button, Label, TextInput, Textarea } from 'flowbite-react';
import { GitMerge, Plus, Trash2, Play } from 'lucide-react';

export default function MappingPage() {
  const [inputJson, setInputJson] = useState('{\n  "user": {\n    "name": "John Doe",\n    "age": 30\n  }\n}');
  const [outputJson, setOutputJson] = useState('{}');
  const [mappingRules, setMappingRules] = useState<Array<{
    source: string;
    target: string;
  }>>([]);

  const addMappingRule = () => {
    setMappingRules([...mappingRules, { source: '', target: '' }]);
  };

  const updateMappingRule = (index: number, field: 'source' | 'target', value: string) => {
    const updated = [...mappingRules];
    updated[index][field] = value;
    setMappingRules(updated);
  };

  const removeMappingRule = (index: number) => {
    setMappingRules(mappingRules.filter((_, i) => i !== index));
  };

  const applyMapping = () => {
    try {
      const input = JSON.parse(inputJson);
      const output: any = {};

      mappingRules.forEach((rule) => {
        if (rule.source && rule.target) {
          const sourceValue = getNestedValue(input, rule.source);
          if (sourceValue !== undefined) {
            setNestedValue(output, rule.target, sourceValue);
          }
        }
      });

      setOutputJson(JSON.stringify(output, null, 2));
    } catch (error) {
      alert('Invalid JSON input');
    }
  };

  const getNestedValue = (obj: any, path: string): any => {
    return path.split('.').reduce((current, key) => current?.[key], obj);
  };

  const setNestedValue = (obj: any, path: string, value: any) => {
    const keys = path.split('.');
    const lastKey = keys.pop()!;
    const target = keys.reduce((current, key) => {
      if (!current[key]) current[key] = {};
      return current[key];
    }, obj);
    target[lastKey] = value;
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <GitMerge className="text-blue-600 dark:text-blue-400" size={32} />
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Field Mapping Builder</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            Build dynamic field mappings between data sources
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">Input JSON</h2>
          <Textarea
            value={inputJson}
            onChange={(e) => setInputJson(e.target.value)}
            rows={12}
            className="font-mono text-sm"
            placeholder="Enter input JSON..."
          />
        </Card>

        <Card>
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">Output JSON</h2>
          <Textarea
            value={outputJson}
            readOnly
            rows={12}
            className="font-mono text-sm bg-gray-50 dark:bg-gray-800"
            placeholder="Output will appear here..."
          />
        </Card>
      </div>

      <Card>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">Mapping Rules</h2>
          <Button size="sm" color="light" icon={Plus} onClick={addMappingRule}>
            Add Rule
          </Button>
        </div>

        {mappingRules.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            <p>No mapping rules yet. Click "Add Rule" to create one.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {mappingRules.map((rule, index) => (
              <Card key={index}>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <Label htmlFor={`source-${index}`} value="Source Path" />
                    <TextInput
                      id={`source-${index}`}
                      type="text"
                      value={rule.source}
                      onChange={(e) => updateMappingRule(index, 'source', e.target.value)}
                      placeholder="user.name"
                    />
                  </div>
                  <div>
                    <Label htmlFor={`target-${index}`} value="Target Path" />
                    <div className="flex gap-2">
                      <TextInput
                        id={`target-${index}`}
                        type="text"
                        value={rule.target}
                        onChange={(e) => updateMappingRule(index, 'target', e.target.value)}
                        placeholder="customer.name"
                        className="flex-1"
                      />
                      <Button
                        size="sm"
                        color="failure"
                        icon={Trash2}
                        onClick={() => removeMappingRule(index)}
                      >
                        Remove
                      </Button>
                    </div>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        )}

        <div className="mt-4 flex justify-end">
          <Button gradientDuoTone="purpleToBlue" icon={Play} onClick={applyMapping}>
            Apply Mapping
          </Button>
        </div>
      </Card>
    </div>
  );
}
