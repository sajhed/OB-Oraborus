import { useEffect } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { useObStore } from '../store/useObStore';

export function useDesktopEvents() {
  const setView = useObStore((state) => state.setView);
  useEffect(() => {
    if (!('__TAURI_INTERNALS__' in window)) return;
    const unlisteners: UnlistenFn[] = [];
    void listen('ob://quick-command', () => setView('chat')).then((value) => unlisteners.push(value));
    void listen('ob://open-settings', () => setView('settings')).then((value) => unlisteners.push(value));
    void register('CommandOrControl+Space', () => setView('chat'));
    return () => { for (const unlisten of unlisteners) unlisten(); void unregister('CommandOrControl+Space'); };
  }, [setView]);
}
