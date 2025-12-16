import { BrickTypeInfo } from '../../api/client';

interface BrickPaletteProps {
  availableBricks: BrickTypeInfo[];
  onAddBrick: (brickType: string) => void;
}

function isIntegrationBrick(brickType: string): boolean {
  return ['openai', 'nvidia', 'hubspot', 'notion', 'odoo', 'n8n'].includes(brickType);
}

function formatBrickName(name: string): string {
  return name
    .split('_')
    .map((word) => {
      if (word.length === 0) return '';
      return word[0].toUpperCase() + word.slice(1);
    })
    .join(' ');
}

export default function BrickPalette({ availableBricks, onAddBrick }: BrickPaletteProps) {
  const integrationBricks = availableBricks.filter((b) => isIntegrationBrick(b.brick_type));
  const processingBricks = availableBricks.filter((b) => !isIntegrationBrick(b.brick_type));

  const handleDragStart = (e: React.DragEvent, brickType: string) => {
    e.dataTransfer.setData('text/plain', `palette:${brickType}`);
    e.dataTransfer.effectAllowed = 'copy';
  };

  const renderBrickCard = (brick: BrickTypeInfo) => (
    <div
      key={brick.brick_type}
      draggable
      onDragStart={(e) => handleDragStart(e, brick.brick_type)}
      onClick={() => onAddBrick(brick.brick_type)}
      className="w-full text-left px-3 py-2 bg-white border border-gray-200 rounded-lg hover:border-primary-500 hover:bg-primary-50 transition-colors cursor-move"
    >
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium text-gray-900">{formatBrickName(brick.name)}</span>
        <span className="text-xs text-gray-400">⋮⋮</span>
      </div>
    </div>
  );

  return (
    <div className="w-64 bg-gray-50 border-r border-gray-200 p-4 overflow-y-auto h-full">
      <h2 className="text-lg font-semibold text-gray-900 mb-4">Brick Palette</h2>

      <div className="space-y-6">
        {integrationBricks.length > 0 && (
          <div>
            <h3 className="text-sm font-medium text-gray-700 mb-2 uppercase tracking-wide">
              Integrations
            </h3>
            <div className="space-y-2">{integrationBricks.map(renderBrickCard)}</div>
          </div>
        )}

        {processingBricks.length > 0 && (
          <div>
            <h3 className="text-sm font-medium text-gray-700 mb-2 uppercase tracking-wide">
              Processing
            </h3>
            <div className="space-y-2">{processingBricks.map(renderBrickCard)}</div>
          </div>
        )}
      </div>
    </div>
  );
}
