export function BrandMark({ compact = false }: { compact?: boolean }) {
  return (
    <div className="brand-mark" aria-label="OB Oraborus">
      <div className="brand-glyph" aria-hidden="true"><span>O</span><span>B</span></div>
      {!compact && <div className="brand-copy"><strong>OB</strong><small>ORABORUS</small></div>}
    </div>
  );
}
