import { useState, useEffect } from 'react';
import { BrickConfig, BrickTypeInfo, api } from '../../api/client';
import BrickPalette from './BrickPalette';
import FlowCanvas from './FlowCanvas';

interface FlowBuilderProps {
  bricks: BrickConfig[];
  onBricksChange: (bricks: BrickConfig[]) => void;
}

export default function FlowBuilder({ bricks, onBricksChange }: FlowBuilderProps) {
  const [availableBricks, setAvailableBricks] = useState<BrickTypeInfo[]>([]);
  const [brickSchemas, setBrickSchemas] = useState<Record<string, Record<string, any>>>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadBricks();
  }, []);

  const loadBricks = async () => {
    try {
      setLoading(true);
      setError(null);

      // Load available bricks
      const bricksResponse = await api.bricksList();
      const bricksList = bricksResponse.bricks || [];
      setAvailableBricks(bricksList);

      // Load schemas for all bricks
      const schemaPromises = bricksList.map(async (brick) => {
        try {
          const schema = await api.bricksGetSchema(brick.brick_type);
          return { name: brick.name, brickType: brick.brick_type, schema };
        } catch (err) {
          console.warn(`Failed to load schema for ${brick.brick_type}:`, err);
          return { name: brick.name, brickType: brick.brick_type, schema: null };
        }
      });

      const schemas = await Promise.all(schemaPromises);
      const schemaMap: Record<string, Record<string, any>> = {};
      schemas.forEach(({ name, brickType, schema }) => {
        if (schema) {
          schemaMap[name] = schema;
          schemaMap[brickType] = schema;
        }
      });
      setBrickSchemas(schemaMap);
    } catch (err) {
      console.error('Failed to load bricks:', err);
      setError('Failed to load available bricks. Please refresh the page.');
    } finally {
      setLoading(false);
    }
  };

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

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">Loading bricks...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-lg p-4">
        <p className="text-sm text-red-800">{error}</p>
        <button
          onClick={loadBricks}
          className="mt-2 text-sm text-red-600 hover:text-red-800 underline"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="flex h-full border border-gray-200 rounded-lg overflow-hidden bg-white">
      <BrickPalette availableBricks={availableBricks} onAddBrick={handleAddBrick} />
      <FlowCanvas
        bricks={bricks}
        availableBricks={availableBricks}
        brickSchemas={brickSchemas}
        onBricksChange={onBricksChange}
      />
    </div>
  );
}
