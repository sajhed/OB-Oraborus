import { create } from 'zustand';
import { backend } from '../lib/backend';
import { detectLocale, type Locale } from '../i18n';
import type { AppSnapshot, ChatMessage, OraState, ViewId } from '../types';

interface ObStore {
  view: ViewId;
  locale: Locale;
  snapshot?: AppSnapshot;
  messages: ChatMessage[];
  commandDraft: string;
  backendError?: string;
  busy: boolean;
  setView: (view: ViewId) => void;
  setLocale: (locale: Locale) => void;
  setCommandDraft: (value: string) => void;
  refresh: () => Promise<void>;
  submit: (text?: string) => Promise<void>;
  stopAll: () => Promise<void>;
  setOraState: (state: OraState) => void;
}

const initialMessage: ChatMessage = {
  id: crypto.randomUUID(),
  role: 'system',
  content: 'OB is ready. Connected capabilities will report real results; unavailable services remain visibly unavailable.',
  createdAt: new Date().toISOString(),
  state: 'complete',
};

export const useObStore = create<ObStore>((set, get) => ({
  view: 'home',
  locale: detectLocale(),
  messages: [initialMessage],
  commandDraft: '',
  busy: false,
  setView: (view) => set({ view }),
  setLocale: (locale) => set({ locale }),
  setCommandDraft: (commandDraft) => set({ commandDraft }),
  setOraState: (oraState) => set((state) => state.snapshot ? { snapshot: { ...state.snapshot, oraState } } : state),
  refresh: async () => {
    try {
      const snapshot = await backend.snapshot();
      set({ snapshot, backendError: undefined });
    } catch (error) {
      set({ backendError: error instanceof Error ? error.message : String(error) });
    }
  },
  submit: async (input) => {
    const text = (input ?? get().commandDraft).trim();
    if (!text || get().busy) return;
    const userMessage: ChatMessage = { id: crypto.randomUUID(), role: 'user', content: text, createdAt: new Date().toISOString(), state: 'complete' };
    set((state) => ({ messages: [...state.messages, userMessage], commandDraft: '', busy: true }));
    get().setOraState('THINKING');
    try {
      const result = await backend.runCommand(text);
      const assistantMessage: ChatMessage = {
        id: crypto.randomUUID(),
        role: 'assistant',
        content: result.message,
        createdAt: new Date().toISOString(),
        state: result.state === 'FAILED' ? 'error' : 'complete',
        meta: { provider: result.provider, model: result.model, routeReason: result.routeReason },
      };
      set((state) => ({ messages: [...state.messages, assistantMessage], busy: false, view: 'chat' }));
      await get().refresh();
    } catch (error) {
      const detail = error instanceof Error ? error.message : String(error);
      set((state) => ({
        messages: [...state.messages, { id: crypto.randomUUID(), role: 'assistant', content: detail, createdAt: new Date().toISOString(), state: 'error' }],
        busy: false,
        backendError: detail,
        view: 'chat',
      }));
    } finally {
      get().setOraState('IDLE');
    }
  },
  stopAll: async () => {
    try {
      await backend.stopAll();
      await get().refresh();
    } catch (error) {
      set({ backendError: error instanceof Error ? error.message : String(error) });
    }
  },
}));
