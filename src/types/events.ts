export interface PodmanEvent {
  /** Tipe objek yang memancarkan event: container, pod, image, volume, network, … */
  kind: string;
  /** Aksi yang terjadi: create, start, die, remove, … */
  action: string;
  id: string;
  name: string;
  attributes: Record<string, string>;
  /** Waktu event dalam milidetik. */
  timestamp: number;
}

export interface PodmanEventsStatus {
  connected: boolean;
  socket: string | null;
  detail: string | null;
}
