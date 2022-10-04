import i18n from 'sveltekit-i18n'
import lang from './lang.json'

/** @type {import('sveltekit-i18n').Config} */
export const config = {
  translations: {
    en: { lang },
    id: { lang }
  },
  fallbackLocale: 'en',
  loaders: [
    {
      locale: 'en',
      key: 'home',
      routes: ['/'],
      loader: async () => (await import('./en/home.json')).default
    },
    {
      locale: 'id',
      key: 'home',
      routes: ['/'],
      loader: async () => (await import('./id/home.json')).default
    },
    {
      locale: 'en',
      key: 'about',
      routes: ['/about'],
      loader: async () => (await import('./en/about.json')).default
    },
    {
      locale: 'id',
      key: 'about',
      routes: ['/about'],
      loader: async () => (await import('./id/about.json')).default
    }
  ]
}

// eslint-disable-next-line new-cap
export const { t, loading, locales, locale, loadTranslations } = new i18n(config)

loading.subscribe(($loading) => $loading)
