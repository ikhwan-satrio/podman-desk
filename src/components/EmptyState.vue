<script setup lang="ts">
defineProps<{ title: string; description?: string }>();
</script>

<template>
  <div
    class="mt-6 rounded-xl border border-dashed border-zinc-300 bg-white px-6 py-12 text-center dark:border-zinc-700 dark:bg-zinc-900"
  >
    <p class="text-sm font-medium text-zinc-900 dark:text-zinc-100">{{ title }}</p>
    <p v-if="description" class="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
      {{ description }}
    </p>
    <div v-if="$slots.default" class="mt-4">
      <slot />
    </div>
  </div>
</template>
