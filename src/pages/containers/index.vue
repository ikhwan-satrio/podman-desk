<script setup lang="ts">
import { computed, ref } from "vue";
import PageHeader from "@components/PageHeader.vue";
import DataTable from "@components/DataTable.vue";
import type { DataTableColumn } from "@components/DataTable.vue";
import ConfirmDialog from "@components/ConfirmDialog.vue";
import RenameContainerDialog from "@components/RenameContainerDialog.vue";
import RowActions from "@components/RowActions.vue";
import type { RowAction } from "@components/RowActions.vue";
import BulkActionsBar from "@components/BulkActionsBar.vue";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import { ContainerSummary } from "@/types/containers";
import { useLiveRefresh } from "@/composables/usePodmanEvents";
import { useAction } from "@/composables/useAction";

const {
  state: containers,
  isReady,
  execute,
} = useAsyncState<ContainerSummary[]>(() => invoke("get_containers"), []);

useLiveRefresh(["container"], execute);

const { busy, error: actionError, run } = useAction();

const statusClass = (state: string | undefined) =>
  state === "running"
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
  { id: "actions", header: "", accessorFn: () => "" },
];

const containerName = (container: ContainerSummary) =>
  container.Names[0] ?? container.Id.slice(0, 12);

const isRunning = (container: ContainerSummary) =>
  container.State === "running";

const toggleContainer = (container: ContainerSummary) =>
  run(`${container.Id}:toggle`, async () => {
    if (isRunning(container)) {
      await invoke("stop_container", { id: container.Id });
    } else {
      await invoke("start_container", { id: container.Id });
    }
    await execute();
  });

const restartContainer = (container: ContainerSummary) =>
  run(`${container.Id}:restart`, async () => {
    await invoke("restart_container", { id: container.Id });
    await execute();
  });

const renameOpen = ref(false);
const renameTarget = ref<ContainerSummary | null>(null);

const openRename = (container: ContainerSummary) => {
  renameTarget.value = container;
  renameOpen.value = true;
};

const deleteOpen = ref(false);
const deleteTarget = ref<ContainerSummary | null>(null);
const deleteLoading = ref(false);
const deleteError = ref<string | undefined>(undefined);

const openDelete = (container: ContainerSummary) => {
  bulkDelete.value = false;
  deleteTarget.value = container;
  deleteError.value = undefined;
  deleteOpen.value = true;
};

const selected = ref<string[]>([]);

const selectedRows = computed(() => {
  const ids = new Set(selected.value);
  return (containers.value ?? []).filter((container) => ids.has(container.Id));
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
    return `${selectedRows.value.length} containers will be permanently removed.`;
  }
  if (deleteTarget.value) {
    return `Container ${containerName(deleteTarget.value)} will be permanently removed.`;
  }
  return undefined;
});

const runBulk = (
  key: string,
  filter: (container: ContainerSummary) => boolean,
  action: (container: ContainerSummary) => Promise<unknown>,
) =>
  run(`bulk:${key}`, async () => {
    for (const row of selectedRows.value.filter(filter)) {
      await action(row);
    }
    selected.value = [];
    await execute();
  });

const bulkItems = computed<RowAction[]>(() => {
  const rows = selectedRows.value;
  if (!rows.length) return [];
  const busyNow = busy.value !== null;
  const anyRunning = rows.some(isRunning);
  const anyStopped = rows.some((row) => !isRunning(row));
  const items: RowAction[] = [];
  if (anyRunning) {
    items.push({
      label: "Stop",
      disabled: busyNow,
      onSelect: () =>
        runBulk(
          "stop",
          isRunning,
          (row) => invoke("stop_container", { id: row.Id }),
        ),
    });
  }
  if (anyStopped) {
    items.push({
      label: "Start",
      disabled: busyNow,
      onSelect: () =>
        runBulk(
          "start",
          (row) => !isRunning(row),
          (row) => invoke("start_container", { id: row.Id }),
        ),
    });
  }
  if (anyRunning) {
    items.push({
      label: "Restart",
      disabled: busyNow,
      onSelect: () =>
        runBulk(
          "restart",
          isRunning,
          (row) => invoke("restart_container", { id: row.Id }),
        ),
    });
  }
  items.push({
    label: "Delete",
    danger: true,
    disabled: busyNow,
    onSelect: openBulkDelete,
  });
  return items;
});

const confirmDelete = async () => {
  const rows: ContainerSummary[] = bulkDelete.value
    ? selectedRows.value
    : deleteTarget.value
      ? [deleteTarget.value]
      : [];
  if (!rows.length) return;
  deleteLoading.value = true;
  deleteError.value = undefined;
  try {
    for (const row of rows) {
      await invoke("remove_container", { id: row.Id, force: true });
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

const rowActions = (container: ContainerSummary): RowAction[] => [
  {
    label: isRunning(container) ? "Stop" : "Start",
    disabled: busy.value !== null,
    onSelect: () => toggleContainer(container),
  },
  {
    label: "Restart",
    disabled: busy.value !== null,
    onSelect: () => restartContainer(container),
  },
  {
    separatorBefore: true,
    label: "Edit",
    onSelect: () => openRename(container),
  },
  {
    label: "Delete",
    danger: true,
    disabled: busy.value !== null,
    onSelect: () => openDelete(container),
  },
];
</script>

<template>
  <div>
    <PageHeader
      title="Containers"
      description="All containers managed by Podman."
    />

    <p
      v-if="actionError"
      class="mt-4 rounded-lg border border-rose-200 bg-rose-50 px-3 py-2 text-xs text-rose-700 dark:border-rose-900 dark:bg-rose-950 dark:text-rose-300"
    >
      {{ actionError }}
    </p>

    <BulkActionsBar
      :count="selected.length"
      :actions="bulkItems"
      :disabled="busy !== null"
      noun="containers selected"
      @clear="selected = []"
    />

    <DataTable
      v-model:selected="selected"
      :selectable="true"
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
          {{ containerName(row) }}
        </RouterLink>
      </template>
      <template #cell-image="{ value }">
        <span class="font-mono text-xs text-zinc-500 dark:text-zinc-400">{{ value }}</span>
      </template>
      <template #cell-status="{ row }">
        <span
          class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-medium"
          :class="statusClass(row.State)"
        >
          <span class="h-1.5 w-1.5 rounded-full bg-current" aria-hidden="true" />
          {{ row.Status }}
        </span>
      </template>
      <template #cell-actions="{ row }">
        <div class="flex items-center justify-end">
          <RowActions :items="rowActions(row)" />
        </div>
      </template>
    </DataTable>

    <RenameContainerDialog
      v-if="renameTarget"
      v-model:open="renameOpen"
      :container-id="renameTarget.Id"
      :initial-name="containerName(renameTarget).replace(/^\//, '')"
      @renamed="execute()"
    />

    <ConfirmDialog
      v-if="deleteTarget || bulkDelete"
      v-model:open="deleteOpen"
      title="Delete container?"
      :description="deleteDescription"
      confirm-label="Delete"
      :loading="deleteLoading"
      :error="deleteError"
      @confirm="confirmDelete"
    />
  </div>
</template>
