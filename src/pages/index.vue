<script setup lang="ts">
import PageHeader from "@components/PageHeader.vue";
import { usePodmanEvents } from "@/composables/usePodmanEvents";
import { useFormat } from "@/composables/useFormat";

const { status, latest } = usePodmanEvents();
const { formatAge } = useFormat();
</script>

<template>
  <div>
    <PageHeader
      title="Dashboard"
      description="Manage your Podman resources from one place."
    />

    <section
      class="mt-6 rounded-xl border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
    >
      <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
        Podman socket
      </h2>
      <dl class="mt-3 space-y-2 text-sm">
        <div class="flex justify-between gap-4">
          <dt class="text-zinc-500 dark:text-zinc-400">Status</dt>
          <dd class="font-medium text-zinc-900 dark:text-zinc-100">
            {{ status.connected ? "Connected" : status.detail ? "Disconnected" : "Connecting…" }}
          </dd>
        </div>
        <div v-if="status.socket" class="flex justify-between gap-4">
          <dt class="text-zinc-500 dark:text-zinc-400">Socket</dt>
          <dd class="font-mono text-xs text-zinc-600 dark:text-zinc-300">
            {{ status.socket }}
          </dd>
        </div>
        <div v-if="latest" class="flex justify-between gap-4">
          <dt class="text-zinc-500 dark:text-zinc-400">Latest event</dt>
          <dd class="text-zinc-700 dark:text-zinc-300">
            {{ latest.kind }} · {{ latest.action }} ·
            {{ latest.name || latest.id.slice(0, 12) }}
            <span class="text-zinc-400 dark:text-zinc-500">
              ({{ formatAge(latest.timestamp) }})
            </span>
          </dd>
        </div>
      </dl>
    </section>
  </div>
</template>
