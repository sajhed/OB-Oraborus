import { ArrowUp, Mic, Paperclip, ShieldAlert, Square } from 'lucide-react';
import { useObStore } from '../store/useObStore';

export function CommandBar() {
  const { commandDraft, setCommandDraft, submit, busy, stopAll } = useObStore();
  return (
    <div className="command-dock">
      <div className="command-bar">
        <button className="icon-button" aria-label="Attach file"><Paperclip size={18} /></button>
        <div className="command-field">
          {!commandDraft && <span className="command-hint">Ask or command OB</span>}
          <input value={commandDraft} onChange={(event) => setCommandDraft(event.target.value)} onKeyDown={(event) => { if (event.key === 'Enter' && !event.shiftKey) { event.preventDefault(); void submit(); } }} aria-label="Ask or command OB" />
        </div>
        <button className="icon-button" aria-label="Voice input"><Mic size={18} /></button>
        {busy ? <button className="send-button danger" onClick={() => void stopAll()} aria-label="Stop all"><Square size={16} /></button> : <button className="send-button" onClick={() => void submit()} aria-label="Send command"><ArrowUp size={18} /></button>}
      </div>
      <div className="command-caption"><ShieldAlert size={12} /> Sensitive and destructive actions require approval · Ctrl + Space</div>
    </div>
  );
}
