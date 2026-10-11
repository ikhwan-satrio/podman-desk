<script setup lang="ts">
import PageHeader from "@components/PageHeader.vue";
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
];
</script>

<template>
  <div>
    <PageHeader
      title="Volumes"
      description="Persistent data stored outside of containers."
    />

    <DataTable
      :columns="columns"
      :data="volumes ?? []"
      :row-id="(v: VolumeSummary) => v.Name"
      :loading="!isReady && !error"
      loading-text="Loading images…"
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
    </DataTable>
  </div>
</template>
