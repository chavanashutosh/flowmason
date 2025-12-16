import { useState } from 'react';
import { BrickConfig } from '../../api/client';
import BrickConfigForm from './BrickConfigForm';
import { X, GripVertical } from '../ui/icons';

interface BrickNodeProps {
  index: number;
  brick: BrickConfig;
  schema: Record<string, any> | null;
  onDelete: (index: number) => void;
  onConfigUpdate: (index: number, config: Record<string, any>) => void;
  onDragStart: (index: number) => void;
  onDragOver: (index: number) => void;
  onDrop: (sourceIndex: number, targetIndex: number) => void;
  isDragging: boolean;
}

function formatBrickName(brickType: string): string {
  return brickType
    .split('_')
    .map((word) => {
      if (word.length === 0) return '';
      return word[0].toUpperCase() + word.slice(1);
    })
    .join(' ');
}

export default function BrickNode({
  index,
  brick,
  schema,
  onDelete,
  onConfigUpdate,
  onDragStart,
  onDragOver,
  onDrop,
  isDragging,
}: BrickNodeProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const handleDragStart = (e: React.DragEvent) => {
    e.dataTransfer.setData('text/plain', `brick:${index}`);
    e.dataTransfer.effectAllowed = 'move';
    onDragStart(index);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    onDragOver(index);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    const data = e.dataTransfer.getData('text/plain');
    if (data.startsWith('brick:')) {
      const sourceIndex = parseInt(data.split(':')[1], 10);
      if (!isNaN(sourceIndex) && sourceIndex !== index) {
        onDrop(sourceIndex, index);
      }
    }
  };

  const handleConfigChange = (config: Record<string, any>) => {
    onConfigUpdate(index, config);
  };

  return (
    <div
      draggable
      onDragStart={handleDragStart}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
      className={`mb-4 bg-white border-2 rounded-lg transition-all ${
        isDragging
          ? 'border-primary-500 shadow-lg opacity-50'
          : 'border-gray-200 hover:border-gray-300'
      }`}
    >
      <div className="flex items-center justify-between p-4">
        <div className="flex items-center gap-3 flex-1">
          <div
            className="cursor-move text-gray-400 hover:text-gray-600"
            title="Drag to reorder"
          >
            <GripVertical size={20} />
          </div>
          <div className="flex-1">
            <div className="flex items-center gap-2">
              <span className="text-sm font-semibold text-gray-900">
                {formatBrickName(brick.brick_type)}
              </span>
              <span className="text-xs text-gray-500 bg-gray-100 px-2 py-0.5 rounded">
                Step {index + 1}
              </span>
            </div>
            {schema && (
              <button
                onClick={() => setIsExpanded(!isExpanded)}
                className="mt-1 text-xs text-primary-600 hover:text-primary-700"
              >
                {isExpanded ? 'Hide' : 'Show'} Configuration
              </button>
            )}
          </div>
        </div>
        <button
          onClick={() => onDelete(index)}
          className="p-1 text-gray-400 hover:text-red-600 transition-colors"
          title="Delete brick"
        >
          <X size={18} />
        </button>
      </div>

      {isExpanded && schema && (
        <div className="border-t border-gray-200">
          <BrickConfigForm
            brickType={brick.brick_type}
            config={brick.config}
            schema={schema}
            onChange={handleConfigChange}
          />
        </div>
      )}
    </div>
  );
}
