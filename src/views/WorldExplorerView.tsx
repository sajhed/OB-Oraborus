import { CloudOff, Layers3, LocateFixed } from 'lucide-react';
import { useEffect, useRef } from 'react';

export function WorldExplorerView() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const canvas = canvasRef.current; if (!canvas) return;
    const context = canvas.getContext('2d'); if (!context) return;
    let frame = 0;
    const points = Array.from({ length: 520 }, (_, index) => { const phi = Math.acos(1 - 2 * (index + .5) / 520); const theta = Math.PI * (1 + Math.sqrt(5)) * index; return { x: Math.sin(phi) * Math.cos(theta), y: Math.cos(phi), z: Math.sin(phi) * Math.sin(theta) }; });
    const resize = () => { const box = canvas.getBoundingClientRect(); const ratio = Math.min(devicePixelRatio, 2); canvas.width = box.width * ratio; canvas.height = box.height * ratio; context.setTransform(ratio, 0, 0, ratio, 0, 0); };
    const draw = (now: number) => { const w = canvas.clientWidth, h = canvas.clientHeight; context.clearRect(0, 0, w, h); const r = Math.min(w, h) * .33, t = now * .00006; for (const point of points) { const x = point.x * Math.cos(t) - point.z * Math.sin(t); const z = point.x * Math.sin(t) + point.z * Math.cos(t); const px = w / 2 + x * r, py = h / 2 + point.y * r; context.fillStyle = `rgba(197,218,226,${.1 + (z + 1) * .24})`; context.fillRect(px, py, z > .2 ? 1.4 : .8, z > .2 ? 1.4 : .8); } context.strokeStyle = 'rgba(255,255,255,.1)'; context.lineWidth = 1; context.beginPath(); context.arc(w / 2, h / 2, r, 0, Math.PI * 2); context.stroke(); frame = requestAnimationFrame(draw); };
    resize(); addEventListener('resize', resize); frame = requestAnimationFrame(draw); return () => { removeEventListener('resize', resize); cancelAnimationFrame(frame); };
  }, []);
  return <div className="view world-view"><div className="view-heading inline"><div><span>WORLD EXPLORER</span><h1>Context at planetary scale</h1><p>Enable sourced layers for weather, time, research, countries, and configured datasets.</p></div><button className="secondary-button"><LocateFixed size={16} /> Locate</button></div><div className="world-stage"><canvas ref={canvasRef} aria-label="Point-cloud globe with no live layers enabled" /><div className="world-overlay left"><span>ACTIVE LAYERS</span><div className="layer-row"><CloudOff size={16} /><div><strong>No live layers</strong><small>Sources are not connected</small></div></div></div><div className="world-overlay right"><Layers3 size={16} /><div><span>DATA POLICY</span><strong>Source + timestamp required</strong></div></div><div className="world-caption"><strong>POINT-CLOUD EARTH</strong><span>Geographic visualization only · no current data displayed</span></div></div></div>;
}
