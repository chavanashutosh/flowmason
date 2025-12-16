import { useState, useEffect } from 'react';
import { BrickConfig } from '../../api/client';

interface BrickConfigFormProps {
  brickType: string;
  config: Record<string, any>;
  schema: Record<string, any>;
  onChange: (config: Record<string, any>) => void;
}

function getDefaultValue(property: any): any {
  if (property.default !== undefined) {
    return property.default;
  }
  switch (property.type) {
    case 'string':
      return '';
    case 'number':
      return 0;
    case 'boolean':
      return false;
    case 'array':
      return [];
    case 'object':
      return {};
    default:
      return null;
  }
}

function renderField(
  key: string,
  property: any,
  value: any,
  onChange: (key: string, value: any) => void,
  prefix = ''
): JSX.Element {
  const fieldKey = prefix ? `${prefix}.${key}` : key;
  const isRequired = property.required || false;
  const fieldId = `field-${fieldKey.replace(/\./g, '-')}`;

  switch (property.type) {
    case 'string':
      if (property.enum) {
        return (
          <div key={fieldKey} className="mb-4">
            <label
              htmlFor={fieldId}
              className="block text-sm font-medium text-gray-700 mb-2"
            >
              {property.title || key}
              {isRequired && <span className="text-red-500 ml-1">*</span>}
            </label>
            <select
              id={fieldId}
              value={value || ''}
              onChange={(e) => onChange(key, e.target.value)}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            >
              <option value="">Select...</option>
              {property.enum.map((option: any) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>
            {property.description && (
              <p className="mt-1 text-xs text-gray-500">{property.description}</p>
            )}
          </div>
        );
      }
      if (property.format === 'textarea' || (property.maxLength && property.maxLength > 100)) {
        return (
          <div key={fieldKey} className="mb-4">
            <label
              htmlFor={fieldId}
              className="block text-sm font-medium text-gray-700 mb-2"
            >
              {property.title || key}
              {isRequired && <span className="text-red-500 ml-1">*</span>}
            </label>
            <textarea
              id={fieldId}
              value={value || ''}
              onChange={(e) => onChange(key, e.target.value)}
              rows={property.rows || 3}
              className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
              placeholder={property.placeholder || ''}
            />
            {property.description && (
              <p className="mt-1 text-xs text-gray-500">{property.description}</p>
            )}
          </div>
        );
      }
      return (
        <div key={fieldKey} className="mb-4">
          <label htmlFor={fieldId} className="block text-sm font-medium text-gray-700 mb-2">
            {property.title || key}
            {isRequired && <span className="text-red-500 ml-1">*</span>}
          </label>
          <input
            id={fieldId}
            type="text"
            value={value || ''}
            onChange={(e) => onChange(key, e.target.value)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            placeholder={property.placeholder || ''}
          />
          {property.description && (
            <p className="mt-1 text-xs text-gray-500">{property.description}</p>
          )}
        </div>
      );

    case 'number':
    case 'integer':
      return (
        <div key={fieldKey} className="mb-4">
          <label htmlFor={fieldId} className="block text-sm font-medium text-gray-700 mb-2">
            {property.title || key}
            {isRequired && <span className="text-red-500 ml-1">*</span>}
          </label>
          <input
            id={fieldId}
            type="number"
            value={value ?? ''}
            onChange={(e) => onChange(key, e.target.value ? Number(e.target.value) : null)}
            min={property.minimum}
            max={property.maximum}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
          {property.description && (
            <p className="mt-1 text-xs text-gray-500">{property.description}</p>
          )}
        </div>
      );

    case 'boolean':
      return (
        <div key={fieldKey} className="mb-4">
          <label className="flex items-center">
            <input
              type="checkbox"
              checked={value || false}
              onChange={(e) => onChange(key, e.target.checked)}
              className="mr-2 h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <span className="text-sm font-medium text-gray-700">
              {property.title || key}
              {isRequired && <span className="text-red-500 ml-1">*</span>}
            </span>
          </label>
          {property.description && (
            <p className="mt-1 text-xs text-gray-500 ml-6">{property.description}</p>
          )}
        </div>
      );

    case 'array':
      return (
        <div key={fieldKey} className="mb-4">
          <label htmlFor={fieldId} className="block text-sm font-medium text-gray-700 mb-2">
            {property.title || key}
            {isRequired && <span className="text-red-500 ml-1">*</span>}
          </label>
          <textarea
            id={fieldId}
            value={Array.isArray(value) ? JSON.stringify(value, null, 2) : ''}
            onChange={(e) => {
              try {
                const parsed = JSON.parse(e.target.value);
                if (Array.isArray(parsed)) {
                  onChange(key, parsed);
                }
              } catch {
                // Invalid JSON, ignore
              }
            }}
            rows={4}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 font-mono text-sm"
            placeholder="[]"
          />
          {property.description && (
            <p className="mt-1 text-xs text-gray-500">{property.description}</p>
          )}
        </div>
      );

    case 'object':
      if (property.properties) {
        return (
          <div key={fieldKey} className="mb-4 p-4 bg-gray-50 rounded-lg border border-gray-200">
            <label className="block text-sm font-medium text-gray-700 mb-3">
              {property.title || key}
              {isRequired && <span className="text-red-500 ml-1">*</span>}
            </label>
            {Object.entries(property.properties).map(([subKey, subProperty]: [string, any]) =>
              renderField(
                subKey,
                subProperty,
                value?.[subKey],
                (k, v) => {
                  onChange(key, { ...(value || {}), [k]: v });
                },
                fieldKey
              )
            )}
            {property.description && (
              <p className="mt-2 text-xs text-gray-500">{property.description}</p>
            )}
          </div>
        );
      }
      return (
        <div key={fieldKey} className="mb-4">
          <label htmlFor={fieldId} className="block text-sm font-medium text-gray-700 mb-2">
            {property.title || key}
            {isRequired && <span className="text-red-500 ml-1">*</span>}
          </label>
          <textarea
            id={fieldId}
            value={typeof value === 'object' ? JSON.stringify(value, null, 2) : ''}
            onChange={(e) => {
              try {
                const parsed = JSON.parse(e.target.value);
                onChange(key, parsed);
              } catch {
                // Invalid JSON, ignore
              }
            }}
            rows={6}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 font-mono text-sm"
            placeholder="{}"
          />
          {property.description && (
            <p className="mt-1 text-xs text-gray-500">{property.description}</p>
          )}
        </div>
      );

    default:
      return (
        <div key={fieldKey} className="mb-4">
          <label htmlFor={fieldId} className="block text-sm font-medium text-gray-700 mb-2">
            {property.title || key}
          </label>
          <textarea
            id={fieldId}
            value={typeof value === 'object' ? JSON.stringify(value, null, 2) : String(value || '')}
            onChange={(e) => {
              try {
                const parsed = JSON.parse(e.target.value);
                onChange(key, parsed);
              } catch {
                onChange(key, e.target.value);
              }
            }}
            rows={3}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 font-mono text-sm"
          />
        </div>
      );
  }
}

export default function BrickConfigForm({ brickType, config, schema, onChange }: BrickConfigFormProps) {
  const [localConfig, setLocalConfig] = useState<Record<string, any>>(config || {});

  useEffect(() => {
    setLocalConfig(config || {});
  }, [config]);

  const handleFieldChange = (key: string, value: any) => {
    const newConfig = { ...localConfig, [key]: value };
    setLocalConfig(newConfig);
    onChange(newConfig);
  };

  if (!schema || !schema.properties) {
    return (
      <div className="p-4 text-sm text-gray-500">
        No configuration required for this brick type.
      </div>
    );
  }

  const properties = schema.properties;
  const required = schema.required || [];

  return (
    <div className="p-4">
      <h3 className="text-sm font-medium text-gray-700 mb-4">Configuration</h3>
      <div className="space-y-4">
        {Object.entries(properties).map(([key, property]: [string, any]) => {
          const propertyWithRequired = { ...property, required: required.includes(key) };
          return renderField(key, propertyWithRequired, localConfig[key], handleFieldChange);
        })}
      </div>
    </div>
  );
}
