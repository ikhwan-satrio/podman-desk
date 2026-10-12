<script setup lang="ts">
import { DialogTrigger, type DialogTriggerProps } from "reka-ui";
import { cn } from "@/lib/utils";

const props = defineProps<DialogTriggerProps & { class?: string }>();
</script>

<template>
  <DialogTrigger :class="cn(props.class)" as-child>
    <slot />
  </DialogTrigger>
</template>
