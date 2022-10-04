<script lang="ts">
 import Icon from '@iconify/svelte'
 import { page } from '$app/stores'
 import { currentLanguage } from '$stores/language'
 import { locale } from '$lib/translations'

 let currentLanguageValue = "en"
 currentLanguage.subscribe(value => {
     currentLanguageValue = value
 })

 let showMenu = false
 // Locale
 let locales = ["en", "id"]
 function toggleLocale() {
     let newLanguage = locales[(locales.indexOf(currentLanguageValue) + 1) % locales.length]
     currentLanguage.set(newLanguage)
     locale.set(newLanguage)
 }
</script>

<div>
    <div class="mb-2 shadow-lg navbar rounded-box bg-primary">
        <div class="md:hidden">
            <button on:click={() => (showMenu = !showMenu)} class="btn btn-ghost btn-square">
                {#if !showMenu}
                    <Icon icon="tabler:menu-2" class="w-8 h-8"/>
                {:else}
                    <Icon icon="tabler:x" class="w-8 h-8" />
                {/if}
            </button>
        </div>

        <div class="px-2 mx-2 navbar-start">
            <span class="text-lg font-bold">
                <Icon icon="noto:building-construction" class="w-8 h-8"/>
            </span>
        </div>

        <div class="px-2 mx-2 navbar-center">
            <div class="hidden items-stretch space-x-4 md:flex">
                <a href="/" class="font-bold" class:active={$page.url.pathname === '/'}>Home</a>
                <a href="/about" class="font-bold" class:active={$page.url.pathname === '/about'}>About</a>
            </div>
        </div>

        <div class="navbar-end">
            <button on:click={toggleLocale} class="btn btn-ghost btn-square">
                <Icon icon="tabler:language" class="w-8 h-8"/>
            </button>
        </div>
    </div>

    {#if showMenu}
        <ul
            class="py-3 md:hidden menu rounded-box bg-primary"
        >
            <li>
                <a href="/" on:click={() => (showMenu = !showMenu)} class="font-bold" class:active={$page.url.pathname === '/'}>Home</a>
            </li>
            <div class="mt-0 mb-0 divider" />
            <li>
                <a href="/about" on:click={() => (showMenu = !showMenu)} class="font-bold" class:active={$page.url.pathname === '/about'}>About</a>
            </li>
        </ul>
    {/if}
</div>
