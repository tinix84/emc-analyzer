<template>
  <div>
    <NavigationMenu />
    <main class="min-h-screen bg-gray-50">
      <NuxtPage />
    </main>
  </div>
</template>

<script setup lang="ts">
// Set global head configuration
useHead({
  title: 'EMC Spectrum Analyzer',
  meta: [
    { name: 'description', content: 'EMC compliance analyzer with Rust WebAssembly' },
    { name: 'keywords', content: 'EMC, spectrum analyzer, compliance, CISPR, EN55032' }
  ],
  link: [
    { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
  ]
})
</script>
