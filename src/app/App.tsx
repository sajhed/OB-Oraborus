import { useEffect } from 'react';
import { CommandBar } from '../components/CommandBar';
import { Sidebar } from '../components/Sidebar';
import { StatusBar } from '../components/StatusBar';
import { TaskInspector } from '../components/TaskInspector';
import { useObStore } from '../store/useObStore';
import { useDesktopEvents } from '../hooks/useDesktopEvents';
import { AgentTownView } from '../views/AgentTownView';
import { ChatView } from '../views/ChatView';
import { HomeView } from '../views/HomeView';
import { ModuleView } from '../views/ModuleView';
import { SettingsView } from '../views/SettingsView';
import { SystemView } from '../views/SystemView';
import { WorldExplorerView } from '../views/WorldExplorerView';

export function App() {
  useDesktopEvents();
  const { view, refresh, backendError } = useObStore();
  useEffect(() => { void refresh(); const timer = window.setInterval(() => void refresh(), 5_000); return () => clearInterval(timer); }, [refresh]);
  const content = view === 'home' ? <HomeView /> : view === 'chat' ? <ChatView /> : view === 'agent-town' ? <AgentTownView /> : view === 'world' ? <WorldExplorerView /> : view === 'system' ? <SystemView /> : view === 'settings' ? <SettingsView /> : <ModuleView id={view} />;
  return <div className="app-shell"><Sidebar /><main className="workspace"><StatusBar />{backendError && <div className="backend-banner" role="status">{backendError}</div>}<div className="workspace-scroll">{content}</div><CommandBar /></main><TaskInspector /></div>;
}
