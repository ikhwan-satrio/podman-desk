<script setup lang="ts">
import { X } from "@lucide/vue";
import { Button } from "@/components/ui/button";
import type { RowAction } from "@components/RowActions.vue";

withDefaults(
  defineProps<{
    count: number;
    actions: RowAction[];
    disabled?: boolean;
    noun?: string;
  }>(),
  {
    disabled: false,
    noun: "selected",
  },
);

const emit = defineEmits<{ clear: [] }>();
</script>

<template>
  <div
    v-if="count > 0"
    class="mt-4 flex flex-wrap items-center gap-x-3 gap-y-2 rounded-xl border border-sky-200 bg-sky-50 px-3 py-2 dark:border-sky-900 dark:bg-sky-950/40"
  >
    <span class="text-sm font-semibold text-sky-700 dark:text-sky-300">
      {{ count }} {{ noun }}
    </span>
    <div class="flex flex-wrap items-center gap-1.5">
      <Button
        v-for="(action, index) in actions"
        :key="index"
        size="sm"
        variant="outline"
        :class="
          action.danger
            ? 'border-rose-200 text-rose-600 hover:bg-rose-50 hover:text-rose-700 dark:border-rose-900 dark:text-rose-400 dark:hover:bg-rose-950/50 dark:hover:text-rose-300'
            : undefined
        "
        :disabled="disabled || action.disabled"
        @click="action.onSelect()"
      >
        {{ action.label }}
      </Button>
    </div>
    <button
      type="button"
      class="ml-auto inline-flex items-center gap-1 text-xs font-medium text-zinc-500 transition-colors hover:text-zinc-900 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-500 dark:text-zinc-400 dark:hover:text-zinc-100"
      @click="emit('clear')"
    >
      <X class="size-3.5" />
      Clear
    </button>
  </div>
</template>
