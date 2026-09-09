import { LockKeyhole, Radio } from 'lucide-react';
import { navigation } from '../app/navigation';
import { useObStore } from '../store/useObStore';
import { BrandMark } from './BrandMark';

export function Sidebar() {
  const { view, setView, snapshot } = useObStore();
  return (
    <aside className="sidebar" aria-label="Primary navigation">
      <BrandMark />
      <nav>
        {(['core', 'intelligence', 'system'] as const).map((section) => (
          <div className="nav-section" key={section}>
            <div className="nav-heading">{section}</div>
            {navigation.filter((item) => item.section === section).map(({ id, label, icon: Icon }) => (
              <button className={`nav-item ${view === id ? 'active' : ''}`} onClick={() => setView(id)} key={id} aria-current={view === id ? 'page' : undefined}>
                <Icon size={17} strokeWidth={1.7} /><span>{label}</span>
              </button>
            ))}
          </div>
        ))}
      </nav>
      <div className="sidebar-footer">
        <div className="privacy-mini"><LockKeyhole size={15} /><span>{snapshot?.localOnly ? 'Local only' : 'Privacy controlled'}</span></div>
        <div className="connection-mini"><Radio size={15} /><span>{snapshot?.connectivity ?? 'OFFLINE'}</span></div>
      </div>
    </aside>
  );
}
