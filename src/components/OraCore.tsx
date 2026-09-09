import { useEffect, useRef } from 'react';
import type { OraState } from '../types';

interface Particle { x: number; y: number; z: number; seed: number; }
const STATE_SPEED: Record<OraState, number> = { IDLE: 0.00018, LISTENING: 0.00036, THINKING: 0.00078, PLANNING: 0.00055, EXECUTING: 0.00092, SPEAKING: 0.00048, ERROR: 0.0011, OFFLINE: 0.00005 };

export function OraCore({ state, compact = false }: { state: OraState; compact?: boolean }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const context = canvas.getContext('2d');
    if (!context) return;
    const particles: Particle[] = Array.from({ length: compact ? 520 : 900 }, (_, index) => {
      const phi = Math.acos(1 - 2 * (index + 0.5) / (compact ? 520 : 900));
      const theta = Math.PI * (1 + Math.sqrt(5)) * index;
      return { x: Math.sin(phi) * Math.cos(theta), y: Math.cos(phi), z: Math.sin(phi) * Math.sin(theta), seed: (index * 73) % 101 };
    });
    let frame = 0;
    let start = performance.now();
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    const resize = () => {
      const box = canvas.getBoundingClientRect();
      const ratio = Math.min(window.devicePixelRatio || 1, 2);
      canvas.width = Math.max(1, Math.round(box.width * ratio));
      canvas.height = Math.max(1, Math.round(box.height * ratio));
      context.setTransform(ratio, 0, 0, ratio, 0, 0);
    };
    const draw = (now: number) => {
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;
      context.clearRect(0, 0, width, height);
      const t = reduced ? 0 : (now - start) * STATE_SPEED[state];
      const radius = Math.min(width, height) * (compact ? 0.31 : 0.34);
      const cos = Math.cos(t), sin = Math.sin(t);
      const points = particles.map((particle) => {
        const pulse = state === 'THINKING' ? Math.sin(now * 0.004 + particle.seed) * 0.025 : 0;
        const x = particle.x * cos - particle.z * sin;
        const z = particle.x * sin + particle.z * cos;
        const scale = (1 + z * 0.22 + pulse);
        return { x: width / 2 + x * radius * scale, y: height / 2 + particle.y * radius * scale, z, alpha: 0.18 + (z + 1) * 0.31 };
      }).sort((a, b) => a.z - b.z);
      for (const point of points) {
        const size = 0.65 + (point.z + 1) * 0.72;
        const error = state === 'ERROR';
        context.fillStyle = error ? `rgba(233,115,102,${point.alpha})` : `rgba(220,236,244,${point.alpha})`;
        context.beginPath(); context.arc(point.x, point.y, size, 0, Math.PI * 2); context.fill();
      }
      const halo = context.createRadialGradient(width / 2, height / 2, radius * 0.1, width / 2, height / 2, radius * 1.12);
      halo.addColorStop(0, state === 'ERROR' ? 'rgba(233,115,102,.08)' : 'rgba(118,179,204,.08)');
      halo.addColorStop(1, 'rgba(0,0,0,0)');
      context.fillStyle = halo; context.fillRect(0, 0, width, height);
      frame = requestAnimationFrame(draw);
    };
    resize(); window.addEventListener('resize', resize); frame = requestAnimationFrame(draw);
    return () => { cancelAnimationFrame(frame); window.removeEventListener('resize', resize); start = 0; };
  }, [compact, state]);

  return <div className={`ora-core ${compact ? 'compact' : ''}`}><canvas ref={canvasRef} aria-label={`ORA Core state: ${state}`} /><div className="ora-label"><span>ORA CORE</span><strong>{state}</strong></div></div>;
}
