import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { ToggleSwitch } from "flowbite-react";

function App() {
  const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    (async () => {
      setEnabled(await invoke<boolean>("get_enabled"));
    })();
  }, []);

  return (
    <div className="h-screen bg-red-50">
      <div className="container mx-auto max-w-lg">
        <h1 className="text-4xl font-bold">alt-ime</h1>

        <hr className="mt-3" />

        <div className="mt-2">
          <div className="flex flex-row items-center justify-between">
            <div>
              <div>
                <h2 className="text-xl font-semibold">Altキーの監視</h2>
              </div>
              <div className="text-gray-700">
                Altキーを監視してIMEを制御する。
              </div>
            </div>
            <div className="mt-2">
              <ToggleSwitch
                checked={enabled}
                label={""}
                onChange={async (v) => {
                  await invoke("set_enabled", { v });
                  setEnabled(v);
                }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
