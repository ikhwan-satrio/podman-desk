<script setup lang="ts">
import { computed, ref } from "vue";
import { useFormat } from "@/composables/useFormat";
import type { FsNode } from "@/types/images";

const props = defineProps<{
  node: FsNode;
  depth?: number;
}>();

const emit = defineEmits<{
  select: [node: FsNode];
}>();

const { formatSize } = useFormat();
const depth = computed(() => props.depth ?? 0);
const isDir = computed(() => props.node.type === "dir");
// Root terbuka supaya isi terlihat; folder di bawahnya tertutup.
const open = ref(depth.value === 0);
const toggle = () => {
  if (isDir.value) open.value = !open.value;
};

const children = computed(() => props.node.children ?? []);

const iconColor = computed(() => {
  if (props.node.type === "symlink") return "text-sky-500";
  if (isDir.value) return open.value ? "text-amber-400" : "text-amber-500";
  return "text-zinc-400 dark:text-zinc-500";
});

const sizeLabel = computed(() =>
  isDir.value ? "" : formatSize(props.node.size),
);
</script>

<template>
  <div>
    <button
      type="button"
      class="group flex h-[26px] w-full items-center gap-1.5 pr-3 text-left text-[13px] transition-colors hover:bg-zinc-100 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-sky-500 dark:hover:bg-zinc-800"
      :style="{ paddingLeft: `${8 + depth * 16}px` }"
      :title="node.path"
      @click="toggle(); emit('select', node)"
    >
      <span
        class="flex h-4 w-4 shrink-0 items-center justify-center text-zinc-400 dark:text-zinc-500"
        aria-hidden="true"
      >
        <svg
          v-if="isDir && children.length"
          class="h-3 w-3 transition-transform"
          :class="open ? 'rotate-90' : ''"
          viewBox="0 0 16 16"
          fill="currentColor"
        >
          <path d="M6 3.5 10.5 8 6 12.5z" />
        </svg>
      </span>

      <span class="flex h-4 w-4 shrink-0 items-center justify-center" :class="iconColor" aria-hidden="true">
        <svg v-if="node.type === 'symlink'" class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4">
          <path d="M6.5 9.5 9.5 6.5" />
          <path d="M8.5 4.5 10 3a2.83 2.83 0 0 1 4 4l-1.5 1.5" />
          <path d="M7.5 11.5 6 13a2.83 2.83 0 0 1-4-4l1.5-1.5" />
        </svg>
        <svg v-else-if="isDir && open" class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="currentColor">
          <path d="M1.5 3.5A1.5 1.5 0 0 1 3 2h3l1.5 1.5H13A1.5 1.5 0 0 1 14.5 5v6A1.5 1.5 0 0 1 13 12.5H3A1.5 1.5 0 0 1 1.5 11V3.5z" />
          <path d="M1.5 6h13V11A1.5 1.5 0 0 1 13 12.5H3A1.5 1.5 0 0 1 1.5 11V6z" opacity=".55" />
        </svg>
        <svg v-else-if="isDir" class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="currentColor">
          <path d="M1.5 4A1.5 1.5 0 0 1 3 2.5h3.2L7.5 4H13A1.5 1.5 0 0 1 14.5 5.5v6A1.5 1.5 0 0 1 13 13H3a1.5 1.5 0 0 1-1.5-1.5V4z" />
        </svg>
        <svg v-else class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3">
          <path d="M4 1.5h5.5L13 5v9.5H4z" />
          <path d="M9.5 1.5V5H13" />
        </svg>
      </span>

      <span class="min-w-0 flex-1 truncate" :class="isDir ? 'text-zinc-800 dark:text-zinc-200' : 'text-zinc-600 dark:text-zinc-400'">
        {{ node.name }}
        <span v-if="node.type === 'symlink' && node.link" class="text-zinc-400 dark:text-zinc-600">
          → {{ node.link }}
        </span>
      </span>

      <span
        v-if="sizeLabel"
        class="shrink-0 text-xs tabular-nums text-zinc-400 dark:text-zinc-600"
      >
        {{ sizeLabel }}
      </span>
    </button>

    <div v-if="isDir && open && children.length">
      <FsTreeNode
        v-for="child in children"
        :key="child.path"
        :node="child"
        :depth="depth + 1"
        @select="emit('select', $event)"
      />
    </div>
  </div>
</template>
