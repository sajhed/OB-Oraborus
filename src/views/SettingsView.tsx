import { Eye, KeyRound, LockKeyhole, Mic, MonitorUp, Network, Shield, Volume2 } from 'lucide-react';
import { backend } from '../lib/backend';
import { useObStore } from '../store/useObStore';

const capabilities = [
  { key: 'microphone', label: 'Microphone', detail: 'Voice input and wake word', icon: Mic },
  { key: 'camera', label: 'Camera', detail: 'On-demand visual context', icon: Eye },
  { key: 'screenCapture', label: 'Screen capture', detail: 'On demand by default', icon: MonitorUp },
  { key: 'filesystem', label: 'Filesystem', detail: 'Approved folders only', icon: LockKeyhole },
  { key: 'browser', label: 'Browser control', detail: 'Playwright automation', icon: Network },
  { key: 'memory', label: 'Persistent memory', detail: 'User-approved facts only', icon: Shield },
] as const;

export function SettingsView() {
  const { snapshot, refresh } = useObStore();
  return <div className="view settings-view"><div className="view-heading"><span>SYSTEM PREFERENCES</span><h1>Settings</h1><p>Configure intelligence, permissions, privacy, and integrations without exposing secrets.</p></div><div className="settings-layout"><nav className="settings-nav">{['AI', 'Voice', 'Memory', 'Agents', 'Tools', 'Security', 'Privacy', 'Automation', 'Appearance', 'Startup', 'Network', 'Integrations', 'Developer', 'Market Lab'].map((item, index) => <button className={index === 6 ? 'active' : ''} key={item}>{item}</button>)}</nav><div className="settings-content"><section><div className="settings-title"><div><span>PRIVACY CENTER</span><h2>Capability access</h2><p>Every sensitive input has a visible switch and an audit trail.</p></div><Shield size={22} /></div>{capabilities.map(({ key, label, detail, icon: Icon }) => { const enabled = snapshot?.privacy[key] ?? false; return <div className="setting-row" key={key}><div className="setting-icon"><Icon size={18} /></div><div><strong>{label}</strong><span>{detail}</span></div><button className={`switch ${enabled ? 'on' : ''}`} role="switch" aria-checked={enabled} onClick={async () => { await backend.setPrivacy(key, !enabled); await refresh(); }}><i /></button></div>; })}</section><section className="settings-subsection"><div className="settings-title small"><div><span>VOICE OUTPUT</span><h2>Installed engines</h2></div><Volume2 size={20} /></div><div className="honest-state"><KeyRound size={18} /><div><strong>REQUIRES CONFIGURATION</strong><p>OB detects Piper, Windows SAPI, Coqui-compatible engines, Kokoro-compatible engines, and eSpeak NG on the host machine.</p></div></div></section></div></div></div>;
}
