<script lang="ts">
 import Icon from '@iconify/svelte'
 import { t } from '$lib/translations'
</script>

<div class="mt-20 hero">
    <div class="text-center hero-content">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">
                {$t('home.hi')}
                <Icon icon="ph:hand-waving" class="inline-block w-10 h-10 animate-bounce"/>
            </h1>

            <p class="py-6">
                {$t('home.intro')}
            </p>
        </div>
    </div>
</div>
