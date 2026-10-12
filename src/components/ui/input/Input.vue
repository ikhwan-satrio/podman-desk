<script setup lang="ts">
import type { InputHTMLAttributes } from "vue";
import { computed } from "vue";
import { cn } from "@/lib/utils";

const props = defineProps<{
  class?: InputHTMLAttributes["class"];
  modelValue?: string;
  placeholder?: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{ "update:modelValue": [value: string] }>();

const classes = computed(() =>
  cn(
    "flex h-8 w-full rounded-md border border-zinc-200 bg-white px-3 py-1 text-sm text-zinc-900 placeholder:text-zinc-400 focus-visible:border-sky-500 focus-visible:outline-2 focus-visible:outline-sky-500/20 disabled:cursor-not-allowed disabled:opacity-50 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100 dark:placeholder:text-zinc-500",
    props.class,
  ),
);
</script>

<template>
  <input
    :class="classes"
    :value="modelValue"
    :placeholder="placeholder"
    :disabled="disabled"
    @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
  />
</template>
