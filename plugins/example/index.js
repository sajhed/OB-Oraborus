export const inspect = (input) => ({ state: 'COMPLETED', keys: Object.keys(input ?? {}).sort() });
