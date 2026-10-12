import { ref } from "vue";

export function useAction() {
  const busy = ref<string | null>(null);
  const error = ref<string | null>(null);

  async function run(key: string, action: () => Promise<unknown>) {
    if (busy.value) return false;
    busy.value = key;
    error.value = null;
    try {
      await action();
      return true;
    } catch (e) {
      error.value = String(e);
      return false;
    } finally {
      busy.value = null;
    }
  }

  return { busy, error, run };
}
