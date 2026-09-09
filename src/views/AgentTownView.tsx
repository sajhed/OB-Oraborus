import { Bot, CircleDashed } from 'lucide-react';
import { useMemo } from 'react';
import { useObStore } from '../store/useObStore';

export function AgentTownView() {
  const agents = useObStore((state) => state.snapshot?.agents ?? []);
  const positioned = useMemo(() => agents.map((agent, index) => { const ring = 110 + Math.floor(index / 10) * 88; const angle = (index % 10) / 10 * Math.PI * 2 - Math.PI / 2; return { ...agent, x: 50 + Math.cos(angle) * ring / 6, y: 50 + Math.sin(angle) * ring / 4.6 }; }), [agents]);
  const active = agents.filter((agent) => agent.state !== 'idle').length;
  return (
    <div className="view town-view">
      <div className="view-heading inline"><div><span>REAL-TIME ORCHESTRATION</span><h1>Agent Town</h1><p>Every node reflects an actual registered agent and its current run state.</p></div><div className="town-summary"><strong>{agents.length}</strong><span>REGISTERED</span><strong>{active}</strong><span>ACTIVE</span></div></div>
      <div className="town-canvas">
        {!agents.length ? <div className="empty-state town-empty"><CircleDashed size={28} /><strong>Agent registry unavailable</strong><p>Launch the OB desktop backend to load real agents.</p></div> : <><svg className="town-lines" viewBox="0 0 100 100" preserveAspectRatio="none" aria-hidden="true">{positioned.filter((agent) => agent.parentAgent).map((agent) => { const parent = positioned.find((item) => item.id === agent.parentAgent); return parent ? <line key={agent.id} x1={parent.x} y1={parent.y} x2={agent.x} y2={agent.y} /> : null; })}</svg>{positioned.map((agent) => <button key={agent.id} className={`agent-node ${agent.state}`} style={{ left: `${agent.x}%`, top: `${agent.y}%` }} title={`${agent.name}: ${agent.state}`}><Bot size={16} /><span>{agent.name}</span><i /></button>)}<div className="town-center"><div className="town-core-dot" /><strong>OB PRIME</strong><span>{active ? 'ORCHESTRATING' : 'IDLE'}</span></div></>}
      </div>
      <div className="legend-row">{['idle', 'working', 'waiting', 'queued', 'completed', 'failed', 'paused'].map((state) => <span key={state}><i className={state} />{state}</span>)}</div>
    </div>
  );
}
