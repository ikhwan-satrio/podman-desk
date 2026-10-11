<script setup lang="ts">
defineProps<{ title: string; description?: string }>();
</script>

<template>
  <header>
    <h1 class="text-2xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
      {{ title }}
    </h1>
    <p
      v-if="description || $slots.description"
      class="mt-1 text-sm text-zinc-500 dark:text-zinc-400"
    >
      <slot>{{ description }}</slot>
    </p>
  </header>
</template>
