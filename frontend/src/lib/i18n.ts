import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import zhCN from '@/locales/zh-CN.json';
import en from '@/locales/en.json';

// Get stored language or default to Chinese
const getDefaultLanguage = () => {
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem('evolith-language');
    if (stored) return stored;
  }
  return 'zh-CN'; // Default to Chinese
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      'zh-CN': {
        translation: zhCN,
      },
      en: {
        translation: en,
      },
    },
    fallbackLng: 'zh-CN',
    lng: getDefaultLanguage(),
    defaultNS: 'translation',
    ns: ['translation'],
    
    detection: {
      order: ['localStorage', 'navigator'],
      caches: ['localStorage'],
      lookupLocalStorage: 'evolith-language',
    },
    
    interpolation: {
      escapeValue: false,
    },
    
    react: {
      useSuspense: false,
    },
  });

export default i18n;

export const languages = [
  { code: 'zh-CN', name: '中文', nativeName: '中文' },
  { code: 'en', name: 'English', nativeName: 'English' },
];

export function changeLanguage(lng: string) {
  i18n.changeLanguage(lng);
  localStorage.setItem('evolith-language', lng);
}

export function getCurrentLanguage() {
  return i18n.language || 'zh-CN';
}
