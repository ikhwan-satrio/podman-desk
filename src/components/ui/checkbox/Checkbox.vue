<script setup lang="ts">
import type { CheckboxRootEmits, CheckboxRootProps } from "reka-ui";
import type { HTMLAttributes } from "vue";
import { reactiveOmit } from "@vueuse/core";
import { CheckboxIndicator, CheckboxRoot, useForwardPropsEmits } from "reka-ui";
import { Check, Minus } from "@lucide/vue";
import { cn } from "@/lib/utils";

const props = defineProps<
  CheckboxRootProps & { class?: HTMLAttributes["class"] }
>();

const emits = defineEmits<CheckboxRootEmits>();

const delegatedProps = reactiveOmit(props, "class");
const forwarded = useForwardPropsEmits(delegatedProps, emits);
</script>

<template>
  <CheckboxRoot
    v-bind="forwarded"
    :class="
      cn(
        'flex size-4 shrink-0 items-center justify-center rounded-sm border border-zinc-300 bg-white shadow-xs transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-500 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:border-sky-600 data-[state=checked]:bg-sky-600 data-[state=indeterminate]:border-sky-600 data-[state=indeterminate]:bg-sky-600 dark:border-zinc-700 dark:bg-zinc-900',
        props.class,
      )
    "
  >
    <CheckboxIndicator class="flex items-center justify-center text-white">
      <Minus v-if="modelValue === 'indeterminate'" class="size-3" />
      <Check v-else class="size-3" />
    </CheckboxIndicator>
  </CheckboxRoot>
</template>
