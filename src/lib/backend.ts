import type { AppSnapshot, CommandResponse, ProviderStatus, SystemSnapshot } from '../types';

type Invoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

const getInvoke = async (): Promise<Invoke> => {
  if (!('__TAURI_INTERNALS__' in window)) {
    throw new Error('UNAVAILABLE — OB desktop backend is not running. Launch with npm run tauri:dev or the installed Windows application.');
  }
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke;
};

export const backend = {
  async snapshot(): Promise<AppSnapshot> {
    return (await getInvoke())<AppSnapshot>('get_app_snapshot');
  },
  async runCommand(text: string, attachments: string[] = []): Promise<CommandResponse> {
    return (await getInvoke())<CommandResponse>('run_command', { request: { text, attachments } });
  },
  async stopAll(): Promise<void> {
    return (await getInvoke())<void>('stop_all');
  },
  async pause(): Promise<void> {
    return (await getInvoke())<void>('set_paused', { paused: true });
  },
  async resume(): Promise<void> {
    return (await getInvoke())<void>('set_paused', { paused: false });
  },
  async systemSnapshot(): Promise<SystemSnapshot> {
    return (await getInvoke())<SystemSnapshot>('get_system_snapshot');
  },
  async providerHealth(): Promise<ProviderStatus[]> {
    return (await getInvoke())<ProviderStatus[]>('get_provider_health');
  },
  async approve(approvalId: string, approved: boolean): Promise<CommandResponse> {
    return (await getInvoke())<CommandResponse>('resolve_approval', { approvalId, approved });
  },
  async setSecret(providerId: string, secret: string): Promise<void> {
    return (await getInvoke())<void>('store_provider_secret', { providerId, secret });
  },
  async setPrivacy(capability: string, enabled: boolean): Promise<AppSnapshot> {
    return (await getInvoke())<AppSnapshot>('set_privacy_capability', { capability, enabled });
  },
};
