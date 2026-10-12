<script setup lang="ts">
import type { LabelHTMLAttributes } from "vue";
import { computed } from "vue";
import { cn } from "@/lib/utils";

const props = defineProps<{ class?: LabelHTMLAttributes["class"] }>();
const classes = computed(() =>
  cn("text-sm font-medium text-zinc-900 dark:text-zinc-100", props.class),
);
</script>

<template>
  <label :class="classes"><slot /></label>
</template>
