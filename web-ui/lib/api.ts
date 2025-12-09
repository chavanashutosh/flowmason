const API_BASE_URL = '/api/v1';

export interface Flow {
  id: string;
  name: string;
  description?: string;
  bricks: Array<{
    brick_type: string;
    config: any;
  }>;
  active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateFlowRequest {
  name: string;
  description?: string;
  bricks: Array<{
    brick_type: string;
    config: any;
  }>;
}

export const api = {
  flows: {
    list: async (): Promise<Flow[]> => {
      const response = await fetch(`${API_BASE_URL}/flows`);
      return response.json();
    },
    get: async (id: string): Promise<Flow> => {
      const response = await fetch(`${API_BASE_URL}/flows/${id}`);
      return response.json();
    },
    create: async (flow: CreateFlowRequest): Promise<Flow> => {
      const response = await fetch(`${API_BASE_URL}/flows`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(flow),
      });
      return response.json();
    },
    update: async (id: string, flow: Partial<CreateFlowRequest>): Promise<Flow> => {
      const response = await fetch(`${API_BASE_URL}/flows/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(flow),
      });
      return response.json();
    },
    delete: async (id: string): Promise<void> => {
      await fetch(`${API_BASE_URL}/flows/${id}`, {
        method: 'DELETE',
      });
    },
  },
  executions: {
    execute: async (flowId: string, inputPayload: any) => {
      const response = await fetch(`${API_BASE_URL}/executions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          flow_id: flowId,
          input_payload: inputPayload,
        }),
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ message: 'Execution failed' }));
        throw new Error(error.message || 'Execution failed');
      }
      return response.json();
    },
    list: async (): Promise<any[]> => {
      const response = await fetch(`${API_BASE_URL}/executions`);
      if (!response.ok) throw new Error('Failed to fetch executions');
      return response.json();
    },
    get: async (executionId: string): Promise<any> => {
      const response = await fetch(`${API_BASE_URL}/executions/${executionId}`);
      if (!response.ok) throw new Error('Failed to fetch execution');
      return response.json();
    },
    listByFlow: async (flowId: string): Promise<any[]> => {
      const response = await fetch(`${API_BASE_URL}/executions/flow/${flowId}`);
      if (!response.ok) throw new Error('Failed to fetch flow executions');
      return response.json();
    },
  },
  usage: {
    getStats: async () => {
      const response = await fetch(`${API_BASE_URL}/usage/stats`);
      return response.json();
    },
  },
  bricks: {
    list: async () => {
      const response = await fetch(`${API_BASE_URL}/bricks`);
      return response.json();
    },
    getSchema: async (brickType: string) => {
      const response = await fetch(`${API_BASE_URL}/bricks/${brickType}/schema`);
      return response.json();
    },
  },
  scheduler: {
    scheduleFlow: async (flowId: string, cronExpression: string) => {
      const response = await fetch(`${API_BASE_URL}/scheduler/flows`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ flow_id: flowId, cron_expression: cronExpression }),
      });
      if (!response.ok) throw new Error('Failed to schedule flow');
      return response.json();
    },
    listScheduledFlows: async () => {
      const response = await fetch(`${API_BASE_URL}/scheduler/flows`);
      if (!response.ok) throw new Error('Failed to fetch scheduled flows');
      return response.json();
    },
    unscheduleFlow: async (flowId: string) => {
      const response = await fetch(`${API_BASE_URL}/scheduler/flows/${flowId}`, {
        method: 'DELETE',
      });
      if (!response.ok) throw new Error('Failed to unschedule flow');
    },
  },
};

