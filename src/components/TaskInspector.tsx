import { CheckCircle2, CircleDashed, KeyRound, TriangleAlert } from 'lucide-react';
import { timeAgo } from '../lib/format';
import { useObStore } from '../store/useObStore';

const labels = ['REQUEST', 'PLAN', 'AGENTS', 'TOOLS', 'PERMISSIONS', 'PROGRESS', 'RESULT', 'VERIFICATION', 'ERRORS'];
export function TaskInspector() {
  const task = useObStore((state) => state.snapshot?.activeTask);
  return (
    <aside className="inspector">
      <div className="panel-heading"><div><span>PROCESS INSPECTOR</span><strong>{task ? task.title : 'No active process'}</strong></div><CircleDashed size={18} /></div>
      {!task ? (
        <div className="inspector-empty"><div className="quiet-orbit" /><strong>System idle</strong><p>Task details appear here when OB plans or executes work.</p></div>
      ) : (
        <div className="inspector-content">
          <div className="task-status-line"><span className={`run-state ${task.status}`}>{task.status.replace('_', ' ')}</span><small>{timeAgo(task.createdAt)}</small></div>
          {labels.map((label) => {
            const events = task.events.filter((event) => event.kind === label.toLowerCase() || (label === 'AGENTS' && event.kind === 'agent') || (label === 'TOOLS' && event.kind === 'tool') || (label === 'PERMISSIONS' && event.kind === 'permission') || (label === 'ERRORS' && event.kind === 'error'));
            return <section className="inspector-section" key={label}><h3>{label}</h3>{events.length ? events.map((event) => <div className="event-line" key={event.id}>{event.kind === 'error' ? <TriangleAlert size={14} /> : event.kind === 'permission' ? <KeyRound size={14} /> : <CheckCircle2 size={14} />}<div><strong>{event.label}</strong><p>{event.detail}</p></div></div>) : <p className="empty-line">No entries</p>}</section>;
          })}
        </div>
      )}
    </aside>
  );
}
