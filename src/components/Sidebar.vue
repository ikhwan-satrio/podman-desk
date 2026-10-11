<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import ThemeToggle from "@components/ThemeToggle.vue";

interface SidebarItem {
  to: string;
  label: string;
  group?: string;
  icon: string[];
  badge?: string;
}

const defaultItems: SidebarItem[] = [
  {
    to: "/containers",
    label: "Containers",
    group: "Resources",
    icon: [
      "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z",
      "m3.3 7 8.7 5 8.7-5",
      "M12 22V12",
    ],
    badge: "3",
  },
  {
    to: "/pods",
    label: "Pods",
    group: "Resources",
    icon: [
      "m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z",
      "m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65",
      "m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65",
    ],
  },
  {
    to: "/images",
    label: "Images",
    group: "Resources",
    icon: [
      "M5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2Z",
      "M10.5 9a1.5 1.5 0 1 1-3 0 1.5 1.5 0 1 1 3 0Z",
      "m21 15-3.1-3.1a2 2 0 0 0-2.8 0L6 21",
    ],
  },
  {
    to: "/volumes",
    label: "Volumes",
    group: "Resources",
    icon: [
      "M21 5c0 1.66-3.58 3-9 3S3 6.66 3 5s3.58-3 9-3 9 1.34 9 3Z",
      "M3 5v14c0 1.66 3.58 3 9 3s9-1.34 9-3V5",
      "M3 12c0 1.66 3.58 3 9 3s9-1.34 9-3",
    ],
  },
  {
    to: "/networks",
    label: "Networks",
    group: "Resources",
    icon: [
      "M21 5a3 3 0 1 1-6 0 3 3 0 1 1 6 0Z",
      "M9 12a3 3 0 1 1-6 0 3 3 0 1 1 6 0Z",
      "M21 19a3 3 0 1 1-6 0 3 3 0 1 1 6 0Z",
      "m8.6 13.5 6.8 4",
      "m15.4 6.5-6.8 4",
    ],
  },
  {
    to: "/settings",
    label: "Settings",
    group: "System",
    icon: [
      "M4 21v-7",
      "M4 10V3",
      "M12 21v-9",
      "M12 8V3",
      "M20 21v-5",
      "M20 12V3",
      "M1 14h6",
      "M9 8h6",
      "M17 16h6",
    ],
  },
];

const props = withDefaults(
  defineProps<{
    items?: SidebarItem[];
    active?: string;
    title?: string;
    subtitle?: string;
    status?: "connected" | "connecting" | "disconnected";
    event?: string;
    detail?: string;
  }>(),
  {
    active: undefined,
    title: "Podman Desk",
    subtitle: "Local machine",
    status: "connected",
    event: undefined,
    detail: undefined,
  },
);

const route = useRoute();

const menu = computed(() => props.items ?? defaultItems);

const currentPath = computed(() => props.active ?? route.path);

const isActive = (to: string) =>
  to === "/"
    ? currentPath.value === "/"
    : currentPath.value === to || currentPath.value.startsWith(`${to}/`);

const groups = computed(() => {
  const map = new Map<string, SidebarItem[]>();
  for (const item of menu.value) {
    const key = item.group ?? "Menu";
    const list = map.get(key);
    if (list) list.push(item);
    else map.set(key, [item]);
  }
  return [...map.entries()].map(([label, children]) => ({ label, children }));
});

const statusText = computed(
  () =>
    ({
      connected: "Podman socket connected",
      connecting: "Connecting to Podman…",
      disconnected: "Podman socket unavailable",
    })[props.status],
);

const statusDot = computed(
  () =>
    ({
      connected: "bg-emerald-500",
      connecting: "bg-amber-500 animate-pulse",
      disconnected: "bg-rose-500",
    })[props.status],
);
</script>

<template>
  <aside
    class="flex h-full w-60 shrink-0 select-none flex-col border-r border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
  >
    <div class="flex items-center gap-2.5 border-b border-zinc-200 px-4 py-3.5 dark:border-zinc-800">
      <span
        class="flex h-8 w-8 items-center justify-center rounded-lg bg-sky-500/10 text-sky-500"
        aria-hidden="true"
      >
        <svg
          class="h-5 w-5"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.75"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path
            d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"
          />
          <path d="m3.3 7 8.7 5 8.7-5" />
          <path d="M12 22V12" />
        </svg>
      </span>
      <RouterLink to="/" class="min-w-0 leading-tight">
        <span class="block truncate text-sm font-semibold text-zinc-900 dark:text-zinc-50">
          {{ title }}
        </span>
        <span class="block truncate text-xs text-zinc-500 dark:text-zinc-400">
          {{ subtitle }}
        </span>
      </RouterLink>
    </div>

    <nav class="flex-1 space-y-6 overflow-y-auto px-3 py-4" aria-label="Main">
      <section v-for="group in groups" :key="group.label">
        <h2
          class="px-3 pb-1 text-[11px] font-medium uppercase tracking-wider text-zinc-400 dark:text-zinc-500"
        >
          {{ group.label }}
        </h2>
        <ul class="space-y-0.5">
          <li v-for="item in group.children" :key="item.to">
            <RouterLink
              :to="item.to"
              class="relative flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-500"
              :class="
                isActive(item.to)
                  ? 'bg-zinc-100 font-medium text-zinc-900 before:absolute before:left-0 before:top-1/2 before:h-4 before:w-0.5 before:-translate-y-1/2 before:rounded-full before:bg-sky-500 dark:bg-zinc-800 dark:text-zinc-50'
                  : 'text-zinc-600 hover:bg-zinc-50 hover:text-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800/60 dark:hover:text-zinc-100'
              "
            >
              <svg
                class="h-4 w-4 shrink-0"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.75"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path v-for="(d, i) in item.icon" :key="i" :d="d" />
              </svg>
              <span class="min-w-0 flex-1 truncate">{{ item.label }}</span>
              <span
                v-if="item.badge"
                class="rounded-full bg-zinc-200 px-1.5 py-0.5 text-[11px] font-medium tabular-nums text-zinc-600 dark:bg-zinc-700 dark:text-zinc-300"
              >
                {{ item.badge }}
              </span>
            </RouterLink>
          </li>
        </ul>
      </section>
    </nav>

    <div class="space-y-1.5 border-t border-zinc-200 px-4 py-3 dark:border-zinc-800">
      <p
        v-if="event"
        class="flex items-center gap-2 text-xs text-zinc-500 dark:text-zinc-400"
        title="Perubahan terakhir yang diterima dari Podman"
      >
        <svg
          class="h-3.5 w-3.5 shrink-0 text-emerald-500"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.75"
          stroke-linecap="round"
          stroke-linejoin="round"
          aria-hidden="true"
        >
          <path d="M3 12h4l3-9 4 18 3-9h4" />
        </svg>
        <span class="truncate">{{ event }}</span>
      </p>
      <p
        class="flex items-center gap-2 text-xs text-zinc-500 dark:text-zinc-400"
        :title="detail"
      >
        <span class="h-2 w-2 shrink-0 rounded-full" :class="statusDot" aria-hidden="true" />
        <span class="truncate">{{ statusText }}</span>
      </p>
      <div class="flex items-center justify-between gap-2 pt-1">
        <span
          class="text-[11px] font-medium uppercase tracking-wider text-zinc-400 dark:text-zinc-500"
        >
          Theme
        </span>
        <ThemeToggle />
      </div>
    </div>
  </aside>
</template>
