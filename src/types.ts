export type Connectivity = 'ONLINE' | 'LOCAL' | 'CLOUD' | 'OFFLINE' | 'PAUSED' | 'PRIVATE' | 'ERROR';
export type OraState = 'IDLE' | 'LISTENING' | 'THINKING' | 'PLANNING' | 'EXECUTING' | 'SPEAKING' | 'ERROR' | 'OFFLINE';
export type CapabilityState = 'AVAILABLE' | 'NOT_CONNECTED' | 'UNAVAILABLE' | 'REQUIRES_CONFIGURATION' | 'DISABLED';
export type AgentRunState = 'idle' | 'working' | 'waiting' | 'queued' | 'completed' | 'failed' | 'paused';
export type RiskLevel = 'READ_ONLY' | 'SAFE' | 'SENSITIVE' | 'DESTRUCTIVE' | 'CRITICAL';

export interface AgentDescriptor {
  id: string;
  name: string;
  purpose: string;
  tools: string[];
  permissions: RiskLevel[];
  modelPreference: string[];
  timeoutMs: number;
  retry: { maxAttempts: number; backoffMs: number };
  verification: string;
  state: AgentRunState;
  currentTask?: string;
  parentAgent?: string;
  collaborators?: string[];
}

export interface ProviderStatus {
  id: string;
  label: string;
  state: CapabilityState;
  activeModel?: string;
  capabilities: string[];
  latencyMs?: number;
  detail?: string;
}

export interface SystemSnapshot {
  measuredAt: string;
  cpuPercent?: number;
  memoryUsedBytes?: number;
  memoryTotalBytes?: number;
  diskUsedBytes?: number;
  diskTotalBytes?: number;
  batteryPercent?: number;
  gpu?: Array<{ name: string; utilizationPercent?: number; memoryUsedBytes?: number; memoryTotalBytes?: number }>;
  networkReceivedBytes?: number;
  networkTransmittedBytes?: number;
  sensorNotes: string[];
}

export interface TaskEvent {
  id: string;
  at: string;
  kind: 'request' | 'plan' | 'agent' | 'tool' | 'permission' | 'progress' | 'result' | 'verification' | 'error';
  label: string;
  detail: string;
}

export interface TaskSummary {
  id: string;
  title: string;
  status: 'queued' | 'running' | 'waiting_approval' | 'completed' | 'partial' | 'failed' | 'cancelled';
  createdAt: string;
  completedAt?: string;
  progress?: { completed: number; total: number };
  events: TaskEvent[];
}

export interface AppSnapshot {
  connectivity: Connectivity;
  oraState: OraState;
  localOnly: boolean;
  autonomyLevel: 0 | 1 | 2 | 3 | 4;
  providers: ProviderStatus[];
  agents: AgentDescriptor[];
  activeTask?: TaskSummary;
  recentTasks: TaskSummary[];
  system?: SystemSnapshot;
  privacy: {
    microphone: boolean;
    camera: boolean;
    screenCapture: boolean;
    filesystem: boolean;
    browser: boolean;
    network: boolean;
    memory: boolean;
  };
  unreadNotifications: number;
}

export interface CommandResponse {
  taskId: string;
  state: 'COMPLETED' | 'WAITING_APPROVAL' | 'PARTIAL' | 'FAILED' | 'CANCELLED';
  message: string;
  provider?: string;
  model?: string;
  routeReason?: string;
  approval?: {
    id: string;
    action: string;
    risk: RiskLevel;
    summary: string;
    expiresAt: string;
  };
  citations?: Array<{ title: string; url: string; retrievedAt: string }>;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  createdAt: string;
  state?: 'sending' | 'complete' | 'error';
  meta?: { provider?: string; model?: string; routeReason?: string };
}

export type ViewId = 'home' | 'chat' | 'tasks' | 'agents' | 'agent-town' | 'world' | 'research' | 'files' | 'memory' | 'automations' | 'developer' | 'market' | 'system' | 'settings';
