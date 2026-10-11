<script setup lang="ts">
import { ref } from "vue";
import PageHeader from "@components/PageHeader.vue";
import { Switch } from "@/components/ui/switch";

const toggles = ref([
  {
    label: "Start at login",
    description: "Launch Podman Desk when the system starts.",
    enabled: true,
  },
  {
    label: "Anonymous telemetry",
    description: "Help improve the app by sending anonymous usage data.",
    enabled: false,
  },
]);
</script>

<template>
  <div>
    <PageHeader
      title="Settings"
      description="Application preferences for Podman Desk."
    />

    <div
      class="mt-6 divide-y divide-zinc-100 rounded-xl border border-zinc-200 bg-white dark:divide-zinc-800 dark:border-zinc-800 dark:bg-zinc-900"
    >
      <div
        v-for="toggle in toggles"
        :key="toggle.label"
        class="flex items-center justify-between gap-4 px-4 py-4"
      >
        <div>
          <p class="text-sm font-medium text-zinc-900 dark:text-zinc-100">
            {{ toggle.label }}
          </p>
          <p class="mt-0.5 text-sm text-zinc-500 dark:text-zinc-400">
            {{ toggle.description }}
          </p>
        </div>
        <Switch
          @click="toggle.enabled = !toggle.enabled"
        />
      </div>
    </div>
  </div>
</template>
