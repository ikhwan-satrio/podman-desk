<script setup lang="ts">
import { useRoute } from "vue-router";
import PageHeader from "@components/PageHeader.vue";

const route = useRoute();
</script>

<template>
  <div>
    <PageHeader title="404" description="Page not found." />
    <p class="mt-6 text-sm text-zinc-500 dark:text-zinc-400">
      No route matches
      <code class="rounded bg-zinc-100 px-1.5 py-0.5 text-zinc-700 dark:bg-zinc-800 dark:text-zinc-300">
        {{ route.path }}
      </code>
    </p>
    <RouterLink
      to="/"
      class="mt-4 inline-flex items-center gap-1 text-sm font-medium text-sky-600 hover:underline dark:text-sky-400"
    >
      ← Back to dashboard
    </RouterLink>
  </div>
</template>
