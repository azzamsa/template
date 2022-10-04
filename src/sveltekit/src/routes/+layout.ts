import type { LayoutLoad } from './$types'
import { loadTranslations } from '$lib/translations'
import { currentLanguage } from '$stores/language'

export const load: LayoutLoad = async ({ url }) => {
  let currentLanguageValue = 'en'
  currentLanguage.subscribe((value) => {
    currentLanguageValue = value
  })

  const { pathname } = url
  await loadTranslations(currentLanguageValue, pathname)
  return {}
}
