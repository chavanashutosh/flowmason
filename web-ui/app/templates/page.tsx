'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Card, Button, Badge, Spinner } from 'flowbite-react';
import { Sparkles, Copy, Loader } from 'lucide-react';
import { api } from '@/lib/api';

interface Template {
  name: string;
  description: string;
  category: string;
  flow: {
    name: string;
    description: string;
    bricks: Array<{
      brick_type: string;
      config: any;
    }>;
  };
}

const templates: Template[] = [
  {
    name: 'Customer Data Processing',
    description: 'Map customer data and process with AI',
    category: 'Data Processing',
    flow: {
      name: 'Customer Data Processing',
      description: 'Process customer data with field mapping and AI',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'user.name', target_path: 'customer_name' },
              { source_path: 'user.email', target_path: 'customer_email' },
            ],
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Process customer: {{customer_name}}',
          },
        },
      ],
    },
  },
  {
    name: 'HubSpot Deal Creation',
    description: 'Create deals in HubSpot from form submissions',
    category: 'CRM Integration',
    flow: {
      name: 'HubSpot Deal Creation',
      description: 'Automatically create deals in HubSpot',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'form.deal_name', target_path: 'properties.dealname' },
              { source_path: 'form.amount', target_path: 'properties.amount' },
            ],
          },
        },
        {
          brick_type: 'hubspot',
          config: {
            api_key: '',
            operation: 'create_deal',
          },
        },
      ],
    },
  },
  {
    name: 'Notion Page Creator',
    description: 'Create pages in Notion from webhooks',
    category: 'Documentation',
    flow: {
      name: 'Notion Page Creator',
      description: 'Create Notion pages automatically',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'webhook.title', target_path: 'title' },
              { source_path: 'webhook.content', target_path: 'content' },
            ],
          },
        },
        {
          brick_type: 'notion',
          config: {
            api_key: '',
            database_id: '',
            operation: 'create_page',
          },
        },
      ],
    },
  },
  {
    name: 'Text Combination & Processing',
    description: 'Combine text fields and process with conditional logic',
    category: 'Text Processing',
    flow: {
      name: 'Text Combination & Processing',
      description: 'Combine and process text fields',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['first_name', 'last_name'],
            separator: ' ',
            output_field: 'full_name',
          },
        },
        {
          brick_type: 'conditional',
          config: {
            condition: 'total > 1000',
            true_value: { status: 'VIP' },
            false_value: { status: 'Standard' },
          },
        },
      ],
    },
  },
];

export default function TemplatesPage() {
  const router = useRouter();
  const [loading, setLoading] = useState<string | null>(null);

  const useTemplate = async (template: Template) => {
    setLoading(template.name);
    try {
      const flow = await api.flows.create(template.flow);
      router.push(`/flows/${flow.id}`);
    } catch (error: any) {
      console.error('Failed to create flow from template:', error);
      alert(`Failed to create flow: ${error.message || 'Unknown error'}`);
    } finally {
      setLoading(null);
    }
  };

  const categories = Array.from(new Set(templates.map(t => t.category)));

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <Sparkles className="text-purple-600 dark:text-purple-400" size={32} />
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Flow Templates</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            Start with a pre-built template and customize it to your needs
          </p>
        </div>
      </div>

      {categories.map((category) => (
        <div key={category}>
          <h2 className="text-2xl font-semibold text-gray-900 dark:text-white mb-4">{category}</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {templates
              .filter(t => t.category === category)
              .map((template) => (
                <Card key={template.name} className="h-full">
                  <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                    {template.name}
                  </h3>
                  <p className="text-gray-600 dark:text-gray-400 mb-4">{template.description}</p>
                  
                  <div className="mb-4">
                    <p className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Bricks:</p>
                    <div className="flex flex-wrap gap-2">
                      {template.flow.bricks.map((brick, idx) => (
                        <Badge key={idx} color="info" className="capitalize">
                          {brick.brick_type.replace(/_/g, ' ')}
                        </Badge>
                      ))}
                    </div>
                  </div>

                  <Button
                    gradientDuoTone="purpleToBlue"
                    className="w-full"
                    icon={loading === template.name ? undefined : Copy}
                    onClick={() => useTemplate(template)}
                    disabled={loading === template.name}
                  >
                    {loading === template.name ? (
                      <>
                        <Spinner size="sm" className="mr-2" />
                        Creating...
                      </>
                    ) : (
                      'Use Template'
                    )}
                  </Button>
                </Card>
              ))}
          </div>
        </div>
      ))}
    </div>
  );
}
