<script setup lang="ts">
import { ref, watch } from "vue";
import { useForm } from "@tanstack/vue-form";
import { invoke } from "@tauri-apps/api/core";
import {
  DialogRoot,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

const props = defineProps<{
  open: boolean;
  containerId: string;
  initialName: string;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
  renamed: [];
}>();

const formError = ref<string | null>(null);

const form = useForm({
  defaultValues: { name: props.initialName },
  onSubmit: async ({ value }) => {
    formError.value = null;
    try {
      await invoke("rename_container", {
        id: props.containerId,
        name: value.name.trim(),
      });
      emit("update:open", false);
      emit("renamed");
    } catch (error) {
      formError.value = String(error);
    }
  },
});

const nameValidator = ({ value }: { value: string }) => {
  if (!value.trim()) return "Name is required.";
  if (/\s/.test(value)) return "Name cannot contain spaces.";
  return undefined;
};

watch(
  () => props.open,
  (open) => {
    if (open) {
      formError.value = null;
      form.reset({ name: props.initialName });
    }
  },
);
</script>

<template>
  <DialogRoot
    :open="open"
    @update:open="(value) => emit('update:open', value)"
  >
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Edit container</DialogTitle>
        <DialogDescription>
          Rename the container. Existing links to the old name will stop
          working.
        </DialogDescription>
      </DialogHeader>

      <form class="grid gap-4" @submit.prevent="form.handleSubmit()">
        <form.Field
          name="name"
          :validators="{ onChange: nameValidator }"
        >
          <template #default="{ field }">
            <div class="grid gap-2">
              <Label :for="`rename-${field.name}`">Name</Label>
              <Input
                :id="`rename-${field.name}`"
                :model-value="field.state.value"
                placeholder="container-name"
                autocomplete="off"
                @update:model-value="field.handleChange"
              />
              <p
                v-if="field.state.meta.errors.length"
                class="text-xs text-rose-600 dark:text-rose-400"
              >
                {{ field.state.meta.errors[0] }}
              </p>
            </div>
          </template>
        </form.Field>

        <p
          v-if="formError"
          class="text-xs text-rose-600 dark:text-rose-400"
        >
          {{ formError }}
        </p>

        <DialogFooter>
          <Button
            variant="outline"
            type="button"
            @click="emit('update:open', false)"
          >
            Cancel
          </Button>
          <Button type="submit">Save</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </DialogRoot>
</template>
