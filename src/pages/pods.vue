<script setup lang="ts">
import { computed, ref } from "vue";
import PageHeader from "@components/PageHeader.vue";
import ConfirmDialog from "@components/ConfirmDialog.vue";
import RowActions from "@components/RowActions.vue";
import type { RowAction } from "@components/RowActions.vue";
import BulkActionsBar from "@components/BulkActionsBar.vue";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import { useFormat } from "@/composables/useFormat";
import { PodSummary } from "@/types/pods";
import DataTable, { DataTableColumn } from "@/components/DataTable.vue";
import { useLiveRefresh } from "@/composables/usePodmanEvents";
import { useAction } from "@/composables/useAction";

const { formatDate, formatAge, shortId } = useFormat();

const {
  state: pods,
  isReady,
  error,
  execute,
} = useAsyncState<PodSummary[]>(() => invoke("get_pods"), []);

useLiveRefresh(["pod", "container"], execute);

const { busy, error: actionError, run } = useAction();

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
  { id: "actions", header: "", accessorFn: () => "" },
];

const isRunning = (pod: PodSummary) => pod.Status === "Running";

const podAction = (pod: PodSummary, action: string) =>
  run(`${pod.Id}:${action}`, async () => {
    await invoke("podman_action", { id: pod.Id, action });
    await execute();
  });

const deleteOpen = ref(false);
const deleteTarget = ref<PodSummary | null>(null);
const deleteLoading = ref(false);
const deleteError = ref<string | undefined>(undefined);

const openDelete = (pod: PodSummary) => {
  bulkDelete.value = false;
  deleteTarget.value = pod;
  deleteError.value = undefined;
  deleteOpen.value = true;
};

const selected = ref<string[]>([]);

const selectedRows = computed(() => {
  const ids = new Set(selected.value);
  return (pods.value ?? []).filter((pod) => ids.has(pod.Id ?? ""));
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
    return `${selectedRows.value.length} pods and their containers will be permanently removed.`;
  }
  if (deleteTarget.value) {
    return `Pod ${deleteTarget.value.Name} and its containers will be permanently removed.`;
  }
  return undefined;
});

const runBulk = (
  key: string,
  filter: (pod: PodSummary) => boolean,
  action: (pod: PodSummary) => Promise<unknown>,
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
        runBulk("stop", isRunning, (row) =>
          invoke("podman_action", { id: row.Id, action: "stop" }),
        ),
    });
  }
  if (anyStopped) {
    items.push({
      label: "Start",
      disabled: busyNow,
      onSelect: () =>
        runBulk("start", (row) => !isRunning(row), (row) =>
          invoke("podman_action", { id: row.Id, action: "start" }),
        ),
    });
  }
  if (anyRunning) {
    items.push({
      label: "Restart",
      disabled: busyNow,
      onSelect: () =>
        runBulk("restart", isRunning, (row) =>
          invoke("podman_action", { id: row.Id, action: "restart" }),
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
  const rows: PodSummary[] = bulkDelete.value
    ? selectedRows.value
    : deleteTarget.value
      ? [deleteTarget.value]
      : [];
  if (!rows.length) return;
  deleteLoading.value = true;
  deleteError.value = undefined;
  try {
    for (const row of rows) {
      await invoke("podman_action", { id: row.Id, action: "remove" });
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

const rowActions = (pod: PodSummary): RowAction[] => [
  {
    label: isRunning(pod) ? "Stop" : "Start",
    disabled: busy.value !== null,
    onSelect: () => podAction(pod, isRunning(pod) ? "stop" : "start"),
  },
  {
    label: "Restart",
    disabled: busy.value !== null,
    onSelect: () => podAction(pod, "restart"),
  },
  {
    separatorBefore: true,
    label: "Delete",
    danger: true,
    disabled: busy.value !== null,
    onSelect: () => openDelete(pod),
  },
];
</script>

<template>
  <div>
    <PageHeader
      title="Pods"
      description="Groups of containers sharing the same network namespace."
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
      noun="pods selected"
      @clear="selected = []"
    />

    <DataTable
      v-model:selected="selected"
      :selectable="true"
      :columns="columns"
      :data="pods ?? []"
      :row-id="(pods: PodSummary) => pods.Id"
      :loading="!isReady && !error"
      loading-text="Loading pods…"
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
      <template #cell-actions="{ row }">
        <div class="flex items-center justify-end">
          <RowActions :items="rowActions(row)" />
        </div>
      </template>
    </DataTable>

    <ConfirmDialog
      v-if="deleteTarget || bulkDelete"
      v-model:open="deleteOpen"
      title="Delete pod?"
      :description="deleteDescription"
      confirm-label="Delete"
      :loading="deleteLoading"
      :error="deleteError"
      @confirm="confirmDelete"
    />
  </div>
</template>
