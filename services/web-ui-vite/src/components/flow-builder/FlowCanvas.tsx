import { useState } from 'react';
import { BrickConfig, BrickTypeInfo } from '../../api/client';
import BrickNode from './BrickNode';
import EmptyState, { EmptyStateIcon } from '../ui/EmptyState';

interface FlowCanvasProps {
  bricks: BrickConfig[];
  availableBricks: BrickTypeInfo[];
  brickSchemas: Record<string, Record<string, any>>;
  onBricksChange: (bricks: BrickConfig[]) => void;
}

export default function FlowCanvas({
  bricks,
  availableBricks,
  brickSchemas,
  onBricksChange,
}: FlowCanvasProps) {
  const [dragSourceIndex, setDragSourceIndex] = useState<number | null>(null);
  const [canvasDragOver, setCanvasDragOver] = useState(false);

  const handleAddBrick = (brickType: string) => {
    const brickInfo = availableBricks.find(
      (b) => b.brick_type === brickType || b.name === brickType
    );

    if (!brickInfo) return;

    const schema = brickSchemas[brickInfo.name] || brickSchemas[brickType] || {};
    const defaultConfig: Record<string, any> = {};

    // Generate default config from schema
    if (schema.properties) {
      Object.entries(schema.properties).forEach(([key, prop]: [string, any]) => {
        if (prop.default !== undefined) {
          defaultConfig[key] = prop.default;
        }
      });
    }

    const newBrick: BrickConfig = {
      brick_type: brickInfo.brick_type,
      config: defaultConfig,
    };

    onBricksChange([...bricks, newBrick]);
  };

  const handleDeleteBrick = (index: number) => {
    const newBricks = bricks.filter((_, i) => i !== index);
    onBricksChange(newBricks);
  };

  const handleConfigUpdate = (index: number, config: Record<string, any>) => {
    const newBricks = [...bricks];
    newBricks[index] = { ...newBricks[index], config };
    onBricksChange(newBricks);
  };

  const handleDragStart = (index: number) => {
    setDragSourceIndex(index);
  };

  const handleDragOver = () => {
    // Visual feedback handled in BrickNode
  };

  const handleDrop = (sourceIndex: number, targetIndex: number) => {
    if (sourceIndex === targetIndex) return;

    const newBricks = [...bricks];
    const [removed] = newBricks.splice(sourceIndex, 1);
    newBricks.splice(targetIndex, 0, removed);
    onBricksChange(newBricks);
    setDragSourceIndex(null);
  };

  const handleCanvasDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setCanvasDragOver(true);
  };

  const handleCanvasDragLeave = () => {
    setCanvasDragOver(false);
  };

  const handleCanvasDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setCanvasDragOver(false);

    const data = e.dataTransfer.getData('text/plain');
    if (data.startsWith('palette:')) {
      const brickType = data.replace('palette:', '');
      handleAddBrick(brickType);
    }
  };

  return (
    <div
      className={`flex-1 bg-gray-50 rounded-lg border-2 p-6 overflow-y-auto transition-colors ${
        canvasDragOver
          ? 'border-primary-500 border-dashed bg-primary-50'
          : 'border-gray-200'
      }`}
      onDragOver={handleCanvasDragOver}
      onDragLeave={handleCanvasDragLeave}
      onDrop={handleCanvasDrop}
    >
      <h3 className="text-sm font-medium text-gray-700 mb-4">Flow Sequence</h3>

      {bricks.length === 0 ? (
        <EmptyState
          title="No bricks added yet"
          description="Drag bricks from the palette or click to add them to your flow"
          icon={EmptyStateIcon.Play}
        />
      ) : (
        <div className="space-y-4">
          {bricks.map((brick, index) => {
            const brickInfo = availableBricks.find(
              (b) => b.brick_type === brick.brick_type || b.name === brick.brick_type
            );
            const schema =
              brickInfo && (brickSchemas[brickInfo.name] || brickSchemas[brick.brick_type])
                ? brickSchemas[brickInfo.name] || brickSchemas[brick.brick_type]
                : null;

            return (
              <div key={index} className="relative">
                {index > 0 && (
                  <div className="absolute left-6 top-0 w-0.5 h-4 bg-gray-300 -translate-y-full z-0" />
                )}
                <BrickNode
                  index={index}
                  brick={brick}
                  schema={schema}
                  onDelete={handleDeleteBrick}
                  onConfigUpdate={handleConfigUpdate}
                  onDragStart={handleDragStart}
                  onDragOver={handleDragOver}
                  onDrop={handleDrop}
                  isDragging={dragSourceIndex === index}
                />
                {index < bricks.length - 1 && (
                  <div className="absolute left-6 bottom-0 w-0.5 h-4 bg-gray-300 translate-y-full z-0" />
                )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
