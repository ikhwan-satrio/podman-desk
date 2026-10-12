<script setup lang="ts">
import { computed, ref } from "vue";
import PageHeader from "@components/PageHeader.vue";
import ConfirmDialog from "@components/ConfirmDialog.vue";
import RowActions from "@components/RowActions.vue";
import type { RowAction } from "@components/RowActions.vue";
import BulkActionsBar from "@components/BulkActionsBar.vue";
import { useAsyncState } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/core";
import DataTable, { DataTableColumn } from "@/components/DataTable.vue";
import { VolumeSummary } from "@/types/volumes";
import { useFormat } from "@/composables/useFormat";
import { useLiveRefresh } from "@/composables/usePodmanEvents";

const {
  state: volumes,
  isReady,
  error,
  execute,
} = useAsyncState<VolumeSummary[]>(() => invoke("get_volumes"), []);

const { formatAge } = useFormat();

useLiveRefresh(["volume"], execute);

const columns: DataTableColumn<VolumeSummary>[] = [
  { id: "name", header: "Name", accessorKey: "Name" },
  { id: "scope", header: "Scope", accessorKey: "Driver" },
  {
    id: "age",
    header: "Age",
    accessorFn: (v) => v.CreatedAt,
  },
  { id: "actions", header: "", accessorFn: () => "" },
];

const deleteOpen = ref(false);
const deleteTarget = ref<VolumeSummary | null>(null);
const deleteLoading = ref(false);
const deleteError = ref<string | undefined>(undefined);

const openDelete = (volume: VolumeSummary) => {
  bulkDelete.value = false;
  deleteTarget.value = volume;
  deleteError.value = undefined;
  deleteOpen.value = true;
};

const selected = ref<string[]>([]);

const selectedRows = computed(() => {
  const ids = new Set(selected.value);
  return (volumes.value ?? []).filter((volume) => ids.has(volume.Name));
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
    return `${selectedRows.value.length} volumes and all of their data will be permanently removed.`;
  }
  if (deleteTarget.value) {
    return `Volume ${deleteTarget.value.Name} and all of its data will be permanently removed.`;
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
  const rows: VolumeSummary[] = bulkDelete.value
    ? selectedRows.value
    : deleteTarget.value
      ? [deleteTarget.value]
      : [];
  if (!rows.length) return;
  deleteLoading.value = true;
  deleteError.value = undefined;
  try {
    for (const row of rows) {
      await invoke("remove_volume", { name: row.Name, force: true });
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

const rowActions = (volume: VolumeSummary): RowAction[] => [
  { label: "Delete", danger: true, onSelect: () => openDelete(volume) },
];
</script>

<template>
  <div>
    <PageHeader
      title="Volumes"
      description="Persistent data stored outside of containers."
    />

    <BulkActionsBar
      :count="selected.length"
      :actions="bulkItems"
      noun="volumes selected"
      @clear="selected = []"
    />

    <DataTable
      v-model:selected="selected"
      :selectable="true"
      :columns="columns"
      :data="volumes ?? []"
      :row-id="(v: VolumeSummary) => v.Name"
      :loading="!isReady && !error"
      loading-text="Loading volumes…"
      :empty-title="error ? 'Could not load volumes' : 'No volumes found'"
      :empty-description="
        error
          ? 'Check that the Podman socket is available, then try again.'
          : 'Pull an image to get started.'
      "
    >
      <template #cell-name="{ value }">
        <span :class="'font-medium text-zinc-900 dark:text-zinc-100'">
          {{ value }}
        </span>
      </template>
      <template #cell-scope="{ value }">
        <span
          class="inline-flex items-center rounded-full bg-zinc-500/10 px-2 py-0.5 text-xs font-medium text-zinc-600 dark:bg-zinc-500/20 dark:text-zinc-400"
        >
          {{ value }}
        </span>
      </template>
      <template #cell-age="{ value }">
        <span class="text-zinc-500 dark:text-zinc-400">
          {{ formatAge(value) }}
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
      title="Delete volume?"
      :description="deleteDescription"
      confirm-label="Delete"
      :loading="deleteLoading"
      :error="deleteError"
      @confirm="confirmDelete"
    />
  </div>
</template>
