<script setup lang="ts">
import { computed, ref } from "vue";
import PageHeader from "@components/PageHeader.vue";
import DataTable from "@components/DataTable.vue";
import type { DataTableColumn } from "@components/DataTable.vue";
import ConfirmDialog from "@components/ConfirmDialog.vue";
import RowActions from "@components/RowActions.vue";
import type { RowAction } from "@components/RowActions.vue";
import BulkActionsBar from "@components/BulkActionsBar.vue";
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
  { id: "actions", header: "", accessorFn: () => "" },
];

const deleteOpen = ref(false);
const deleteTarget = ref<NetworkSummary | null>(null);
const deleteLoading = ref(false);
const deleteError = ref<string | undefined>(undefined);

const openDelete = (network: NetworkSummary) => {
  bulkDelete.value = false;
  deleteTarget.value = network;
  deleteError.value = undefined;
  deleteOpen.value = true;
};

const selected = ref<string[]>([]);

const selectedRows = computed(() => {
  const ids = new Set(selected.value);
  return (networks.value ?? []).filter((network, index) =>
    ids.has(network.Id ?? network.Name ?? String(index)),
  );
});

const bulkDelete = ref(false);

const openBulkDelete = () => {
  deleteTarget.value = null;
  deleteError.value = undefined;
  bulkDelete.value = true;
  deleteOpen.value = true;
};

const deleteDescription = computed(() => {
  if (bulkDelete.value) {
    return `${selectedRows.value.length} networks will be permanently removed.`;
  }
  if (deleteTarget.value) {
    return `Network ${deleteTarget.value.Name ?? shortId(deleteTarget.value.Id ?? "")} will be permanently removed.`;
  }
  return undefined;
});

const bulkItems = computed<RowAction[]>(() =>
  selectedRows.value.length
    ? [
        {
          label: "Delete",
          danger: true,
          onSelect: openBulkDelete,
        },
      ]
    : [],
);

const confirmDelete = async () => {
  const rows: NetworkSummary[] = bulkDelete.value
    ? selectedRows.value
    : deleteTarget.value
      ? [deleteTarget.value]
      : [];
  if (!rows.length) return;
  deleteLoading.value = true;
  deleteError.value = undefined;
  try {
    for (const row of rows) {
      if (!row.Name) continue;
      await invoke("remove_network", { name: row.Name });
    }
    deleteOpen.value = false;
    deleteTarget.value = null;
    bulkDelete.value = false;
    selected.value = [];
    await execute();
  } catch (e) {
    deleteError.value = String(e);
  } finally {
    deleteLoading.value = false;
  }
};

const rowActions = (network: NetworkSummary): RowAction[] => [
  { label: "Delete", danger: true, onSelect: () => openDelete(network) },
];
</script>

<template>
  <div>
    <PageHeader
      title="Networks"
      description="Networks for containers to communicate over."
    />

    <BulkActionsBar
      :count="selected.length"
      :actions="bulkItems"
      noun="networks selected"
      @clear="selected = []"
    />

    <DataTable
      v-model:selected="selected"
      :selectable="true"
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
      <template #cell-actions="{ row }">
        <div class="flex items-center justify-end">
          <RowActions :items="rowActions(row)" />
        </div>
      </template>
    </DataTable>

    <ConfirmDialog
      v-if="deleteTarget || bulkDelete"
      v-model:open="deleteOpen"
      title="Delete network?"
      :description="deleteDescription"
      confirm-label="Delete"
      :loading="deleteLoading"
      :error="deleteError"
      @confirm="confirmDelete"
    />
  </div>
</template>
