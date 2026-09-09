import {
  Activity, Bot, BrainCircuit, CalendarClock, Code2, FileSearch, Globe2, Home,
  Landmark, MessageSquare, Network, Search, Settings, Sparkles, Workflow,
  type LucideIcon,
} from 'lucide-react';
import type { ViewId } from '../types';

export interface NavItem { id: ViewId; label: string; icon: LucideIcon; section: 'core' | 'intelligence' | 'system'; }
export const navigation: NavItem[] = [
  { id: 'home', label: 'Home', icon: Home, section: 'core' },
  { id: 'chat', label: 'Chat', icon: MessageSquare, section: 'core' },
  { id: 'tasks', label: 'Tasks', icon: CalendarClock, section: 'core' },
  { id: 'agents', label: 'Agents', icon: Bot, section: 'intelligence' },
  { id: 'agent-town', label: 'Agent Town', icon: Network, section: 'intelligence' },
  { id: 'world', label: 'World Explorer', icon: Globe2, section: 'intelligence' },
  { id: 'research', label: 'Research', icon: Search, section: 'intelligence' },
  { id: 'files', label: 'Files', icon: FileSearch, section: 'intelligence' },
  { id: 'memory', label: 'Memory', icon: BrainCircuit, section: 'intelligence' },
  { id: 'automations', label: 'Automations', icon: Workflow, section: 'intelligence' },
  { id: 'developer', label: 'Developer', icon: Code2, section: 'system' },
  { id: 'market', label: 'Market Lab', icon: Landmark, section: 'system' },
  { id: 'system', label: 'System', icon: Activity, section: 'system' },
  { id: 'settings', label: 'Settings', icon: Settings, section: 'system' },
];

export const quickActions = [
  { label: 'Ask OB', command: '', icon: Sparkles },
  { label: 'Inspect system', command: "What's using my RAM?", icon: Activity },
  { label: 'Search files', command: 'Find files in my approved folders', icon: FileSearch },
  { label: 'Start research', command: 'Research: ', icon: Search },
];
