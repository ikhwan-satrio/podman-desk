<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, onUnmounted, ref } from "vue";

const win = getCurrentWindow();
const isMaximized = ref(false);

async function refreshMaximized() {
  try {
    isMaximized.value = await win.isMaximized();
  } catch {
    /* window mungkin sudah tertutup */
  }
}

let unlisten: (() => void) | undefined;

onMounted(async () => {
  await refreshMaximized();
  unlisten = await win.onResized(() => {
    void refreshMaximized();
  });
});

onUnmounted(() => unlisten?.());
</script>

<template>
  <header
    class="flex h-9 shrink-0 select-none items-center border-b border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
  >
    <div
      data-tauri-drag-region
      class="flex h-full flex-1 items-center gap-2 pl-3"
    >
      <span
        class="flex h-4 w-4 items-center justify-center text-sky-500"
        aria-hidden="true"
      >
        <svg
          class="h-3.5 w-3.5"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" />
          <path d="m3.3 7 8.7 5 8.7-5" />
          <path d="M12 22V12" />
        </svg>
      </span>
      <span
        class="truncate text-xs font-medium text-zinc-500 dark:text-zinc-400"
      >
        Podman Desk
      </span>
    </div>

    <div class="flex h-full">
      <button
        type="button"
        class="flex h-full w-11 items-center justify-center text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
        aria-label="Minimize"
        @click="win.minimize()"
      >
        <svg
          class="h-3.5 w-3.5"
          viewBox="0 0 12 12"
          fill="none"
          stroke="currentColor"
          stroke-width="1.25"
        >
          <path d="M2 6h8" />
        </svg>
      </button>

      <button
        type="button"
        class="flex h-full w-11 items-center justify-center text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
        :aria-label="isMaximized ? 'Restore' : 'Maximize'"
        @click="win.toggleMaximize()"
      >
        <svg
          v-if="!isMaximized"
          class="h-3.5 w-3.5"
          viewBox="0 0 12 12"
          fill="none"
          stroke="currentColor"
          stroke-width="1.25"
        >
          <rect x="2.5" y="2.5" width="7" height="7" rx="0.5" />
        </svg>
        <svg
          v-else
          class="h-3.5 w-3.5"
          viewBox="0 0 12 12"
          fill="none"
          stroke="currentColor"
          stroke-width="1.25"
        >
          <rect x="2.5" y="4.5" width="5" height="5" rx="0.5" />
          <path d="M4.5 4.5v-2h5v5h-2" />
        </svg>
      </button>

      <button
        type="button"
        class="flex h-full w-11 items-center justify-center text-zinc-500 transition-colors hover:bg-rose-500 hover:text-white dark:text-zinc-400"
        aria-label="Close"
        @click="win.close()"
      >
        <svg
          class="h-3.5 w-3.5"
          viewBox="0 0 12 12"
          fill="none"
          stroke="currentColor"
          stroke-width="1.25"
          stroke-linecap="round"
        >
          <path d="M3 3l6 6M9 3l-6 6" />
        </svg>
      </button>
    </div>
  </header>
</template>
