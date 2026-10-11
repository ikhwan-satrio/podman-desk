<script setup lang="ts">
import { computed, ref } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import PageHeader from "@components/PageHeader.vue";
import EmptyState from "@components/EmptyState.vue";
import { useFormat } from "@/composables/useFormat";
import { useLiveRefresh } from "@/composables/usePodmanEvents";
import type { ImageHistoryItem, ImageInspect } from "@/types/images";

const route = useRoute("/images/[id]");
const id = computed(() => String(route.params.id));

const { formatDate, formatAge, formatSize, shortId } = useFormat();

const {
  state: image,
  isReady,
  error,
  execute,
} = useAsyncState<ImageInspect | null>(
  () => invoke<ImageInspect>("get_image", { id: id.value }),
  null,
);

const {
  state: history,
  isReady: historyReady,
} = useAsyncState<ImageHistoryItem[]>(
  () => invoke<ImageHistoryItem[]>("get_image_history", { id: id.value }),
  [],
);

useLiveRefresh(["image"], execute);

type TabId = "info" | "filesystem";

const tabs: { id: TabId; label: string }[] = [
  { id: "info", label: "Info" },
  { id: "filesystem", label: "Filesystem" },
];
const activeTab = ref<TabId>("info");

const title = computed(() => {
  const tag = image.value?.RepoTags?.find((item) => item && !item.startsWith("<none>"));
  if (!tag) return shortId(id.value);
  const lastSlash = tag.lastIndexOf("/");
  const lastColon = tag.lastIndexOf(":");
  return lastColon > lastSlash ? tag.slice(0, lastColon) : tag;
});

const summary = computed(() => {
  const data = image.value;
  if (!data) return [];

  return [
    { label: "Image ID", value: shortId(data.Id), mono: true },
    { label: "Tags", value: data.RepoTags?.filter((t) => t && !t.startsWith("<none>")).join(", ") || "—" },
    { label: "Created", value: formatDate(data.Created), title: data.Created },
    { label: "Age", value: formatAge(data.Created) },
    { label: "Size", value: formatSize(data.Size ?? 0) },
    { label: "Architecture", value: data.Architecture ?? "—" },
    { label: "OS", value: data.Os ?? "—" },
    { label: "Docker version", value: data.DockerVersion ?? "—" },
    { label: "Author", value: data.Author ?? "—" },
  ];
});

const command = computed(() => {
  const data = image.value?.Config;
  if (!data) return [];
  const entrypoint = Array.isArray(data.Entrypoint)
    ? data.Entrypoint.join(" ")
    : (data.Entrypoint ?? "");
  const cmd = Array.isArray(data.Cmd) ? data.Cmd.join(" ") : "";
  const full = [entrypoint, cmd].filter(Boolean).join(" ").trim();

  return [
    { label: "Working dir", value: data.WorkingDir || "/" },
    { label: "User", value: data.User || "root" },
    { label: "Command", value: full || "—", mono: true },
  ];
});

const environment = computed(() => image.value?.Config?.Env ?? []);

const labels = computed(() => {
  const map = image.value?.Config?.Labels;
  if (!map) return [];
  return Object.entries(map);
});

const digests = computed(() => image.value?.RepoDigests ?? []);

const layers = computed(() => image.value?.RootFS?.Layers ?? []);

const fsLayers = computed(() =>
  (history.value ?? [])
    .slice()
    .sort((a, b) => (b.Created ?? 0) - (a.Created ?? 0)),
);

const layerCommand = (createdBy: string | undefined) => {
  if (!createdBy) return "—";
  return createdBy.replace(/^\/bin\/sh\s+-c\s+#\s*\(\s*nop\s*\)\s*/i, "").trim() || createdBy;
};
</script>

<template>
  <div>
    <RouterLink
      to="/images"
      class="inline-flex items-center gap-1 text-sm text-zinc-500 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
    >
      ← Images
    </RouterLink>

    <div class="mt-3">
      <PageHeader :title="title" :description="shortId(id)" />
    </div>

    <div
      v-if="!isReady && !error"
      class="mt-6 rounded-xl border border-zinc-200 bg-white px-4 py-6 text-center text-sm text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
    >
      Loading image…
    </div>

    <EmptyState
      v-else-if="error"
      title="Could not load image"
      :description="String(error)"
    />

    <template v-else-if="image">
      <div
        class="mt-6 flex gap-1 border-b border-zinc-200 dark:border-zinc-800"
        role="tablist"
        aria-label="Image sections"
      >
        <button
          v-for="tab in tabs"
          :key="tab.id"
          type="button"
          role="tab"
          class="relative px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-500"
          :aria-selected="activeTab === tab.id"
          :class="
            activeTab === tab.id
              ? 'text-zinc-900 dark:text-zinc-50'
              : 'text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100'
          "
          @click="activeTab = tab.id"
        >
          {{ tab.label }}
          <span
            v-if="activeTab === tab.id"
            class="absolute inset-x-0 -bottom-px h-0.5 bg-sky-500"
            aria-hidden="true"
          />
        </button>
      </div>

      <div v-show="activeTab === 'info'" role="tabpanel" class="mt-6 space-y-6">
        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Summary
          </h2>
          <dl
            class="mt-3 divide-y divide-zinc-100 rounded-xl border border-zinc-200 bg-white dark:divide-zinc-800 dark:border-zinc-800 dark:bg-zinc-900"
          >
            <div
              v-for="item in summary"
              :key="item.label"
              class="flex items-center justify-between gap-4 px-4 py-3 text-sm"
            >
              <dt class="shrink-0 text-zinc-500 dark:text-zinc-400">
                {{ item.label }}
              </dt>
              <dd
                class="min-w-0 truncate text-right font-medium text-zinc-900 dark:text-zinc-100"
                :class="item.mono ? 'font-mono text-xs' : ''"
                :title="item.title"
              >
                {{ item.value }}
              </dd>
            </div>
          </dl>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Command
          </h2>
          <dl
            class="mt-3 divide-y divide-zinc-100 rounded-xl border border-zinc-200 bg-white dark:divide-zinc-800 dark:border-zinc-800 dark:bg-zinc-900"
          >
            <div
              v-for="item in command"
              :key="item.label"
              class="flex items-center justify-between gap-4 px-4 py-3 text-sm"
            >
              <dt class="shrink-0 text-zinc-500 dark:text-zinc-400">
                {{ item.label }}
              </dt>
              <dd
                class="min-w-0 truncate text-right font-medium text-zinc-900 dark:text-zinc-100"
                :class="item.mono ? 'font-mono text-xs' : ''"
              >
                {{ item.value }}
              </dd>
            </div>
          </dl>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Environment
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!environment.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No environment variables.
            </p>
            <ul
              v-else
              class="max-h-64 divide-y divide-zinc-100 overflow-y-auto dark:divide-zinc-800"
            >
              <li
                v-for="entry in environment"
                :key="entry"
                class="px-4 py-2.5 font-mono text-xs text-zinc-700 dark:text-zinc-300"
              >
                {{ entry }}
              </li>
            </ul>
          </div>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Digests
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!digests.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No digests.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="digest in digests"
                :key="digest"
                class="px-4 py-2.5 font-mono text-xs break-all text-zinc-700 dark:text-zinc-300"
              >
                {{ digest }}
              </li>
            </ul>
          </div>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Labels
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!labels.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No labels.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="[key, value] in labels"
                :key="key"
                class="flex items-start justify-between gap-4 px-4 py-2.5 text-sm"
              >
                <span class="shrink-0 font-mono text-xs text-zinc-600 dark:text-zinc-400">
                  {{ key }}
                </span>
                <span class="min-w-0 break-all text-right text-zinc-900 dark:text-zinc-100">
                  {{ value || "—" }}
                </span>
              </li>
            </ul>
          </div>
        </section>
      </div>

      <div
        v-show="activeTab === 'filesystem'"
        role="tabpanel"
        class="mt-6 space-y-6"
      >
        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Layers
          </h2>
          <p class="mt-1 text-xs text-zinc-500 dark:text-zinc-400">
            {{ layers.length }} layer dalam image ini
          </p>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!layers.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No layer info.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="(layer, index) in layers"
                :key="layer"
                class="flex items-center gap-3 px-4 py-2.5 text-sm"
              >
                <span class="w-6 shrink-0 text-right text-xs tabular-nums text-zinc-400">
                  {{ index + 1 }}
                </span>
                <span class="min-w-0 truncate font-mono text-xs text-zinc-700 dark:text-zinc-300">
                  {{ layer }}
                </span>
              </li>
            </ul>
          </div>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Build history
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!historyReady"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              Loading history…
            </p>
            <p
              v-else-if="!fsLayers.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No history.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="layer in fsLayers"
                :key="layer.Id"
                class="px-4 py-3 text-sm"
              >
                <div class="flex items-start justify-between gap-3">
                  <p class="min-w-0 flex-1 text-zinc-700 dark:text-zinc-300">
                    {{ layerCommand(layer.CreatedBy) }}
                  </p>
                  <span class="shrink-0 text-xs tabular-nums text-zinc-500 dark:text-zinc-400">
                    {{ formatSize(layer.Size ?? 0) }}
                  </span>
                </div>
                <p class="mt-1 text-xs text-zinc-400 dark:text-zinc-500">
                  {{ layer.Tags?.join(", ") || shortId(layer.Id) }}
                  <span v-if="layer.Created"> · {{ formatAge(layer.Created * 1000) }}</span>
                </p>
              </li>
            </ul>
          </div>
        </section>
      </div>
    </template>
  </div>
</template>
