<script setup lang="ts">
const counterStore = useCounterStore()
</script>

<template>
  <div class="mt-20 hero">
    <div class="text-center hero-content">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold">
          {{ $t('index.hi') }}
          <i-ph:hand-waving class="inline-block w-10 h-10 animate-bounce" />
        </h1>

        <p class="py-6">
          {{ $t('index.intro') }}
        </p>
      </div>
    </div>
  </div>
</template>
