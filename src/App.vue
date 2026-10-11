<script setup lang="ts">
import "./App.css";
import { computed } from "vue";
import TitleBar from "@components/TitleBar.vue";
import Sidebar from "@components/Sidebar.vue";
import { usePodmanEvents } from "@/composables/usePodmanEvents";

const { status, latest } = usePodmanEvents();

const sidebarStatus = computed<"connected" | "connecting" | "disconnected">(
  () => {
    if (status.value.connected) return "connected";
    return status.value.detail ? "disconnected" : "connecting";
  },
);

const lastEvent = computed(() => {
  const event = latest.value;
  if (!event) return undefined;

  const target = event.name || event.id.slice(0, 12);
  return target ? `${event.kind} · ${event.action} · ${target}` : undefined;
});
</script>

<template>
  <div class="flex h-screen flex-col overflow-hidden bg-zinc-50 text-zinc-900 dark:bg-zinc-950 dark:text-zinc-100">
    <TitleBar />

    <div class="flex min-h-0 flex-1">
      <Sidebar
        :status="sidebarStatus"
        :event="lastEvent"
        :detail="status.detail ?? status.socket ?? undefined"
      />

      <main class="min-w-0 flex-1 overflow-y-auto">
        <div class="mx-auto max-w-5xl px-8 py-8">
          <RouterView v-slot="{ Component }">
            <Transition name="page" mode="out-in">
              <component :is="Component" />
            </Transition>
          </RouterView>
        </div>
      </main>
    </div>
  </div>
</template>

<style scoped>
.page-enter-active,
.page-leave-active {
  transition: opacity 0.15s ease;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
}
</style>
