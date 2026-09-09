import en from './en.json';
import bn from './bn.json';
import hi from './hi.json';

type Dictionary = typeof en;
export type Locale = 'en' | 'bn' | 'hi';
const dictionaries: Record<Locale, Dictionary> = { en, bn, hi };

export const detectLocale = (): Locale => {
  const value = navigator.language.toLowerCase();
  if (value.startsWith('bn')) return 'bn';
  if (value.startsWith('hi')) return 'hi';
  return 'en';
};

export const translate = (locale: Locale, key: keyof Dictionary): string => dictionaries[locale][key] ?? en[key];
