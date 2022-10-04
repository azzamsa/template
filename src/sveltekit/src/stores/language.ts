import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'

export const currentLanguage: Writable<string> = writable('en')
