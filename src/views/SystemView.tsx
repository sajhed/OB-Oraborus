import { Cpu, Database, HardDrive, MemoryStick, Network, RefreshCw, TriangleAlert } from 'lucide-react';
import { backend } from '../lib/backend';
import { formatBytes, formatPercent } from '../lib/format';
import { useObStore } from '../store/useObStore';

export function SystemView() {
  const { snapshot, refresh } = useObStore();
  const system = snapshot?.system;
  const cards = [
    { label: 'CPU', value: formatPercent(system?.cpuPercent), sub: 'Measured utilization', icon: Cpu },
    { label: 'MEMORY', value: formatBytes(system?.memoryUsedBytes), sub: system?.memoryTotalBytes ? `of ${formatBytes(system.memoryTotalBytes)}` : 'UNAVAILABLE', icon: MemoryStick },
    { label: 'DISK', value: formatBytes(system?.diskUsedBytes), sub: system?.diskTotalBytes ? `of ${formatBytes(system.diskTotalBytes)}` : 'UNAVAILABLE', icon: HardDrive },
    { label: 'NETWORK RX', value: formatBytes(system?.networkReceivedBytes), sub: 'Since monitor refresh', icon: Network },
  ];
  return <div className="view system-view"><div className="view-heading inline"><div><span>OB HEALTH CENTER</span><h1>System</h1><p>Measurements are read from the machine. Unsupported sensors remain unavailable.</p></div><button className="secondary-button" onClick={() => void refresh()}><RefreshCw size={16} /> Refresh</button></div><div className="metric-grid">{cards.map(({ label, value, sub, icon: Icon }) => <section className="metric-card" key={label}><div><span>{label}</span><Icon size={18} /></div><strong>{value}</strong><small>{sub}</small></section>)}</div><div className="system-columns"><section className="surface-card"><div className="section-title"><div><span>PROVIDERS</span><h2>AI connections</h2></div><Database size={18} /></div><div className="provider-list">{snapshot?.providers.map((provider) => <div className="provider-row" key={provider.id}><i className={provider.state === 'AVAILABLE' ? 'positive' : ''} /><div><strong>{provider.label}</strong><span>{provider.activeModel ?? provider.detail ?? provider.state.replaceAll('_', ' ')}</span></div><b>{provider.state.replaceAll('_', ' ')}</b></div>) ?? <div className="empty-state"><TriangleAlert size={20} /><strong>Backend unavailable</strong></div>}</div></section><section className="surface-card"><div className="section-title"><div><span>SENSORS</span><h2>Availability notes</h2></div><TriangleAlert size={18} /></div>{system?.sensorNotes.length ? <ul className="notes-list">{system.sensorNotes.map((note) => <li key={note}>{note}</li>)}</ul> : <div className="empty-state"><strong>No sensor notes</strong><p>Refresh after the desktop backend starts.</p></div>}</section></div></div>;
}
