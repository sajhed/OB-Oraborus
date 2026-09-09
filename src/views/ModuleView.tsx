import { Archive, Bot, BrainCircuit, CalendarClock, CircleDashed, Code2, FileSearch, Landmark, Search, Workflow } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import type { ViewId } from '../types';
import { useObStore } from '../store/useObStore';

const config: Partial<Record<ViewId, { eyebrow: string; title: string; description: string; empty: string; action: string; icon: LucideIcon }>> = {
  tasks: { eyebrow: 'TASK GRAPH', title: 'Tasks', description: 'Inspect dependencies, approvals, retries, verification, and outcomes.', empty: 'No tasks are queued.', action: 'Create a task', icon: CalendarClock },
  agents: { eyebrow: 'AGENT REGISTRY', title: 'Agents', description: 'Thirty purpose-built agents with scoped tools and independent permissions.', empty: 'Agent registry is unavailable.', action: 'Inspect agents', icon: Bot },
  research: { eyebrow: 'ORABORUS RESEARCH ENGINE', title: 'Research', description: 'Collect, cross-check, synthesize, and cite real sources.', empty: 'No research task is active.', action: 'Start research', icon: Search },
  files: { eyebrow: 'FILE INTELLIGENCE', title: 'Files', description: 'Search and organize only the folders you approve.', empty: 'No matching files found.', action: 'Search approved folders', icon: FileSearch },
  memory: { eyebrow: 'MY WORLD', title: 'Memory', description: 'View, edit, export, or forget approved persistent context.', empty: 'No approved memories stored.', action: 'Review memory policy', icon: BrainCircuit },
  automations: { eyebrow: 'AUTOMATION ENGINE', title: 'Automations', description: 'Scheduled and event-driven workflows with separate permissions.', empty: 'No automations are enabled.', action: 'Create automation', icon: Workflow },
  developer: { eyebrow: 'ORABORUS DEV CONSOLE', title: 'Developer', description: 'Inspect projects, builds, tests, diffs, tools, and execution traces.', empty: 'No workspace is open.', action: 'Open a project', icon: Code2 },
  market: { eyebrow: 'ORABORUS MARKET LAB', title: 'Market Lab', description: 'Research, backtesting, alerts, and paper trading with hard safety limits.', empty: 'Market Lab is disabled by default.', action: 'Review safeguards', icon: Landmark },
};

export function ModuleView({ id }: { id: ViewId }) {
  const item = config[id] ?? { eyebrow: 'OB MODULE', title: id, description: '', empty: 'No data available.', action: 'Open', icon: Archive };
  const Icon = item.icon;
  const snapshot = useObStore((state) => state.snapshot);
  const agents = id === 'agents' ? snapshot?.agents ?? [] : [];
  return <div className="view module-view"><div className="view-heading inline"><div><span>{item.eyebrow}</span><h1>{item.title}</h1><p>{item.description}</p></div><button className="primary-button"><Icon size={16} /> {item.action}</button></div>{agents.length ? <div className="agent-grid">{agents.map((agent) => <section className="agent-card" key={agent.id}><div><Icon size={18} /><span className={`agent-state ${agent.state}`}>{agent.state}</span></div><h2>{agent.name}</h2><p>{agent.purpose}</p><footer><span>{agent.tools.length} tools</span><span>{Math.round(agent.timeoutMs / 1000)}s timeout</span></footer></section>)}</div> : <div className="module-empty"><div className="empty-symbol"><CircleDashed size={34} /></div><strong>{item.empty}</strong><p>OB will display verified results and real connection states here.</p><button className="secondary-button"><Icon size={16} /> {item.action}</button></div>}</div>;
}
