import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { api } from '../api/client';
import { NotionIcon, HubSpotIcon, OpenAIIcon, NvidiaIcon, OdooIcon, N8nIcon, Workflow, LinkIcon, Shuffle } from '../components/ui/icons';

interface Template {
  name: string;
  description: string;
  category: string;
  icon: React.ReactNode;
  flow: {
    name: string;
    description: string;
    bricks: any[];
  };
}

export default function Templates() {
  const navigate = useNavigate();
  const [loadingTemplate, setLoadingTemplate] = useState<string | null>(null);
  const [selectedCategory, setSelectedCategory] = useState<string>('All');

  // Helper function to get icon based on bricks
  const getTemplateIcon = (bricks: any[]): React.ReactNode => {
    const brickTypes = bricks.map((b) => b.brick_type);
    if (brickTypes.includes('notion')) return <NotionIcon size={32} className="text-gray-700" />;
    if (brickTypes.includes('hubspot')) return <HubSpotIcon size={32} className="text-gray-700" />;
    if (brickTypes.includes('openai')) return <OpenAIIcon size={32} className="text-gray-700" />;
    if (brickTypes.includes('nvidia')) return <NvidiaIcon size={32} className="text-gray-700" />;
    if (brickTypes.includes('odoo')) return <OdooIcon size={32} className="text-gray-700" />;
    if (brickTypes.includes('n8n')) return <N8nIcon size={32} className="text-gray-700" />;
    return <Workflow size={32} className="text-gray-700" />;
  };

  const templates: Template[] = [
    // Workflow Templates (5)
    {
      name: 'AI Content to Notion',
      description: 'Generate content with AI and save to Notion',
      category: 'Workflows',
      icon: getTemplateIcon([
        { brick_type: 'openai' },
        { brick_type: 'field_mapping' },
        { brick_type: 'notion' },
      ]),
      flow: {
        name: 'AI Content to Notion',
        description: 'Generate blog post content using AI and create Notion pages',
        bricks: [
          {
            brick_type: 'openai',
            config: {
              api_key: 'your-openai-api-key',
              model_name: 'gpt-4',
              prompt_template: 'Write a blog post about {{topic}} with the following requirements:\n- Length: {{length}} words\n- Tone: {{tone}}\n- Include: {{requirements}}',
              temperature: 0.8,
              max_tokens: 2000,
            },
          },
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'topic', target_path: 'title' },
                { source_path: 'content', target_path: 'content' },
              ],
            },
          },
          {
            brick_type: 'notion',
            config: {
              api_key: 'your-notion-api-key',
              database_id: 'your-database-id',
              operation: 'create_page',
            },
          },
        ],
      },
    },
    {
      name: 'Customer Data Enrichment',
      description: 'Enrich customer data with AI and sync to CRM',
      category: 'Workflows',
      icon: getTemplateIcon([
        { brick_type: 'field_mapping' },
        { brick_type: 'combine_text' },
        { brick_type: 'openai' },
        { brick_type: 'hubspot' },
      ]),
      flow: {
        name: 'Customer Data Enrichment',
        description: 'Process customer data, enrich with AI, and create CRM records',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'form.first_name', target_path: 'first_name' },
                { source_path: 'form.last_name', target_path: 'last_name' },
                { source_path: 'form.email', target_path: 'email' },
              ],
            },
          },
          {
            brick_type: 'combine_text',
            config: {
              fields: ['first_name', 'last_name'],
              separator: ' ',
              output_field: 'full_name',
            },
          },
          {
            brick_type: 'openai',
            config: {
              api_key: 'your-openai-api-key',
              model_name: 'gpt-3.5-turbo',
              prompt_template: 'Generate a professional company description for: {{full_name}} working in {{industry}}',
              temperature: 0.7,
              max_tokens: 200,
            },
          },
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'full_name', target_path: 'properties.firstname' },
                { source_path: 'email', target_path: 'properties.email' },
                { source_path: 'content', target_path: 'properties.company_description' },
              ],
            },
          },
          {
            brick_type: 'hubspot',
            config: {
              api_key: 'your-hubspot-api-key',
              operation: 'create_deal',
            },
          },
        ],
      },
    },
    {
      name: 'HubSpot to Notion Sync',
      description: 'Sync HubSpot deals to Notion database',
      category: 'Workflows',
      icon: getTemplateIcon([
        { brick_type: 'hubspot' },
        { brick_type: 'field_mapping' },
        { brick_type: 'notion' },
      ]),
      flow: {
        name: 'HubSpot to Notion Sync',
        description: 'Automatically sync HubSpot deals to a Notion database',
        bricks: [
          {
            brick_type: 'hubspot',
            config: {
              api_key: 'your-hubspot-api-key',
              operation: 'get_deals',
            },
          },
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'results[].properties.dealname', target_path: 'title' },
                { source_path: 'results[].properties.amount', target_path: 'amount' },
                { source_path: 'results[].properties.dealstage', target_path: 'status' },
              ],
            },
          },
          {
            brick_type: 'notion',
            config: {
              api_key: 'your-notion-api-key',
              database_id: 'your-database-id',
              operation: 'create_page',
            },
          },
        ],
      },
    },
    {
      name: 'Webhook to Odoo Invoice',
      description: 'Create invoices in Odoo from webhook data',
      category: 'Workflows',
      icon: getTemplateIcon([
        { brick_type: 'field_mapping' },
        { brick_type: 'conditional' },
        { brick_type: 'odoo' },
      ]),
      flow: {
        name: 'Webhook to Odoo Invoice',
        description: 'Process webhook data and create invoices in Odoo ERP',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'webhook.customer_id', target_path: 'partner_id' },
                { source_path: 'webhook.order_date', target_path: 'invoice_date' },
                { source_path: 'webhook.items', target_path: 'invoice_line_ids' },
              ],
            },
          },
          {
            brick_type: 'conditional',
            config: {
              condition_field: 'webhook.status',
              condition: "== 'paid'",
              true_value: 'posted',
              false_value: 'draft',
              output_field: 'state',
            },
          },
          {
            brick_type: 'odoo',
            config: {
              url: 'https://your-odoo-instance.com',
              database: 'your-database-name',
              username: 'your-username',
              password: 'your-password',
              operation: 'create_invoice',
            },
          },
        ],
      },
    },
    {
      name: 'n8n Webhook Pipeline',
      description: 'Process data through n8n and sync to HubSpot',
      category: 'Workflows',
      icon: getTemplateIcon([
        { brick_type: 'field_mapping' },
        { brick_type: 'n8n' },
        { brick_type: 'hubspot' },
      ]),
      flow: {
        name: 'n8n Webhook Pipeline',
        description: 'Send data to n8n for processing and then sync results to HubSpot',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'event.type', target_path: 'event_type' },
                { source_path: 'event.data', target_path: 'payload' },
              ],
            },
          },
          {
            brick_type: 'n8n',
            config: {
              webhook_url: 'https://your-n8n-instance.com/webhook/process-data',
              method: 'POST',
            },
          },
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'processed_data.customer_name', target_path: 'properties.dealname' },
                { source_path: 'processed_data.amount', target_path: 'properties.amount' },
              ],
            },
          },
          {
            brick_type: 'hubspot',
            config: {
              api_key: 'your-hubspot-api-key',
              operation: 'create_deal',
            },
          },
        ],
      },
    },
    // Integration Templates - AI Services (3)
    {
      name: 'OpenAI Text Generation',
      description: 'Generate text using GPT models',
      category: 'AI Services',
      icon: getTemplateIcon([{ brick_type: 'openai' }]),
      flow: {
        name: 'OpenAI Text Generation',
        description: 'Generate text using GPT models',
        bricks: [
          {
            brick_type: 'openai',
            config: {
              api_key: 'your-openai-api-key',
              model_name: 'gpt-3.5-turbo',
              prompt_template: 'Write a professional email about: {{topic}}',
              temperature: 0.7,
              max_tokens: 1000,
            },
          },
        ],
      },
    },
    {
      name: 'OpenAI Customer Support',
      description: 'Generate customer support responses',
      category: 'AI Services',
      icon: getTemplateIcon([{ brick_type: 'openai' }]),
      flow: {
        name: 'OpenAI Customer Support',
        description: 'Generate customer support responses',
        bricks: [
          {
            brick_type: 'openai',
            config: {
              api_key: 'your-openai-api-key',
              model_name: 'gpt-4',
              prompt_template: 'Customer inquiry: {{inquiry}}\n\nGenerate a helpful response:',
              temperature: 0.5,
              max_tokens: 500,
            },
          },
        ],
      },
    },
    {
      name: 'OpenAI Content Summarization',
      description: 'Summarize long text content',
      category: 'AI Services',
      icon: getTemplateIcon([{ brick_type: 'openai' }]),
      flow: {
        name: 'OpenAI Content Summarization',
        description: 'Summarize long text content',
        bricks: [
          {
            brick_type: 'openai',
            config: {
              api_key: 'your-openai-api-key',
              model_name: 'gpt-3.5-turbo',
              prompt_template: 'Summarize the following text in 3 sentences:\n\n{{content}}',
              temperature: 0.3,
              max_tokens: 200,
            },
          },
        ],
      },
    },
    {
      name: 'NVIDIA ASR Processing',
      description: 'Convert speech to text using NVIDIA',
      category: 'AI Services',
      icon: getTemplateIcon([{ brick_type: 'nvidia' }]),
      flow: {
        name: 'NVIDIA ASR Processing',
        description: 'Convert speech to text using NVIDIA ASR',
        bricks: [
          {
            brick_type: 'nvidia',
            config: {
              api_key: 'your-nvidia-api-key',
              service: 'asr',
              model: 'nvidia/nemospeechtotext',
            },
          },
        ],
      },
    },
    // Integration Templates - CRM (3)
    {
      name: 'HubSpot Deal Creation',
      description: 'Create deals in HubSpot from form submissions',
      category: 'CRM Integration',
      icon: getTemplateIcon([
        { brick_type: 'field_mapping' },
        { brick_type: 'hubspot' },
      ]),
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
                { source_path: 'form.stage', target_path: 'properties.dealstage' },
              ],
            },
          },
          {
            brick_type: 'hubspot',
            config: {
              api_key: 'your-hubspot-api-key',
              operation: 'create_deal',
            },
          },
        ],
      },
    },
    {
      name: 'Notion Page Creation',
      description: 'Create pages in Notion database',
      category: 'Content Management',
      icon: getTemplateIcon([{ brick_type: 'notion' }]),
      flow: {
        name: 'Notion Page Creation',
        description: 'Create pages in a Notion database',
        bricks: [
          {
            brick_type: 'notion',
            config: {
              api_key: 'your-notion-api-key',
              database_id: 'your-database-id',
              operation: 'create_page',
            },
          },
        ],
      },
    },
    {
      name: 'Odoo Invoice Creation',
      description: 'Create invoices in Odoo ERP',
      category: 'CRM Integration',
      icon: getTemplateIcon([{ brick_type: 'odoo' }]),
      flow: {
        name: 'Odoo Invoice Creation',
        description: 'Create invoices in Odoo ERP',
        bricks: [
          {
            brick_type: 'odoo',
            config: {
              url: 'https://your-odoo-instance.com',
              database: 'your-database-name',
              username: 'your-username',
              password: 'your-password',
              operation: 'create_invoice',
            },
          },
        ],
      },
    },
    // Data Processing Templates (4)
    {
      name: 'Field Mapping',
      description: 'Map fields between different data structures',
      category: 'Data Processing',
      icon: <LinkIcon size={32} className="text-gray-700" />,
      flow: {
        name: 'Field Mapping',
        description: 'Map fields from one structure to another',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'user.name', target_path: 'customer_name' },
                { source_path: 'user.email', target_path: 'customer_email' },
                { source_path: 'user.phone', target_path: 'contact_phone' },
              ],
            },
          },
        ],
      },
    },
    {
      name: 'Combine Text Fields',
      description: 'Combine multiple text fields into one',
      category: 'Data Processing',
      icon: <Workflow size={32} className="text-gray-700" />,
      flow: {
        name: 'Combine Text Fields',
        description: 'Combine first and last name into full name',
        bricks: [
          {
            brick_type: 'combine_text',
            config: {
              fields: ['first_name', 'last_name'],
              separator: ' ',
              output_field: 'full_name',
            },
          },
        ],
      },
    },
    {
      name: 'Conditional Logic',
      description: 'Apply conditional logic based on field values',
      category: 'Data Processing',
      icon: <Shuffle size={32} className="text-gray-700" />,
      flow: {
        name: 'Conditional Logic',
        description: 'Set VIP status based on purchase amount',
        bricks: [
          {
            brick_type: 'conditional',
            config: {
              condition_field: 'total',
              condition: '> 1000',
              true_value: { status: 'VIP', discount: 0.15 },
              false_value: { status: 'Standard', discount: 0.0 },
              output_field: 'customer_tier',
            },
          },
        ],
      },
    },
    {
      name: 'NVIDIA OCR Processing',
      description: 'Extract text from images using NVIDIA',
      category: 'Data Processing',
      icon: getTemplateIcon([{ brick_type: 'nvidia' }]),
      flow: {
        name: 'NVIDIA OCR Processing',
        description: 'Extract text from images using NVIDIA OCR',
        bricks: [
          {
            brick_type: 'nvidia',
            config: {
              api_key: 'your-nvidia-api-key',
              service: 'ocr',
              model: 'nvidia/nv-ocr',
            },
          },
        ],
      },
    },
    // Advanced Templates (4+)
    {
      name: 'Email to Notion',
      description: 'Process emails and create Notion pages',
      category: 'Automation',
      icon: getTemplateIcon([
        { brick_type: 'field_mapping' },
        { brick_type: 'notion' },
      ]),
      flow: {
        name: 'Email to Notion',
        description: 'Process email content and create Notion pages',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'email.subject', target_path: 'title' },
                { source_path: 'email.body', target_path: 'content' },
                { source_path: 'email.from', target_path: 'sender' },
              ],
            },
          },
          {
            brick_type: 'notion',
            config: {
              api_key: 'your-notion-api-key',
              database_id: 'your-database-id',
              operation: 'create_page',
            },
          },
        ],
      },
    },
    {
      name: 'Form to CRM',
      description: 'Process form submissions and create CRM records',
      category: 'Automation',
      icon: getTemplateIcon([
        { brick_type: 'field_mapping' },
        { brick_type: 'hubspot' },
      ]),
      flow: {
        name: 'Form to CRM',
        description: 'Process form submissions and create CRM records',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'form.name', target_path: 'properties.dealname' },
                { source_path: 'form.email', target_path: 'properties.email' },
                { source_path: 'form.company', target_path: 'properties.company' },
              ],
            },
          },
          {
            brick_type: 'hubspot',
            config: {
              api_key: 'your-hubspot-api-key',
              operation: 'create_deal',
            },
          },
        ],
      },
    },
    {
      name: 'Data Transformation Pipeline',
      description: 'Multi-step data transformation workflow',
      category: 'Data Processing',
      icon: <Workflow size={32} className="text-gray-700" />,
      flow: {
        name: 'Data Transformation Pipeline',
        description: 'Multi-step data transformation workflow',
        bricks: [
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'input.first_name', target_path: 'first_name' },
                { source_path: 'input.last_name', target_path: 'last_name' },
              ],
            },
          },
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
              condition_field: 'input.amount',
              condition: '> 1000',
              true_value: 'premium',
              false_value: 'standard',
              output_field: 'tier',
            },
          },
        ],
      },
    },
    {
      name: 'AI-Powered Content Workflow',
      description: 'Complete AI content generation workflow',
      category: 'Workflows',
      icon: getTemplateIcon([
        { brick_type: 'openai' },
        { brick_type: 'field_mapping' },
        { brick_type: 'notion' },
      ]),
      flow: {
        name: 'AI-Powered Content Workflow',
        description: 'Complete AI content generation and publishing workflow',
        bricks: [
          {
            brick_type: 'openai',
            config: {
              api_key: 'your-openai-api-key',
              model_name: 'gpt-4',
              prompt_template: 'Write a comprehensive article about {{topic}}',
              temperature: 0.8,
              max_tokens: 2000,
            },
          },
          {
            brick_type: 'field_mapping',
            config: {
              mappings: [
                { source_path: 'topic', target_path: 'title' },
                { source_path: 'content', target_path: 'content' },
              ],
            },
          },
          {
            brick_type: 'notion',
            config: {
              api_key: 'your-notion-api-key',
              database_id: 'your-database-id',
              operation: 'create_page',
            },
          },
        ],
      },
    },
  ];

  const categories = ['All', ...Array.from(new Set(templates.map((t) => t.category)))];

  const filteredTemplates =
    selectedCategory === 'All'
      ? templates
      : templates.filter((t) => t.category === selectedCategory);

  const handleUseTemplate = async (template: Template) => {
    setLoadingTemplate(template.name);
    try {
      const flow = await api.flowsCreate({
        name: template.flow.name,
        description: template.flow.description,
        bricks: template.flow.bricks.map((b) => ({
          brick_type: b.brick_type || '',
          config: b.config || {},
        })),
      });
      navigate(`/flows/${flow.id}`);
    } catch (error) {
      console.error('Failed to create flow from template:', error);
    } finally {
      setLoadingTemplate(null);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-gray-900">Templates</h1>
        <p className="text-gray-600 mt-1">Choose a template to get started</p>
      </div>

      <div className="flex flex-wrap gap-2 mb-6">
        {categories.map((category) => (
          <button
            key={category}
            onClick={() => setSelectedCategory(category)}
            className={`px-4 py-2 text-sm font-medium rounded-lg transition-colors ${
              selectedCategory === category
                ? 'bg-primary-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            {category}
          </button>
        ))}
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredTemplates.map((template) => (
          <div key={template.name} className="bg-white rounded-lg shadow p-6 hover:shadow-lg transition-shadow">
            <div className="flex items-center gap-3 mb-4">
              <div className="flex-shrink-0">{template.icon}</div>
              <div className="flex-1">
                <h3 className="text-lg font-semibold text-gray-900">{template.name}</h3>
              </div>
              <span className="px-2 py-1 text-xs font-semibold rounded bg-blue-100 text-blue-800">
                {template.category}
              </span>
            </div>
            <p className="text-gray-600 text-sm mb-4">{template.description}</p>
            <button
              className="w-full px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700 disabled:opacity-50"
              disabled={loadingTemplate !== null}
              onClick={() => handleUseTemplate(template)}
            >
              {loadingTemplate === template.name ? 'Loading...' : 'Use Template'}
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
