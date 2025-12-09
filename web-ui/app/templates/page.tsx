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
            condition_field: 'total',
            condition: '> 1000',
            true_value: { status: 'VIP' },
            false_value: { status: 'Standard' },
            output_field: 'status',
          },
        },
      ],
    },
  },
  {
    name: 'HubSpot Contact Sync',
    description: 'Sync contacts to HubSpot with data mapping',
    category: 'CRM Integration',
    flow: {
      name: 'HubSpot Contact Sync',
      description: 'Map and sync contact data to HubSpot',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'contact.first_name', target_path: 'properties.firstname' },
              { source_path: 'contact.last_name', target_path: 'properties.lastname' },
              { source_path: 'contact.email', target_path: 'properties.email' },
              { source_path: 'contact.phone', target_path: 'properties.phone' },
            ],
          },
        },
        {
          brick_type: 'hubspot',
          config: {
            api_key: '',
            operation: 'get_contacts',
          },
        },
      ],
    },
  },
  {
    name: 'HubSpot Deal Update',
    description: 'Update existing deals in HubSpot based on conditions',
    category: 'CRM Integration',
    flow: {
      name: 'HubSpot Deal Update',
      description: 'Conditionally update deals in HubSpot',
      bricks: [
        {
          brick_type: 'conditional',
          config: {
            condition_field: 'deal.amount',
            condition: '> 50000',
            true_value: { properties: { dealstage: 'closed-won' } },
            false_value: { properties: { dealstage: 'qualified' } },
            output_field: 'deal_update',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'deal.id', target_path: 'id' },
              { source_path: 'deal_update.properties', target_path: 'properties' },
            ],
          },
        },
        {
          brick_type: 'hubspot',
          config: {
            api_key: '',
            operation: 'update_deal',
          },
        },
      ],
    },
  },
  {
    name: 'HubSpot Deal Analytics',
    description: 'Fetch and analyze deals from HubSpot',
    category: 'CRM Integration',
    flow: {
      name: 'HubSpot Deal Analytics',
      description: 'Retrieve deals for analysis',
      bricks: [
        {
          brick_type: 'hubspot',
          config: {
            api_key: '',
            operation: 'get_deals',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Analyze these deals and provide insights: {{deals}}',
          },
        },
      ],
    },
  },
  {
    name: 'Notion Task Tracker',
    description: 'Create task pages in Notion from form submissions',
    category: 'Documentation',
    flow: {
      name: 'Notion Task Tracker',
      description: 'Automatically create task pages in Notion',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['task.title', 'task.description'],
            separator: ' - ',
            output_field: 'page_title',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'page_title', target_path: 'title' },
              { source_path: 'task.priority', target_path: 'priority' },
              { source_path: 'task.due_date', target_path: 'due_date' },
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
    name: 'Notion Meeting Notes',
    description: 'Create meeting notes pages in Notion with AI summary',
    category: 'Documentation',
    flow: {
      name: 'Notion Meeting Notes',
      description: 'Generate AI summaries and create Notion pages',
      bricks: [
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Summarize these meeting notes: {{meeting_notes}}',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'meeting.title', target_path: 'title' },
              { source_path: 'openai_response', target_path: 'summary' },
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
    name: 'Notion Page Updater',
    description: 'Update existing Notion pages with new content',
    category: 'Documentation',
    flow: {
      name: 'Notion Page Updater',
      description: 'Update Notion pages with mapped data',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'page.id', target_path: 'page_id' },
              { source_path: 'page.content', target_path: 'properties.content' },
            ],
          },
        },
        {
          brick_type: 'notion',
          config: {
            api_key: '',
            operation: 'update_page',
          },
        },
      ],
    },
  },
  {
    name: 'Odoo Invoice Creation',
    description: 'Create invoices in Odoo ERP system',
    category: 'ERP Integration',
    flow: {
      name: 'Odoo Invoice Creation',
      description: 'Generate invoices in Odoo',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'order.customer', target_path: 'partner_id' },
              { source_path: 'order.amount', target_path: 'amount_total' },
              { source_path: 'order.items', target_path: 'invoice_line_ids' },
            ],
          },
        },
        {
          brick_type: 'odoo',
          config: {
            url: '',
            database: '',
            username: '',
            password: '',
            operation: 'create_invoice',
          },
        },
      ],
    },
  },
  {
    name: 'Odoo Product Sync',
    description: 'Sync product data from external source to Odoo',
    category: 'ERP Integration',
    flow: {
      name: 'Odoo Product Sync',
      description: 'Retrieve and sync products',
      bricks: [
        {
          brick_type: 'odoo',
          config: {
            url: '',
            database: '',
            username: '',
            password: '',
            operation: 'get_products',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'external_product.name', target_path: 'name' },
              { source_path: 'external_product.price', target_path: 'list_price' },
            ],
          },
        },
      ],
    },
  },
  {
    name: 'Odoo Invoice Retrieval',
    description: 'Fetch invoices from Odoo for processing',
    category: 'ERP Integration',
    flow: {
      name: 'Odoo Invoice Retrieval',
      description: 'Get invoices from Odoo system',
      bricks: [
        {
          brick_type: 'odoo',
          config: {
            url: '',
            database: '',
            username: '',
            password: '',
            operation: 'get_invoices',
          },
        },
        {
          brick_type: 'conditional',
          config: {
            condition_field: 'invoice.state',
            condition: "== 'draft'",
            true_value: { status: 'needs_review' },
            false_value: { status: 'processed' },
            output_field: 'processing_status',
          },
        },
      ],
    },
  },
  {
    name: 'AI Content Generator',
    description: 'Generate content using OpenAI with custom prompts',
    category: 'AI Processing',
    flow: {
      name: 'AI Content Generator',
      description: 'Create AI-generated content',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['topic', 'keywords', 'tone'],
            separator: ', ',
            output_field: 'prompt_context',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-4',
            prompt_template: 'Write content about {{prompt_context}}',
            temperature: 0.8,
            max_tokens: 2000,
          },
        },
      ],
    },
  },
  {
    name: 'AI Email Responder',
    description: 'Generate email responses using AI',
    category: 'AI Processing',
    flow: {
      name: 'AI Email Responder',
      description: 'Auto-generate email replies',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'email.subject', target_path: 'subject' },
              { source_path: 'email.body', target_path: 'original_message' },
            ],
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Write a professional response to this email. Subject: {{subject}}, Message: {{original_message}}',
          },
        },
      ],
    },
  },
  {
    name: 'AI Data Enrichment',
    description: 'Enrich customer data with AI insights',
    category: 'AI Processing',
    flow: {
      name: 'AI Data Enrichment',
      description: 'Add AI-generated insights to data',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['customer.name', 'customer.email', 'customer.purchase_history'],
            separator: ' | ',
            output_field: 'customer_summary',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Analyze this customer data and provide insights: {{customer_summary}}',
          },
        },
      ],
    },
  },
  {
    name: 'NVIDIA Speech to Text',
    description: 'Convert audio to text using NVIDIA ASR',
    category: 'AI Processing',
    flow: {
      name: 'NVIDIA Speech to Text',
      description: 'Transcribe audio files',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'audio.file_url', target_path: 'audio_url' },
            ],
          },
        },
        {
          brick_type: 'nvidia',
          config: {
            api_key: '',
            endpoint: 'asr',
          },
        },
      ],
    },
  },
  {
    name: 'NVIDIA OCR Document Processing',
    description: 'Extract text from images using NVIDIA OCR',
    category: 'AI Processing',
    flow: {
      name: 'NVIDIA OCR Document Processing',
      description: 'Extract text from document images',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'document.image_url', target_path: 'image_url' },
            ],
          },
        },
        {
          brick_type: 'nvidia',
          config: {
            api_key: '',
            endpoint: 'ocr',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Process and structure this extracted text: {{text}}',
          },
        },
      ],
    },
  },
  {
    name: 'NVIDIA Text Generation',
    description: 'Generate text using NVIDIA models',
    category: 'AI Processing',
    flow: {
      name: 'NVIDIA Text Generation',
      description: 'Create text with NVIDIA AI',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'request.prompt', target_path: 'prompt' },
            ],
          },
        },
        {
          brick_type: 'nvidia',
          config: {
            api_key: '',
            endpoint: 'text_generation',
            model: 'nvidia/meta/llama-3-8b-instruct',
          },
        },
      ],
    },
  },
  {
    name: 'n8n Webhook Trigger',
    description: 'Trigger n8n workflows via webhook',
    category: 'Webhook Integration',
    flow: {
      name: 'n8n Webhook Trigger',
      description: 'Send data to n8n webhook',
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
            webhook_url: '',
            method: 'POST',
          },
        },
      ],
    },
  },
  {
    name: 'n8n Data Pipeline',
    description: 'Process and forward data through n8n',
    category: 'Webhook Integration',
    flow: {
      name: 'n8n Data Pipeline',
      description: 'Transform and send data to n8n',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['data.field1', 'data.field2'],
            separator: ',',
            output_field: 'combined_data',
          },
        },
        {
          brick_type: 'n8n',
          config: {
            webhook_url: '',
            method: 'POST',
          },
        },
      ],
    },
  },
  {
    name: 'Customer Onboarding Flow',
    description: 'Complete customer onboarding with multiple integrations',
    category: 'Workflow Automation',
    flow: {
      name: 'Customer Onboarding Flow',
      description: 'Automated customer onboarding',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['customer.first_name', 'customer.last_name'],
            separator: ' ',
            output_field: 'full_name',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'full_name', target_path: 'properties.dealname' },
              { source_path: 'customer.email', target_path: 'properties.email' },
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
    name: 'Lead Qualification',
    description: 'Qualify leads with AI and create CRM records',
    category: 'Workflow Automation',
    flow: {
      name: 'Lead Qualification',
      description: 'AI-powered lead qualification',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['lead.company', 'lead.industry', 'lead.budget'],
            separator: ' | ',
            output_field: 'lead_summary',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Qualify this lead: {{lead_summary}}. Return JSON with qualification_score and recommendation.',
          },
        },
        {
          brick_type: 'conditional',
          config: {
            condition_field: 'qualification_score',
            condition: '> 70',
            true_value: { status: 'qualified', priority: 'high' },
            false_value: { status: 'needs_review', priority: 'low' },
            output_field: 'qualification_result',
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
    name: 'Invoice Processing Pipeline',
    description: 'Process invoices with OCR and ERP integration',
    category: 'Workflow Automation',
    flow: {
      name: 'Invoice Processing Pipeline',
      description: 'Extract invoice data and sync to ERP',
      bricks: [
        {
          brick_type: 'nvidia',
          config: {
            api_key: '',
            endpoint: 'ocr',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Extract structured data from this invoice text: {{text}}. Return JSON with amount, date, vendor.',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'extracted.amount', target_path: 'amount_total' },
              { source_path: 'extracted.vendor', target_path: 'partner_id' },
            ],
          },
        },
        {
          brick_type: 'odoo',
          config: {
            url: '',
            database: '',
            username: '',
            password: '',
            operation: 'create_invoice',
          },
        },
      ],
    },
  },
  {
    name: 'Support Ticket Automation',
    description: 'Create support tickets with AI categorization',
    category: 'Workflow Automation',
    flow: {
      name: 'Support Ticket Automation',
      description: 'Auto-categorize and route support tickets',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['ticket.subject', 'ticket.description'],
            separator: ' - ',
            output_field: 'ticket_content',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Categorize this support ticket: {{ticket_content}}. Return category and priority.',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'ticket_content', target_path: 'title' },
              { source_path: 'category', target_path: 'category' },
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
    name: 'Sales Pipeline Sync',
    description: 'Sync sales data between CRM and documentation',
    category: 'Workflow Automation',
    flow: {
      name: 'Sales Pipeline Sync',
      description: 'Keep CRM and docs in sync',
      bricks: [
        {
          brick_type: 'hubspot',
          config: {
            api_key: '',
            operation: 'get_deals',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'deal.properties.dealname', target_path: 'title' },
              { source_path: 'deal.properties.amount', target_path: 'amount' },
              { source_path: 'deal.properties.dealstage', target_path: 'stage' },
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
    name: 'Product Catalog Sync',
    description: 'Sync products between systems',
    category: 'Workflow Automation',
    flow: {
      name: 'Product Catalog Sync',
      description: 'Synchronize product data',
      bricks: [
        {
          brick_type: 'odoo',
          config: {
            url: '',
            database: '',
            username: '',
            password: '',
            operation: 'get_products',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'product.name', target_path: 'title' },
              { source_path: 'product.description', target_path: 'content' },
              { source_path: 'product.price', target_path: 'price' },
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
    name: 'Customer Feedback Analysis',
    description: 'Analyze customer feedback with AI',
    category: 'Data Processing',
    flow: {
      name: 'Customer Feedback Analysis',
      description: 'Process and analyze feedback',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['feedback.rating', 'feedback.comments'],
            separator: ': ',
            output_field: 'feedback_text',
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Analyze this customer feedback and extract sentiment, key themes, and action items: {{feedback_text}}',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'feedback.customer_id', target_path: 'customer_id' },
              { source_path: 'analysis', target_path: 'analysis_result' },
            ],
          },
        },
      ],
    },
  },
  {
    name: 'Data Transformation Pipeline',
    description: 'Transform and route data through multiple steps',
    category: 'Data Processing',
    flow: {
      name: 'Data Transformation Pipeline',
      description: 'Multi-step data transformation',
      bricks: [
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'source.field1', target_path: 'target.field1' },
              { source_path: 'source.field2', target_path: 'target.field2' },
            ],
          },
        },
        {
          brick_type: 'combine_text',
          config: {
            fields: ['target.field1', 'target.field2'],
            separator: ' | ',
            output_field: 'combined',
          },
        },
        {
          brick_type: 'conditional',
          config: {
            condition_field: 'target.field1',
            condition: "!= ''",
            true_value: { status: 'valid' },
            false_value: { status: 'invalid' },
            output_field: 'validation_status',
          },
        },
      ],
    },
  },
  {
    name: 'Smart Form Submission',
    description: 'Process form data with validation and routing',
    category: 'Data Processing',
    flow: {
      name: 'Smart Form Submission',
      description: 'Validate and route form submissions',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['form.first_name', 'form.last_name'],
            separator: ' ',
            output_field: 'full_name',
          },
        },
        {
          brick_type: 'conditional',
          config: {
            condition_field: 'form.amount',
            condition: '> 1000',
            true_value: { route: 'high_value', priority: 'high' },
            false_value: { route: 'standard', priority: 'normal' },
            output_field: 'routing',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'full_name', target_path: 'properties.dealname' },
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
    name: 'Meeting Transcription & Summary',
    description: 'Transcribe audio and generate meeting summaries',
    category: 'AI Processing',
    flow: {
      name: 'Meeting Transcription & Summary',
      description: 'Convert audio to text and summarize',
      bricks: [
        {
          brick_type: 'nvidia',
          config: {
            api_key: '',
            endpoint: 'asr',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'text', target_path: 'transcription' },
            ],
          },
        },
        {
          brick_type: 'openai',
          config: {
            api_key: '',
            model_name: 'gpt-3.5-turbo',
            prompt_template: 'Summarize this meeting transcription with key points and action items: {{transcription}}',
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
    name: 'Multi-Channel Notification',
    description: 'Send notifications through multiple channels',
    category: 'Workflow Automation',
    flow: {
      name: 'Multi-Channel Notification',
      description: 'Notify via multiple channels',
      bricks: [
        {
          brick_type: 'combine_text',
          config: {
            fields: ['notification.title', 'notification.message'],
            separator: ': ',
            output_field: 'notification_text',
          },
        },
        {
          brick_type: 'field_mapping',
          config: {
            mappings: [
              { source_path: 'notification_text', target_path: 'message' },
            ],
          },
        },
        {
          brick_type: 'n8n',
          config: {
            webhook_url: '',
            method: 'POST',
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
