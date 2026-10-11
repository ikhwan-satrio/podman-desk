<script lang="ts">
import type { ColumnDef, RowData } from "@tanstack/vue-table";

export type DataTableColumn<T extends RowData> = ColumnDef<any, T, any>;
</script>

<script setup lang="ts" generic="T extends RowData">
import { toRef } from "vue";
import { FlexRender, tableFeatures, useTable } from "@tanstack/vue-table";
import EmptyState from "./EmptyState.vue";

const props = withDefaults(
  defineProps<{
    columns: DataTableColumn<T>[];
    data: T[];
    rowId?: (row: T, index: number) => string;
    loading?: boolean;
    loadingText?: string;
    emptyTitle?: string;
    emptyDescription?: string;
  }>(),
  {
    rowId: undefined,
    loading: false,
    loadingText: "Loading…",
    emptyTitle: "No results",
    emptyDescription: undefined,
  },
);

const features = tableFeatures({});

defineSlots<{
  empty?: () => any;
  [name: `cell-${string}`]: (props: { row: T; value: any; cell: any }) => any;
  [name: `header-${string}`]: (props: { header: any }) => any;
}>();

const table = useTable({
  features,
  columns: toRef(props, "columns"),
  data: toRef(props, "data"),
  getRowId: (row: T, index: number) => props.rowId?.(row, index) ?? String(index),
});
</script>

<template>
  <div
    v-if="loading"
    class="mt-6 rounded-xl border border-zinc-200 bg-white px-4 py-6 text-center text-sm text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
  >
    {{ loadingText }}
  </div>

  <EmptyState
    v-else-if="!data.length"
    :title="emptyTitle"
    :description="emptyDescription"
  >
    <template v-if="$slots.empty" #default>
      <slot name="empty" />
    </template>
  </EmptyState>

  <div
    v-else
    class="mt-6 overflow-hidden rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
  >
    <table class="w-full text-left text-sm">
      <thead
        class="border-b border-zinc-200 text-xs uppercase tracking-wide text-zinc-500 dark:border-zinc-800 dark:text-zinc-400"
      >
        <tr
          v-for="headerGroup in table.getHeaderGroups()"
          :key="headerGroup.id"
        >
          <th
            v-for="header in headerGroup.headers"
            :key="header.id"
            class="px-4 py-3 font-medium"
          >
            <slot
              :name="`header-${header.column.id}`"
              :header="header"
            >
              <FlexRender v-if="!header.isPlaceholder" :header="header" />
            </slot>
          </th>
        </tr>
      </thead>
      <tbody class="divide-y divide-zinc-100 dark:divide-zinc-800">
        <tr
          v-for="row in table.getRowModel().rows"
          :key="row.id"
          class="transition-colors hover:bg-zinc-50 dark:hover:bg-zinc-800/50"
        >
          <td
            v-for="cell in row.getAllCells()"
            :key="cell.id"
            class="px-4 py-3"
          >
            <slot
              :name="`cell-${cell.column.id}`"
              :row="cell.row.original"
              :value="cell.getValue()"
              :cell="cell"
            >
              <FlexRender v-if="cell.column.columnDef.cell" :cell="cell" />
              <template v-else>{{ cell.getValue() }}</template>
            </slot>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
