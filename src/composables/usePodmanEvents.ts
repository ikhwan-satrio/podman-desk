import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useDebounceFn } from "@vueuse/core";
import { onScopeDispose, ref } from "vue";
import type { PodmanEvent, PodmanEventsStatus } from "@/types/events";

const HISTORY_LIMIT = 100;

const status = ref<PodmanEventsStatus>({
  connected: false,
  socket: null,
  detail: null,
});
const latest = ref<PodmanEvent | null>(null);
const history = ref<PodmanEvent[]>([]);

const eventHandlers = new Set<(event: PodmanEvent) => void>();
const statusHandlers = new Set<(status: PodmanEventsStatus) => void>();

let listener: Promise<void> | null = null;

function applyStatus(payload: PodmanEventsStatus) {
  status.value = payload;
  statusHandlers.forEach((handler) => {
    handler(payload);
  });
}

function ensureListening(): Promise<void> {
  listener ??= (async () => {
    await Promise.all([
      listen<PodmanEvent>("podman-event", ({ payload }) => {
        latest.value = payload;
        history.value = [payload, ...history.value].slice(0, HISTORY_LIMIT);
        eventHandlers.forEach((handler) => {
          handler(payload);
        });
      }),
      listen<PodmanEventsStatus>("podman-events-status", ({ payload }) => {
        applyStatus(payload);
      }),
    ]);

    // Stream mungkin sudah tersambung sebelum listener ini terpasang,
    // jadi status terkini diminta langsung ke backend.
    try {
      applyStatus(await invoke<PodmanEventsStatus>("events_status"));
    } catch (err) {
      console.error("gagal membaca status event Podman:", err);
    }
  })();

  return listener;
}

export function usePodmanEvents() {
  void ensureListening();

  return { status, latest, history };
}

/**
 * Daftarkan handler untuk event dengan `kind` tertentu. Daftar kosong berarti
 * semua jenis event. Handler dilepas otomatis lewat fungsi pengembali.
 */
export function onPodmanEvent(
  kinds: string[],
  handler: (event: PodmanEvent) => void,
): () => void {
  void ensureListening();

  const filter = kinds.length > 0 ? new Set(kinds) : null;
  const wrapped = (event: PodmanEvent) => {
    if (!filter || filter.has(event.kind)) handler(event);
  };

  eventHandlers.add(wrapped);
  return () => {
    eventHandlers.delete(wrapped);
  };
}

export function onEventsStatus(
  handler: (status: PodmanEventsStatus) => void,
): () => void {
  void ensureListening();

  statusHandlers.add(handler);
  return () => {
    statusHandlers.delete(handler);
  };
}

export interface LiveRefreshOptions {
  debounceMs?: number;
  /** Dipakai sebagai cadangan hanya ketika stream event belum tersambung. */
  pollMs?: number;
}

/**
 * Perbarui data halaman dari stream event Podman: banyak event berturut-turut
 * (mis. `die` lalu `stop`) dirangkum jadi satu fetch, dan sambungan yang baru
 * pulih langsung memicu fetch ulang karena event selama putus mungkin terlewat.
 */
export function useLiveRefresh(
  kinds: string[],
  refresh: () => unknown,
  options: LiveRefreshOptions = {},
) {
  const { debounceMs = 250, pollMs = 3000 } = options;
  const run = useDebounceFn(() => {
    void refresh();
  }, debounceMs);

  const stopEvents = onPodmanEvent(kinds, run);
  const stopStatus = onEventsStatus((value) => {
    if (value.connected) run();
  });

  const fallback = setInterval(() => {
    if (!status.value.connected) run();
  }, pollMs);

  onScopeDispose(() => {
    stopEvents();
    stopStatus();
    clearInterval(fallback);
  });

  return { status, latest };
}
