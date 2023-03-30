import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";

export function useEnabled(): [boolean, (enabled: boolean) => Promise<void>] {
  const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    (async () => setEnabled(await invoke<boolean>("get_enabled")))();
  }, []);

  return [
    enabled,
    async (v: boolean) => {
      await invoke("set_enabled", { v });
      setEnabled(v);
    },
  ];
}
