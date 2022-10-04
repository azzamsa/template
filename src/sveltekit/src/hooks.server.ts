import type { Handle } from '@sveltejs/kit'
import { locale } from '$lib/translations'
import { currentLanguage } from '$stores/language'

export const handle: Handle = async ({ event, resolve }) => {
  let currentLanguageValue = 'en'
  currentLanguage.subscribe((value) => {
    currentLanguageValue = value
  })
  locale.set(currentLanguageValue)
  const response = await resolve(event)
  return response
}
