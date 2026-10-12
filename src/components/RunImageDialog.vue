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
  imageRef: string;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
  started: [containerId: string];
}>();

const formError = ref<string | null>(null);

const form = useForm({
  defaultValues: {
    name: "",
    env: "",
    ports: "",
    cmd: "",
  },
  onSubmit: async ({ value }) => {
    formError.value = null;

    const env = value.env
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);

    const ports = value.ports
      .split(/[\s,]+/)
      .map((item) => item.trim())
      .filter(Boolean);

    for (const port of ports) {
      if (!/^\d+:\d+$/.test(port)) {
        formError.value = `Invalid port mapping "${port}". Use host:container (e.g. 8080:80).`;
        return;
      }
    }

    try {
      const containerId = await invoke<string>("run_image", {
        req: {
          image: props.imageRef,
          name: value.name.trim() || null,
          env: env.length ? env : null,
          ports: ports.length ? ports : null,
          cmd: value.cmd.trim()
            ? value.cmd.trim().split(/\s+/)
            : null,
        },
      });
      emit("update:open", false);
      emit("started", containerId);
    } catch (error) {
      formError.value = String(error);
    }
  },
});

const nameValidator = ({ value }: { value: string }) => {
  if (value && !/^[a-zA-Z0-9][a-zA-Z0-9_.-]*$/.test(value)) {
    return 'Only letters, numbers, "_", "." and "-" are allowed.';
  }
  return undefined;
};

watch(
  () => props.open,
  (open) => {
    if (open) {
      formError.value = null;
      form.reset({ name: "", env: "", ports: "", cmd: "" });
    }
  },
);
</script>

<template>
  <DialogRoot
    :open="open"
    @update:open="(value) => emit('update:open', value)"
  >
    <DialogContent class="max-w-xl">
      <DialogHeader>
        <DialogTitle>Run image</DialogTitle>
        <DialogDescription>
          Create and start a container from
          <span class="font-mono">{{ imageRef }}</span>.
        </DialogDescription>
      </DialogHeader>

      <form class="grid gap-4" @submit.prevent="form.handleSubmit()">
        <form.Field
          name="name"
          :validators="{ onChange: nameValidator }"
        >
          <template #default="{ field }">
            <div class="grid gap-2">
              <Label :for="`run-${field.name}`">Container name</Label>
              <Input
                :id="`run-${field.name}`"
                :model-value="field.state.value"
                placeholder="optional"
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

        <form.Field name="ports">
          <template #default="{ field }">
            <div class="grid gap-2">
              <Label :for="`run-${field.name}`">Ports</Label>
              <Input
                :id="`run-${field.name}`"
                :model-value="field.state.value"
                placeholder="8080:80 3000:3000"
                autocomplete="off"
                @update:model-value="field.handleChange"
              />
              <p class="text-xs text-zinc-500 dark:text-zinc-400">
                host:container, dipisah spasi atau koma.
              </p>
            </div>
          </template>
        </form.Field>

        <form.Field name="env">
          <template #default="{ field }">
            <div class="grid gap-2">
              <Label :for="`run-${field.name}`">Environment</Label>
              <textarea
                :id="`run-${field.name}`"
                :value="field.state.value"
                rows="3"
                placeholder="KEY=value (satu per baris)"
                class="flex w-full rounded-md border border-zinc-200 bg-white px-3 py-2 font-mono text-xs text-zinc-900 placeholder:font-sans placeholder:text-zinc-400 focus-visible:border-sky-500 focus-visible:outline-2 focus-visible:outline-sky-500/20 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100 dark:placeholder:text-zinc-500"
                @input="
                  field.handleChange(
                    ($event.target as HTMLTextAreaElement).value,
                  )
                "
              />
            </div>
          </template>
        </form.Field>

        <form.Field name="cmd">
          <template #default="{ field }">
            <div class="grid gap-2">
              <Label :for="`run-${field.name}`">Command</Label>
              <Input
                :id="`run-${field.name}`"
                :model-value="field.state.value"
                placeholder="optional override"
                autocomplete="off"
                @update:model-value="field.handleChange"
              />
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
          <Button type="submit">Run</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </DialogRoot>
</template>
