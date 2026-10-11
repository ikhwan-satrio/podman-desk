const toMs = (value: string | number | Date | null | undefined): number => {
  if (value === null || value === undefined || value === "") return Number.NaN;
  if (value instanceof Date) return value.getTime();
  if (typeof value === "number")
    return Number.isFinite(value) ? value * 1000 : Number.NaN;
  return Date.parse(value);
};

export function useFormat() {
  const formatDate = (value: string | number | Date | null | undefined) => {
    const ms = toMs(value);
    if (Number.isNaN(ms)) return "—";
    return new Date(ms).toLocaleString(undefined, {
      dateStyle: "medium",
      timeStyle: "short",
    });
  };

  const formatAge = (value: string | number | Date | null | undefined) => {
    const ms = toMs(value);
    if (Number.isNaN(ms)) return "—";

    const seconds = Math.max(0, Date.now() / 1000 - ms / 1000);
    const ago = (amount: number, unit: string) => {
      const value = Math.floor(amount);
      return `${value} ${unit}${value === 1 ? "" : "s"} ago`;
    };

    if (seconds < 60) return "less than a minute ago";
    if (seconds < 3600) return ago(seconds / 60, "minute");
    if (seconds < 86400) return ago(seconds / 3600, "hour");
    if (seconds < 604800) return ago(seconds / 86400, "day");
    if (seconds < 2678400) return ago(seconds / 604800, "week");
    if (seconds < 31536000) return ago(seconds / 2678400, "month");
    return ago(seconds / 31536000, "year");
  };

  const formatSize = (bytes: number) => {
    const units = ["B", "kB", "MB", "GB", "TB"];
    let value = bytes;
    let unit = 0;
    while (value >= 1000 && unit < units.length - 1) {
      value /= 1000;
      unit += 1;
    }
    const text =
      unit === 0
        ? String(Math.round(value))
        : String(Number(value.toPrecision(3)));
    return `${text} ${units[unit]}`;
  };

  const shortId = (id: string) => id.replace(/^sha256:/, "").slice(0, 12);

  return { formatDate, formatAge, formatSize, shortId };
}
