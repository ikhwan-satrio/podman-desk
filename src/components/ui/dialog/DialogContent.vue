<script setup lang="ts">
import type { DialogContentEmits, DialogContentProps } from "reka-ui";
import type { HTMLAttributes } from "vue";
import { reactiveOmit } from "@vueuse/core";
import {
  DialogClose,
  DialogContent,
  DialogOverlay,
  DialogPortal,
  useForwardPropsEmits,
} from "reka-ui";
import { X } from "@lucide/vue";
import { cn } from "@/lib/utils";

const props = defineProps<
  DialogContentProps & { class?: HTMLAttributes["class"] }
>();

const emits = defineEmits<DialogContentEmits>();

const delegatedProps = reactiveOmit(props, "class");
const forwarded = useForwardPropsEmits(delegatedProps, emits);
</script>

<template>
  <DialogPortal>
    <DialogOverlay
      class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-zinc-950/50 backdrop-blur-sm"
    />
    <DialogContent
      v-bind="forwarded"
      :class="
        cn(
          'data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-1/2 left-1/2 z-50 flex max-h-[85vh] w-full max-w-lg -translate-x-1/2 -translate-y-1/2 flex-col gap-4 rounded-xl border border-zinc-200 bg-white p-6 shadow-lg dark:border-zinc-800 dark:bg-zinc-900',
          props.class,
        )
      "
    >
      <div class="grid min-h-0 flex-1 gap-4 overflow-y-auto">
        <slot />
      </div>

      <DialogClose
        class="absolute top-4 right-4 rounded-md p-1 text-zinc-400 opacity-70 transition-opacity hover:opacity-100 focus-visible:outline-2 focus-visible:outline-sky-500 hover:text-zinc-900 dark:hover:text-zinc-100"
        aria-label="Close"
      >
        <X class="h-4 w-4" />
      </DialogClose>
    </DialogContent>
  </DialogPortal>
</template>
