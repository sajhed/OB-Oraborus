import { describe, expect, it } from 'vitest';
import { formatBytes, formatPercent, timeAgo } from './format';

describe('format helpers', () => {
  it('never substitutes a numeric reading when unavailable', () => {
    expect(formatBytes(undefined)).toBe('UNAVAILABLE');
    expect(formatPercent(undefined)).toBe('UNAVAILABLE');
  });
  it('formats measured values', () => {
    expect(formatBytes(1024)).toBe('1.0 KB');
    expect(formatPercent(42.4)).toBe('42%');
  });
  it('formats recent timestamps', () => {
    expect(timeAgo(new Date().toISOString())).toBe('just now');
  });
});
