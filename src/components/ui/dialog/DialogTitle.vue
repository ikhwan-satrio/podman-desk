<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { computed } from "vue";
import { DialogTitle, type DialogTitleProps } from "reka-ui";
import { cn } from "@/lib/utils";

const props = defineProps<
  DialogTitleProps & { class?: HTMLAttributes["class"] }
>();

const classes = computed(() =>
  cn("text-base font-semibold text-zinc-900 dark:text-zinc-100", props.class),
);
</script>

<template>
  <DialogTitle :class="classes"><slot /></DialogTitle>
</template>
