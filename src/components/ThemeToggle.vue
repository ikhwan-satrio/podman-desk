<script setup lang="ts">
import { useColorMode } from "@vueuse/core";

type ThemeMode = "light" | "dark" | "auto";

const colorMode = useColorMode<ThemeMode>({
  attribute: "class",
  initialValue: "auto",
  storageKey: "podman-desk-color-mode",
  emitAuto: true,
});

const options: { value: ThemeMode; label: string; icon: string[] }[] = [
  {
    value: "light",
    label: "Light",
    icon: [
      "M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Z",
      "M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M6.3 17.7l-1.4 1.4M19.1 4.9l-1.4 1.4",
    ],
  },
  {
    value: "dark",
    label: "Dark",
    icon: ["M20.5 14.3A8.5 8.5 0 1 1 9.7 3.5a6.5 6.5 0 0 0 10.8 10.8Z"],
  },
  {
    value: "auto",
    label: "System",
    icon: ["M3 5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5Z", "M8 21h8M12 17v4"],
  },
];

const isActive = (value: ThemeMode) => colorMode.value === value;

const select = (value: ThemeMode) => {
  colorMode.value = value;
};
</script>

<template>
  <div
    role="group"
    aria-label="Theme"
    class="flex items-center gap-0.5 rounded-md border border-zinc-200 p-0.5 dark:border-zinc-700"
  >
    <button
      v-for="option in options"
      :key="option.value"
      type="button"
      class="rounded p-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-sky-500"
      :class="
        isActive(option.value)
          ? 'bg-zinc-100 text-zinc-900 dark:bg-zinc-800 dark:text-zinc-50'
          : 'text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100'
      "
      :title="option.label"
      :aria-label="option.label"
      :aria-pressed="isActive(option.value)"
      @click="select(option.value)"
    >
      <svg
        class="h-3.5 w-3.5"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="1.75"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <path v-for="(d, i) in option.icon" :key="i" :d="d" />
      </svg>
    </button>
  </div>
</template>
