import { Cloud, Cpu, LockKeyhole, Radio, ShieldCheck, WifiOff } from 'lucide-react';
import { formatPercent } from '../lib/format';
import { useObStore } from '../store/useObStore';

export function StatusBar() {
  const snapshot = useObStore((state) => state.snapshot);
  const disconnected = !snapshot || snapshot.connectivity === 'OFFLINE' || snapshot.connectivity === 'ERROR';
  return (
    <header className="status-bar">
      <div className="status-left">
        <span className={`status-chip ${disconnected ? 'muted' : 'positive'}`}>{disconnected ? <WifiOff size={14} /> : <Radio size={14} />} {snapshot?.connectivity ?? 'OFFLINE'}</span>
        {snapshot?.localOnly && <span className="status-chip"><LockKeyhole size={14} /> LOCAL ONLY</span>}
      </div>
      <div className="status-right">
        <span><Cpu size={14} /> CPU {formatPercent(snapshot?.system?.cpuPercent)}</span>
        <span><Cloud size={14} /> {snapshot?.providers.some((provider) => provider.state === 'AVAILABLE') ? 'AI READY' : 'AI NOT CONNECTED'}</span>
        <span><ShieldCheck size={14} /> LEVEL {snapshot?.autonomyLevel ?? 1}</span>
      </div>
    </header>
  );
}
