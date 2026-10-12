<script setup lang="ts">
import { computed, ref } from "vue";
import PageHeader from "@components/PageHeader.vue";
import DataTable from "@components/DataTable.vue";
import type { DataTableColumn } from "@components/DataTable.vue";
import ConfirmDialog from "@components/ConfirmDialog.vue";
import RunImageDialog from "@components/RunImageDialog.vue";
import RowActions from "@components/RowActions.vue";
import type { RowAction } from "@components/RowActions.vue";
import BulkActionsBar from "@components/BulkActionsBar.vue";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import { useFormat } from "@/composables/useFormat";
import { ImageSummary } from "@/types/images";
import { useLiveRefresh } from "@/composables/usePodmanEvents";

const { formatDate, formatAge, formatSize, shortId } = useFormat();

const {
  state: images,
  isReady,
  error,
  execute,
} = useAsyncState<ImageSummary[]>(() => invoke("get_images"), []);

useLiveRefresh(["image"], execute);

const NONE = "<none>";

const imageRef = (image: ImageSummary) =>
  image.RepoTags?.find((item) => item && !item.startsWith(NONE));

const repo = (image: ImageSummary) => {
  const ref = imageRef(image);
  if (!ref) return NONE;
  const lastSlash = ref.lastIndexOf("/");
  const lastColon = ref.lastIndexOf(":");
  return lastColon > lastSlash ? ref.slice(0, lastColon) : ref;
};

const tag = (image: ImageSummary) => {
  const ref = imageRef(image);
  if (!ref) return NONE;
  const lastSlash = ref.lastIndexOf("/");
  const lastColon = ref.lastIndexOf(":");
  return lastColon > lastSlash ? ref.slice(lastColon + 1) : NONE;
};

const columns: DataTableColumn<ImageSummary>[] = [
  { id: "repository", header: "Repository", accessorFn: repo },
  { id: "tag", header: "Tag", accessorFn: tag },
  { id: "id", header: "Image ID", accessorFn: (image) => shortId(image.Id) },
  { id: "created", header: "Created", accessorFn: (image) => image.Created },
  { id: "size", header: "Size", accessorFn: (image) => formatSize(image.Size) },
  { id: "actions", header: "", accessorFn: () => "" },
];

const runOpen = ref(false);
const runTarget = ref<ImageSummary | null>(null);

const openRun = (image: ImageSummary) => {
  runTarget.value = image;
  runOpen.value = true;
};

const deleteOpen = ref(false);
const deleteTarget = ref<ImageSummary | null>(null);
const deleteLoading = ref(false);
const deleteError = ref<string | undefined>(undefined);

const openDelete = (image: ImageSummary) => {
  bulkDelete.value = false;
  deleteTarget.value = image;
  deleteError.value = undefined;
  deleteOpen.value = true;
};

const selected = ref<string[]>([]);

const selectedRows = computed(() => {
  const ids = new Set(selected.value);
  return (images.value ?? []).filter((image) => ids.has(image.Id));
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
    return `${selectedRows.value.length} images will be permanently removed.`;
  }
  if (deleteTarget.value) {
    return `Image ${imageRef(deleteTarget.value) ?? shortId(deleteTarget.value.Id)} will be permanently removed.`;
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
  const rows: ImageSummary[] = bulkDelete.value
    ? selectedRows.value
    : deleteTarget.value
      ? [deleteTarget.value]
      : [];
  if (!rows.length) return;
  deleteLoading.value = true;
  deleteError.value = undefined;
  try {
    for (const row of rows) {
      await invoke("remove_image", { id: row.Id, force: true });
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

const rowActions = (image: ImageSummary): RowAction[] => [
  { label: "Run", onSelect: () => openRun(image) },
  { label: "Delete", danger: true, separatorBefore: true, onSelect: () => openDelete(image) },
];
</script>

<template>
  <div>
    <PageHeader
      title="Images"
      description="Local container images available to Podman."
    />

    <BulkActionsBar
      :count="selected.length"
      :actions="bulkItems"
      noun="images selected"
      @clear="selected = []"
    />

    <DataTable
      v-model:selected="selected"
      :selectable="true"
      :columns="columns"
      :data="images ?? []"
      :row-id="(image: ImageSummary) => image.Id"
      :loading="!isReady && !error"
      loading-text="Loading images…"
      :empty-title="error ? 'Could not load images' : 'No images found'"
      :empty-description="
        error
          ? 'Check that the Podman socket is available, then try again.'
          : 'Pull an image to get started.'
      "
    >
      <template #cell-repository="{ value, row }">
        <RouterLink
          v-if="value !== NONE"
          :to="{ path: `/images/${row.Id}` }"
          class="font-medium text-sky-600 hover:underline dark:text-sky-400"
        >
          {{ value }}
        </RouterLink>
        <span v-else class="text-zinc-400 dark:text-zinc-500">{{ value }}</span>
      </template>
      <template #cell-tag="{ value }">
        <span
          :class="
            value === NONE
              ? 'text-zinc-400 dark:text-zinc-500'
              : 'text-zinc-600 dark:text-zinc-400'
          "
        >
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
      <template #cell-actions="{ row }">
        <div class="flex items-center justify-end">
          <RowActions :items="rowActions(row)" />
        </div>
      </template>
    </DataTable>

    <RunImageDialog
      v-if="runTarget"
      v-model:open="runOpen"
      :image-ref="imageRef(runTarget) ?? runTarget.Id"
      @started="execute()"
    />

    <ConfirmDialog
      v-if="deleteTarget || bulkDelete"
      v-model:open="deleteOpen"
      title="Delete image?"
      :description="deleteDescription"
      confirm-label="Delete"
      :loading="deleteLoading"
      :error="deleteError"
      @confirm="confirmDelete"
    />
  </div>
</template>
