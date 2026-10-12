<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { computed } from "vue";
import { DialogDescription, type DialogDescriptionProps } from "reka-ui";
import { cn } from "@/lib/utils";

const props = defineProps<
  DialogDescriptionProps & { class?: HTMLAttributes["class"] }
>();

const classes = computed(() =>
  cn("text-sm text-zinc-500 dark:text-zinc-400", props.class),
);
</script>

<template>
  <DialogDescription :class="classes"><slot /></DialogDescription>
</template>
