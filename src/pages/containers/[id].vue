<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { useAsyncState } from "@vueuse/core";
import PageHeader from "@components/PageHeader.vue";
import EmptyState from "@components/EmptyState.vue";
import { useFormat } from "@/composables/useFormat";
import { useLiveRefresh } from "@/composables/usePodmanEvents";
import type { ContainerInspect, ContainerPort } from "@/types/containers";

const route = useRoute("/containers/[id]");
const id = computed(() => String(route.params.id));

const { formatDate, formatAge, shortId } = useFormat();

const {
  state: container,
  isReady,
  error,
  execute,
} = useAsyncState<ContainerInspect | null>(
  () => invoke<ContainerInspect>("get_container", { id: id.value }),
  null,
);

useLiveRefresh(["container"], execute);

type TabId = "info" | "logs";

const tabs: { id: TabId; label: string }[] = [
  { id: "info", label: "Info" },
  { id: "logs", label: "Logs" },
];
const activeTab = ref<TabId>("info");

const {
  state: logs,
  isReady: logsReady,
  error: logsError,
  execute: executeLogs,
} = useAsyncState<string>(
  () => invoke<string>("get_container_logs", { id: id.value }),
  "",
);

const logLines = computed(() =>
  (logs.value ?? "")
    .split("\n")
    .filter((line) => line.trim() !== ""),
);

watch(activeTab, (tab) => {
  if (tab === "logs" && !logsReady.value) {
    void executeLogs();
  }
});

const refreshLogs = () => {
  void executeLogs();
};

const name = computed(() => {
  const raw = container.value?.Name ?? "";
  return raw.replace(/^\//, "") || shortId(id.value);
});

const stateClass = computed(() => {
  const status = container.value?.State?.Status;
  if (status === "running")
    return "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400";
  if (status === "exited")
    return "bg-zinc-500/10 text-zinc-600 dark:text-zinc-400";
  if (status === "paused")
    return "bg-amber-500/10 text-amber-600 dark:text-amber-400";
  return "bg-zinc-500/10 text-zinc-600 dark:text-zinc-400";
});

const summary = computed(() => {
  const data = container.value;
  if (!data) return [];

  return [
    { label: "Container ID", value: shortId(data.Id), mono: true },
    { label: "Image", value: data.Config?.Image ?? data.Image ?? "—", mono: true },
    { label: "Created", value: formatDate(data.Created), title: data.Created },
    { label: "Started", value: formatAge(data.State?.StartedAt), title: data.State?.StartedAt },
    { label: "Platform", value: data.Platform ?? "—" },
    { label: "Restart count", value: String(data.RestartCount ?? 0) },
  ];
});

const command = computed(() => {
  const data = container.value?.Config;
  if (!data) return [];
  const entrypoint = Array.isArray(data.Entrypoint)
    ? data.Entrypoint.join(" ")
    : (data.Entrypoint ?? "");
  const cmd = Array.isArray(data.Cmd) ? data.Cmd.join(" ") : "";
  const full = [entrypoint, cmd].filter(Boolean).join(" ").trim();

  return [
    { label: "Hostname", value: data.Hostname ?? "—" },
    { label: "Working dir", value: data.WorkingDir || "/" },
    { label: "User", value: data.User || "root" },
    { label: "Command", value: full || "—", mono: true },
  ];
});

const environment = computed(
  () => container.value?.Config?.Env ?? [],
);

const ports = computed<ContainerPort[]>(() => {
  const map = container.value?.NetworkSettings?.Ports;
  if (!map) return [];
  const list: ContainerPort[] = [];
  for (const bindings of Object.values(map)) {
    if (!bindings?.length) continue;
    list.push(...bindings);
  }
  return list;
});

const portLabel = (port: ContainerPort) => {
  const target = `${port.PrivatePort}/${port.Type ?? "tcp"}`;
  if (port.PublicPort) {
    return `${port.IP && port.IP !== "0.0.0.0" ? `${port.IP}:` : ""}${port.PublicPort} → ${target}`;
  }
  return target;
};

const networks = computed(() => {
  const map = container.value?.NetworkSettings?.Networks;
  if (!map) return [];
  return Object.entries(map).map(([network, detail]) => ({
    network,
    ip: detail.IPAddress || detail.GlobalIPv6Address || "—",
    gateway: detail.Gateway ?? detail.IPv6Gateway ?? "—",
  }));
});

const mounts = computed(() => container.value?.Mounts ?? []);

const labels = computed(() => {
  const map = container.value?.Config?.Labels;
  if (!map) return [];
  return Object.entries(map);
});
</script>

<template>
  <div>
    <RouterLink
      to="/containers"
      class="inline-flex items-center gap-1 text-sm text-zinc-500 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
    >
      ← Containers
    </RouterLink>

    <div class="mt-3 flex flex-wrap items-start justify-between gap-3">
      <PageHeader :title="name" :description="shortId(id)" />
      <span
        v-if="container?.State?.Status"
        class="mt-2 inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium"
        :class="stateClass"
      >
        <span class="h-1.5 w-1.5 rounded-full bg-current" aria-hidden="true" />
        {{ container.State.Status }}
      </span>
    </div>

    <div
      v-if="!isReady && !error"
      class="mt-6 rounded-xl border border-zinc-200 bg-white px-4 py-6 text-center text-sm text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
    >
      Loading container…
    </div>

    <EmptyState
      v-else-if="error"
      title="Could not load container"
      :description="String(error)"
    />

    <template v-else-if="container">
      <div
        class="mt-6 flex gap-1 border-b border-zinc-200 dark:border-zinc-800"
        role="tablist"
        aria-label="Container sections"
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
            Ports
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!ports.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No published ports.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="(port, index) in ports"
                :key="index"
                class="flex items-center justify-between gap-4 px-4 py-2.5 text-sm"
              >
                <span class="font-mono text-xs text-zinc-700 dark:text-zinc-300">
                  {{ portLabel(port) }}
                </span>
                <span
                  class="rounded-full bg-zinc-500/10 px-2 py-0.5 text-xs font-medium text-zinc-600 dark:bg-zinc-500/20 dark:text-zinc-400"
                >
                  {{ port.Type ?? "tcp" }}
                </span>
              </li>
            </ul>
          </div>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Networks
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!networks.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              Not connected to any network.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="item in networks"
                :key="item.network"
                class="flex items-center justify-between gap-4 px-4 py-3 text-sm"
              >
                <span class="font-medium text-zinc-900 dark:text-zinc-100">
                  {{ item.network }}
                </span>
                <span class="text-right text-zinc-500 dark:text-zinc-400">
                  <span class="font-mono text-xs">{{ item.ip }}</span>
                  <span v-if="item.gateway !== '—'" class="ml-2 text-xs">
                    gw {{ item.gateway }}
                  </span>
                </span>
              </li>
            </ul>
          </div>
        </section>

        <section>
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Mounts
          </h2>
          <div
            class="mt-3 rounded-xl border border-zinc-200 bg-white dark:border-zinc-800 dark:bg-zinc-900"
          >
            <p
              v-if="!mounts.length"
              class="px-4 py-3 text-sm text-zinc-500 dark:text-zinc-400"
            >
              No mounts.
            </p>
            <ul v-else class="divide-y divide-zinc-100 dark:divide-zinc-800">
              <li
                v-for="(mount, index) in mounts"
                :key="index"
                class="px-4 py-3 text-sm"
              >
                <div class="flex items-center justify-between gap-3">
                  <span class="font-medium text-zinc-900 dark:text-zinc-100">
                    {{ mount.Name || mount.Source || "—" }}
                  </span>
                  <span
                    class="rounded-full bg-zinc-500/10 px-2 py-0.5 text-xs font-medium text-zinc-600 dark:bg-zinc-500/20 dark:text-zinc-400"
                  >
                    {{ mount.Type ?? "bind" }}
                  </span>
                </div>
                <p class="mt-1 font-mono text-xs text-zinc-500 dark:text-zinc-400">
                  {{ mount.Source }} → {{ mount.Destination }}
                  <span v-if="mount.RW === false">(ro)</span>
                </p>
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
        v-show="activeTab === 'logs'"
        role="tabpanel"
        class="mt-6"
      >
        <div class="flex items-center justify-between gap-3">
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            Logs
          </h2>
          <button
            type="button"
            class="rounded-md border border-zinc-200 px-2.5 py-1 text-xs font-medium text-zinc-600 transition-colors hover:bg-zinc-50 hover:text-zinc-900 dark:border-zinc-700 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
            @click="refreshLogs"
          >
            Refresh
          </button>
        </div>

        <div
          v-if="!logsReady && !logsError"
          class="mt-3 rounded-xl border border-zinc-200 bg-white px-4 py-6 text-center text-sm text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
        >
          Loading logs…
        </div>

        <div
          v-else-if="logsError"
          class="mt-3 rounded-xl border border-zinc-200 bg-white px-4 py-6 text-center text-sm text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
        >
          {{ logsError }}
        </div>

        <div
          v-else-if="!logLines.length"
          class="mt-3 rounded-xl border border-zinc-200 bg-white px-4 py-6 text-center text-sm text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
        >
          No logs yet.
        </div>

        <div
          v-else
          class="mt-3 max-h-[32rem] overflow-auto rounded-xl border border-zinc-200 bg-zinc-950 dark:border-zinc-800"
        >
          <pre
            class="px-4 py-3 font-mono text-xs leading-relaxed whitespace-pre-wrap text-zinc-300"
          >{{ logLines.join("\n") }}</pre>
        </div>
      </div>
    </template>
  </div>
</template>
