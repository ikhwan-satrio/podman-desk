<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { computed } from "vue";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-1.5 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-500 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
  {
    variants: {
      variant: {
        default:
          "bg-sky-600 text-white shadow-xs hover:bg-sky-500 dark:bg-sky-500 dark:hover:bg-sky-400",
        destructive:
          "bg-rose-600 text-white shadow-xs hover:bg-rose-500 dark:bg-rose-500 dark:hover:bg-rose-400",
        outline:
          "border border-zinc-200 bg-white text-zinc-900 shadow-xs hover:bg-zinc-50 dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100 dark:hover:bg-zinc-800",
        secondary:
          "bg-zinc-100 text-zinc-900 shadow-xs hover:bg-zinc-200 dark:bg-zinc-800 dark:text-zinc-100 dark:hover:bg-zinc-700",
        ghost: "text-zinc-600 hover:bg-zinc-100 hover:text-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100",
        link: "text-sky-600 underline-offset-4 hover:underline dark:text-sky-400",
      },
      size: {
        default: "h-8 px-3 py-1.5",
        sm: "h-7 rounded-md px-2 text-xs",
        lg: "h-9 rounded-md px-4",
        icon: "size-8",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

type ButtonVariant = VariantProps<typeof buttonVariants>;

const props = withDefaults(
  defineProps<{
    variant?: ButtonVariant["variant"];
    size?: ButtonVariant["size"];
    class?: HTMLAttributes["class"];
    type?: "button" | "submit" | "reset";
  }>(),
  { type: "button" },
);

const classes = computed(() => cn(buttonVariants({ variant: props.variant, size: props.size }), props.class));
</script>

<template>
  <button :type="type" :class="classes">
    <slot />
  </button>
</template>
