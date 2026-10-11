<script setup lang="ts">
import PageHeader from "@components/PageHeader.vue";
import DataTable from "@components/DataTable.vue";
import type { DataTableColumn } from "@components/DataTable.vue";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import { ContainerSummary } from "@/types/containers";
import { useLiveRefresh } from "@/composables/usePodmanEvents";

const {
  state: containers,
  isReady,
  execute,
} = useAsyncState<ContainerSummary[]>(() => invoke("get_containers"), []);

useLiveRefresh(["container"], execute);

const statusClass = (status: string) =>
  status === "Running"
    ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400"
    : "bg-zinc-500/10 text-zinc-600 dark:text-zinc-400";

const columns: DataTableColumn<ContainerSummary>[] = [
  {
    id: "name",
    header: "Name",
    accessorFn: (container) => container.Names[0] ?? container.Id.slice(0, 12),
  },
  { id: "image", header: "Image", accessorKey: "Image" },
  { id: "status", header: "Status", accessorKey: "Status" },
];
</script>

<template>
  <div>
    <PageHeader
      title="Containers"
      description="All containers managed by Podman."
    />

    <DataTable
      :columns="columns"
      :data="containers ?? []"
      :row-id="(container: ContainerSummary) => container.Id"
      :loading="!isReady"
      loading-text="Loading containers…"
      empty-title="No containers found"
      empty-description="Run a container to see it listed here."
    >
      <template #cell-name="{ row }">
        <RouterLink
          :to="{ path: `/containers/${row.Id}` }"
          class="font-medium text-sky-600 hover:underline dark:text-sky-400"
        >
          {{ row.Names[0] ?? row.Id.slice(0, 12) }}
        </RouterLink>
      </template>
      <template #cell-image="{ value }">
        <span class="font-mono text-xs text-zinc-500 dark:text-zinc-400">{{ value }}</span>
      </template>
      <template #cell-status="{ value }">
        <span
          class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-medium"
          :class="statusClass(value)"
        >
          <span class="h-1.5 w-1.5 rounded-full bg-current" aria-hidden="true" />
          {{ value }}
        </span>
      </template>
    </DataTable>
  </div>
</template>
