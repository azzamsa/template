<script setup lang="ts">
useHead({
  title: 'Frontend - About',
})
</script>

<template>
  <div class="mx-8 text-justify">
    <p class="mt-4">
      {{ $t('about.hi') }}
    </p>
    <p class="mt-4">
      {{ $t('about.thank') }}.
    </p>
  </div>
</template>

