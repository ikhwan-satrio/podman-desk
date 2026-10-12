<script setup lang="ts">
import {
  DialogRoot,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

withDefaults(
  defineProps<{
    open: boolean;
    title: string;
    description?: string;
    confirmLabel?: string;
    loading?: boolean;
    error?: string;
  }>(),
  {
    description: undefined,
    confirmLabel: "Delete",
    loading: false,
    error: undefined,
  },
);

const emit = defineEmits<{
  "update:open": [value: boolean];
  confirm: [];
}>();
</script>

<template>
  <DialogRoot
    :open="open"
    @update:open="(value) => emit('update:open', value)"
  >
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{{ title }}</DialogTitle>
        <DialogDescription v-if="description">
          {{ description }}
        </DialogDescription>
      </DialogHeader>

      <p
        v-if="error"
        class="text-xs text-rose-600 dark:text-rose-400"
      >
        {{ error }}
      </p>

      <DialogFooter>
        <Button
          variant="outline"
          :disabled="loading"
          @click="emit('update:open', false)"
        >
          Cancel
        </Button>
        <Button
          variant="destructive"
          :disabled="loading"
          @click="emit('confirm')"
        >
          {{ loading ? "Working…" : confirmLabel }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </DialogRoot>
</template>
