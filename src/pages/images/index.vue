<script setup lang="ts">
import PageHeader from "@components/PageHeader.vue";
import DataTable from "@components/DataTable.vue";
import type { DataTableColumn } from "@components/DataTable.vue";
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
];
</script>

<template>
  <div>
    <PageHeader
      title="Images"
      description="Local container images available to Podman."
    />

    <DataTable
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
    </DataTable>
  </div>
</template>
