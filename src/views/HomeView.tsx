import { ArrowUpRight, Bell, CheckCircle2, CircleDashed } from 'lucide-react';
import { quickActions } from '../app/navigation';
import { useObStore } from '../store/useObStore';
import { OraCore } from '../components/OraCore';
import { timeAgo } from '../lib/format';

export function HomeView() {
  const { snapshot, submit, setView } = useObStore();
  const greeting = new Date().getHours() < 12 ? 'Good morning' : new Date().getHours() < 18 ? 'Good afternoon' : 'Good evening';
  return (
    <div className="view home-view">
      <div className="hero-copy"><span className="eyebrow">OB · INTELLIGENCE LAYER</span><h1>{greeting}.</h1><p>OB sees. OB thinks. OB acts — only through capabilities you control.</p></div>
      <div className="home-core"><OraCore state={snapshot?.oraState ?? 'OFFLINE'} /><div className="core-meta left"><span>SYSTEM STATE</span><strong>{snapshot?.connectivity ?? 'OFFLINE'}</strong></div><div className="core-meta right"><span>AGENT ACTIVITY</span><strong>{snapshot?.agents.filter((agent) => agent.state === 'working').length ?? 0} ACTIVE</strong></div></div>
      <section className="quick-grid" aria-label="Quick actions">
        {quickActions.map(({ label, command, icon: Icon }) => <button key={label} onClick={() => command ? void submit(command) : setView('chat')}><Icon size={18} /><span>{label}</span><ArrowUpRight size={15} /></button>)}
      </section>
      <div className="home-lower">
        <section className="surface-card"><div className="section-title"><div><span>RECENT</span><h2>Task history</h2></div><button onClick={() => setView('tasks')}>View all</button></div>{snapshot?.recentTasks.length ? <div className="recent-list">{snapshot.recentTasks.slice(0, 4).map((task) => <div className="recent-row" key={task.id}>{task.status === 'completed' ? <CheckCircle2 size={17} /> : <CircleDashed size={17} />}<div><strong>{task.title}</strong><span>{task.status.replace('_', ' ')} · {timeAgo(task.createdAt)}</span></div></div>)}</div> : <div className="empty-state"><CircleDashed size={22} /><strong>No task history yet</strong><p>Your verified task results will appear here.</p></div>}</section>
        <section className="surface-card"><div className="section-title"><div><span>ATTENTION</span><h2>Notifications</h2></div><Bell size={18} /></div>{snapshot?.unreadNotifications ? <p>{snapshot.unreadNotifications} unread notification{snapshot.unreadNotifications === 1 ? '' : 's'}.</p> : <div className="empty-state"><Bell size={22} /><strong>Nothing needs attention</strong><p>OB remains quiet when no useful action is required.</p></div>}</section>
      </div>
    </div>
  );
}
