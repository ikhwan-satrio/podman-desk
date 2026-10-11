<script setup lang="ts">
import PageHeader from "@components/PageHeader.vue";
import DataTable from "@components/DataTable.vue";
import type { DataTableColumn } from "@components/DataTable.vue";
import { useAsyncState } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/core";
import { useFormat } from "@/composables/useFormat";
import { NetworkSummary } from "@/types/networks";
import { useLiveRefresh } from "@/composables/usePodmanEvents";

const { formatDate, formatAge, shortId } = useFormat();

const {
  state: networks,
  isReady,
  error,
  execute,
} = useAsyncState<NetworkSummary[]>(() => invoke("get_networks"), []);

useLiveRefresh(["network", "container"], execute);

const columns: DataTableColumn<NetworkSummary>[] = [
  { id: "name", header: "Name", accessorKey: "Name" },
  {
    id: "id",
    header: "Network ID",
    accessorFn: (network) => shortId(network.Id ?? ""),
  },
  { id: "driver", header: "Driver", accessorKey: "Driver" },
  { id: "scope", header: "Scope", accessorKey: "Scope" },
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
      title="Networks"
      description="Networks for containers to communicate over."
    />

    <DataTable
      :columns="columns"
      :data="networks ?? []"
      :row-id="
        (_network: NetworkSummary, index: number) =>
          _network.Id ?? _network.Name ?? String(index)
      "
      :loading="!isReady && !error"
      loading-text="Loading networks…"
      :empty-title="error ? 'Could not load networks' : 'No networks found'"
      :empty-description="
        error
          ? 'Check that the Podman socket is available, then try again.'
          : 'Create a network to connect containers together.'
      "
    >
      <template #cell-name="{ value }">
        <span class="font-medium text-zinc-900 dark:text-zinc-100">
          {{ value || "—" }}
        </span>
      </template>
      <template #cell-id="{ row }">
        <span
          class="font-mono text-xs text-zinc-500 dark:text-zinc-400"
          :title="row.Id"
        >
          {{ shortId(row.Id ?? "") || "—" }}
        </span>
      </template>
      <template #cell-driver="{ value }">
        <span class="text-zinc-600 dark:text-zinc-400">{{ value || "—" }}</span>
      </template>
      <template #cell-scope="{ value }">
        <span
          class="inline-flex items-center rounded-full bg-zinc-500/10 px-2 py-0.5 text-xs font-medium text-zinc-600 dark:bg-zinc-500/20 dark:text-zinc-400"
        >
          {{ value || "—" }}
        </span>
      </template>
      <template #cell-created="{ row }">
        <span
          class="text-zinc-500 dark:text-zinc-400"
          :title="formatDate(row.Created)"
        >
          {{ formatAge(row.Created) }}
        </span>
      </template>
    </DataTable>
  </div>
</template>
