import { Bot, CircleAlert, UserRound } from 'lucide-react';
import { useEffect, useRef } from 'react';
import { useObStore } from '../store/useObStore';

export function ChatView() {
  const { messages, busy } = useObStore();
  const endRef = useRef<HTMLDivElement>(null);
  useEffect(() => endRef.current?.scrollIntoView({ behavior: 'smooth' }), [messages, busy]);
  return (
    <div className="view chat-view">
      <div className="view-heading"><span>CONVERSATION</span><h1>Chat with OB</h1><p>Natural language, files, tools, agents, and verifiable outcomes in one thread.</p></div>
      <div className="message-list">
        {messages.map((message) => <article className={`message ${message.role} ${message.state ?? ''}`} key={message.id}><div className="message-avatar">{message.role === 'user' ? <UserRound size={17} /> : message.state === 'error' ? <CircleAlert size={17} /> : <Bot size={17} />}</div><div className="message-body"><div className="message-meta"><strong>{message.role === 'user' ? 'You' : message.role === 'system' ? 'System' : 'OB'}</strong><time>{new Date(message.createdAt).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</time></div><p>{message.content}</p>{message.meta && <div className="model-trace">{message.meta.provider && <span>{message.meta.provider}</span>}{message.meta.model && <span>{message.meta.model}</span>}{message.meta.routeReason && <span>{message.meta.routeReason}</span>}</div>}</div></article>)}
        {busy && <article className="message assistant"><div className="message-avatar"><Bot size={17} /></div><div className="message-body"><div className="message-meta"><strong>OB</strong></div><div className="thinking-line"><i /><i /><i /><span>Thinking and routing</span></div></div></article>}
        <div ref={endRef} />
      </div>
    </div>
  );
}
