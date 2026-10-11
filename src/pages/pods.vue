<script setup lang="ts">
import PageHeader from "@components/PageHeader.vue";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import { useFormat } from "@/composables/useFormat";
import { PodSummary } from "@/types/pods";
import DataTable, { DataTableColumn } from "@/components/DataTable.vue";
import { useLiveRefresh } from "@/composables/usePodmanEvents";

const { formatDate, formatAge, shortId } = useFormat();

const {
  state: pods,
  isReady,
  error,
  execute,
} = useAsyncState<PodSummary[]>(() => invoke("get_pods"), []);

useLiveRefresh(["pod", "container"], execute);

const columns: DataTableColumn<PodSummary>[] = [
  { id: "name", header: "Name", accessorKey: "Name" },
  { id: "id", header: "Pods ID", accessorFn: (pods) => shortId(pods.Id ?? "") },
  { id: "container", header: "Container", accessorKey: "Containers" },
  { id: "status", header: "Status", accessorKey: "Status" },
  {
    id: "created",
    header: "Created",
    accessorFn: (network) => network.Created,
  },
];
</script>

<template>
  <div>
    <PageHeader
      title="Pods"
      description="Groups of containers sharing the same network namespace."
    />

    <DataTable
      :columns="columns"
      :data="pods ?? []"
      :row-id="(pods: PodSummary) => pods.Id"
      :loading="!isReady && !error"
      loading-text="Loading images…"
      :empty-title="error ? 'Could not load pods' : 'No pods found'"
      :empty-description="
        error
          ? 'Check that the Podman socket is available, then try again.'
          : 'Pull an image to get started.'
      "
    >
      <template #cell-container="{ value }">
        <span :class="'font-medium text-zinc-900 dark:text-zinc-100'">
          {{ value[0].Names }}
        </span>
      </template>
      <template #cell-status="{ value }">
        <span :class="'text-zinc-600 dark:text-zinc-400'">
          {{ value }}
        </span>
      </template>
      <template #cell-id="{ value }">
        <span class="font-mono text-xs text-zinc-500 dark:text-zinc-400">{{
          value
        }}</span>
      </template>
      <template #cell-created="{ row }">
        <span
          class="text-zinc-500 dark:text-zinc-400"
          :title="formatDate(row.Created)"
        >
          {{ formatAge(row.Created) }}
        </span>
      </template>
      <template #cell-size="{ value }">
        <span class="tabular-nums text-zinc-500 dark:text-zinc-400">{{
          value
        }}</span>
      </template>
    </DataTable>
  </div>
</template>
