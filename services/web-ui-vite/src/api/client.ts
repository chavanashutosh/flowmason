import axios, { AxiosInstance, AxiosError } from 'axios';

const API_BASE_URL = '/api/v1';
const AUTH_TOKEN_KEY = 'flowmason_auth_token';

const getAuthToken = (): string | null => {
  // Prefer runtime token (e.g. set via login or dev tools), fall back to env
  if (typeof window !== 'undefined') {
    const stored = window.localStorage.getItem(AUTH_TOKEN_KEY);
    if (stored) return stored;
  }
  const envToken = import.meta.env.VITE_API_TOKEN as string | undefined;
  return envToken ?? null;
};

// Create axios instance
const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Attach auth token when available
apiClient.interceptors.request.use((config) => {
  const token = getAuthToken();
  if (token) {
    config.headers = config.headers ?? {};
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Response interceptor for error handling
apiClient.interceptors.response.use(
  (response) => response,
  (error: AxiosError) => {
    if (error.response?.status === 431) {
      throw new Error('Request headers too large. Please clear your browser cookies and try again.');
    }
    throw error;
  }
);

// Types
export interface Flow {
  id: string;
  name: string;
  description: string | null;
  bricks: BrickConfig[];
  active: boolean;
  created_at: string;
  updated_at: string;
}

export interface BrickConfig {
  brick_type: string;
  config: Record<string, any>;
}

export interface CreateFlowRequest {
  name: string;
  description?: string | null;
  bricks: BrickConfig[];
}

export interface Execution {
  execution_id: string;
  flow_id: string;
  status: string;
  input_payload: Record<string, any>;
  output_payload?: Record<string, any> | null;
  error_message?: string | null;
  created_at: string;
  completed_at?: string | null;
}

export interface ScheduledFlow {
  flow_id: string;
  cron_expression: string;
  next_run_time?: string | null;
}

export interface BrickTypeInfo {
  brick_type: string;
  name: string;
  config_schema: Record<string, any>;
}

export interface BrickListResponse {
  bricks: BrickTypeInfo[];
}

export interface User {
  id: string;
  email: string;
}

export interface AuthResponse {
  token: string;
  user_id: string;
  email: string;
}

export interface ApiKey {
  id: string;
  name: string | null;
  created_at: string;
  last_used_at: string | null;
}

export interface ApiKeyListResponse {
  keys: ApiKey[];
}

export interface CreateApiKeyRequest {
  name?: string | null;
}

export interface ApiKeyResponse {
  id: string;
  key: string;
  name: string | null;
  created_at: string;
}

// API Client
export const api = {
  // Flows
  async flowsList(): Promise<Flow[]> {
    const response = await apiClient.get<Flow[]>('/flows');
    return response.data || [];
  },

  async flowsGet(id: string): Promise<Flow> {
    const response = await apiClient.get<Flow>(`/flows/${id}`);
    return response.data;
  },

  async flowsCreate(flow: CreateFlowRequest): Promise<Flow> {
    const response = await apiClient.post<Flow>('/flows', flow);
    return response.data;
  },

  async flowsUpdate(id: string, flow: CreateFlowRequest): Promise<Flow> {
    const response = await apiClient.put<Flow>(`/flows/${id}`, flow);
    return response.data;
  },

  async flowsDelete(id: string): Promise<void> {
    await apiClient.delete(`/flows/${id}`);
  },

  // Executions
  async executionsList(): Promise<Execution[]> {
    const response = await apiClient.get<Execution[]>('/executions');
    return response.data || [];
  },

  async executionsGet(id: string): Promise<Execution> {
    const response = await apiClient.get<Execution>(`/executions/${id}`);
    return response.data;
  },

  async executionsListByFlow(flowId: string): Promise<Execution[]> {
    const response = await apiClient.get<Execution[]>(`/executions/flow/${flowId}`);
    return response.data || [];
  },

  async executionsExecute(flowId: string, inputPayload: Record<string, any>): Promise<Execution> {
    const response = await apiClient.post<Execution>('/executions', {
      flow_id: flowId,
      input_payload: inputPayload,
    });
    return response.data;
  },

  // Scheduler
  async schedulerListScheduledFlows(): Promise<ScheduledFlow[]> {
    const response = await apiClient.get<{ flows: ScheduledFlow[] }>('/scheduler/flows');
    return response.data?.flows || [];
  },

  async schedulerScheduleFlow(flowId: string, cronExpression: string): Promise<ScheduledFlow> {
    const response = await apiClient.post<ScheduledFlow>('/scheduler/flows', {
      flow_id: flowId,
      cron_expression: cronExpression,
    });
    return response.data;
  },

  async schedulerUnscheduleFlow(flowId: string): Promise<void> {
    await apiClient.delete(`/scheduler/flows/${flowId}`);
  },

  // Bricks
  async bricksList(): Promise<BrickListResponse> {
    const response = await apiClient.get<BrickListResponse>('/bricks');
    return response.data;
  },

  async bricksGetSchema(brickType: string): Promise<Record<string, any>> {
    const response = await apiClient.get<Record<string, any>>(`/bricks/${brickType}/schema`);
    return response.data;
  },

  // Usage
  async usageGetStats(): Promise<Record<string, any>> {
    const response = await apiClient.get<Record<string, any>>('/usage/stats');
    return response.data;
  },

  // Auth
  async authLogin(email: string, password: string): Promise<AuthResponse> {
    const response = await apiClient.post<AuthResponse>('/auth/login', {
      email,
      password,
    });
    // Store token in localStorage
    if (response.data.token) {
      if (typeof window !== 'undefined') {
        window.localStorage.setItem(AUTH_TOKEN_KEY, response.data.token);
      }
    }
    return response.data;
  },

  async authRegister(email: string, password: string): Promise<AuthResponse> {
    const response = await apiClient.post<AuthResponse>('/auth/register', {
      email,
      password,
    });
    // Store token in localStorage
    if (response.data.token) {
      if (typeof window !== 'undefined') {
        window.localStorage.setItem(AUTH_TOKEN_KEY, response.data.token);
      }
    }
    return response.data;
  },

  async authGetMe(): Promise<User> {
    const response = await apiClient.get<User>('/auth/me');
    return response.data;
  },

  async authCreateApiKey(request: CreateApiKeyRequest): Promise<ApiKeyResponse> {
    const response = await apiClient.post<ApiKeyResponse>('/auth/api-keys', request);
    return response.data;
  },

  async authListApiKeys(): Promise<ApiKeyListResponse> {
    const response = await apiClient.get<ApiKeyListResponse>('/auth/api-keys');
    return response.data;
  },

  async authDeleteApiKey(id: string): Promise<void> {
    await apiClient.delete(`/auth/api-keys/${id}`);
  },

  // Execution Data
  async executionsGetData(executionId: string): Promise<ExecutionData[]> {
    const response = await apiClient.get<ExecutionData[]>(`/executions/${executionId}/data`);
    return response.data;
  },

  async executionsGetBrickData(executionId: string, brickIndex: number): Promise<ExecutionData[]> {
    const response = await apiClient.get<ExecutionData[]>(`/executions/${executionId}/data/brick/${brickIndex}`);
    return response.data;
  },

  async executionsGetFetchedData(executionId: string): Promise<ExecutionData[]> {
    const response = await apiClient.get<ExecutionData[]>(`/executions/${executionId}/data/fetched`);
    return response.data;
  },

  async executionsGetIntermediateData(executionId: string): Promise<ExecutionData[]> {
    const response = await apiClient.get<ExecutionData[]>(`/executions/${executionId}/data/intermediate`);
    return response.data;
  },
};

export interface ExecutionData {
  id: string;
  execution_id: string;
  brick_index: number;
  brick_type: string;
  data_type: string;
  data_key: string;
  data_value: any;
  timestamp: string;
}
