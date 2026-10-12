<script setup lang="ts">
import { EllipsisVertical } from "@lucide/vue";
import { Button } from "@/components/ui/button";
import {
  DropdownMenuRoot,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu";

export interface RowAction {
  label: string;
  onSelect: () => void;
  disabled?: boolean;
  danger?: boolean;
  separatorBefore?: boolean;
}

withDefaults(
  defineProps<{
    items: RowAction[];
    disabled?: boolean;
    label?: string;
  }>(),
  {
    disabled: false,
    label: "Actions",
  },
);
</script>

<template>
  <DropdownMenuRoot>
    <DropdownMenuTrigger as-child>
      <Button
        size="sm"
        variant="ghost"
        class="px-1.5 text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
        :disabled="disabled"
        :title="label"
      >
        <EllipsisVertical />
        <span class="sr-only">{{ label }}</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end" :side-offset="4">
      <template v-for="(item, index) in items" :key="index">
        <DropdownMenuSeparator v-if="item.separatorBefore && index > 0" />
        <DropdownMenuItem
          :disabled="item.disabled"
          :class="
            item.danger
              ? 'text-rose-600 focus:bg-rose-50 focus:text-rose-700 dark:text-rose-400 dark:focus:bg-rose-950/50 dark:focus:text-rose-300'
              : undefined
          "
          @select="item.onSelect"
        >
          {{ item.label }}
        </DropdownMenuItem>
      </template>
    </DropdownMenuContent>
  </DropdownMenuRoot>
</template>
