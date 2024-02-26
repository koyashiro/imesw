import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import Box from "@mui/joy/Box";
import Divider from "@mui/joy/Divider";
import FormControl from "@mui/joy/FormControl";
import FormHelperText from "@mui/joy/FormHelperText";
import FormLabel from "@mui/joy/FormLabel";
import Option from "@mui/joy/Option";
import Select from "@mui/joy/Select";
import Stack from "@mui/joy/Stack";
import Switch from "@mui/joy/Switch";
import Typography from "@mui/joy/Typography";

const Key = {
  LeftAlt: "left_alt",
  RightAlt: "right_alt",
  LeftCtrl: "left_ctrl",
  RightCtrl: "right_ctrl",
  LeftShift: "left_shift",
  RightShift: "right_shift",
} as const;

type Key = (typeof Key)[keyof typeof Key];

const keyLabels: Record<Key, string> = {
  left_alt: "Left Alt",
  right_alt: "Right Alt",
  left_ctrl: "Left Ctrl",
  right_ctrl: "Right Ctrl",
  left_shift: "Left Shift",
  right_shift: "Right Shift",
};

function App() {
  const [isRunning, setIsRunning] = useState(false);
  const [activateKey, setActivateKey] = useState<Key>("right_alt");
  const [deactivateKey, setDeactivateKey] = useState<Key>("left_alt");

  const fetchSettings = async () => {
    type Setting = {
      isRunning: boolean;
      activateKey: Key;
      deactivateKey: Key;
    };
    const settings = await invoke<Setting>("get_setting");

    console.log(settings);

    setIsRunning(settings.isRunning);
    setActivateKey(settings.activateKey);
    setDeactivateKey(settings.deactivateKey);
  };

  useEffect(() => {
    fetchSettings();

    const unlistenFuncPromise = listen("reload_setting", fetchSettings);

    return () => {
      unlistenFuncPromise.then((unlistenFunc) => unlistenFunc());
    };
  });

  return (
    <Box sx={{ p: 2 }}>
      <Typography level="h1">imesw for Windows</Typography>

      <Stack direction="column" spacing={2} sx={{ mt: 6 }}>
        <Typography level="title-md">Features</Typography>

        <FormControl
          orientation="horizontal"
          sx={{ justifyContent: "space-between" }}
        >
          <div>
            <FormLabel>Key Monitoring</FormLabel>
            <FormHelperText sx={{ mt: 1 }}>
              Activate key monitoring and switche IME.
            </FormHelperText>
          </div>
          <Switch
            checked={isRunning}
            onChange={async (e) => {
              await invoke("set_is_running", { isRunning: e.target.checked });
              await fetchSettings();
            }}
            endDecorator={
              <Box sx={{ ml: 1 }}>{isRunning ? "Running" : "Stopped"}</Box>
            }
            sx={{ width: 120 }}
          />
        </FormControl>

        <Divider />
      </Stack>

      <Stack direction="column" spacing={2} sx={{ mt: 6 }}>
        <Typography level="title-md">Keybinds</Typography>

        <FormControl
          orientation="horizontal"
          sx={{ justifyContent: "space-between" }}
        >
          <div>
            <FormLabel>
              <Typography level="title-md">Enable IME</Typography>
            </FormLabel>
            <FormHelperText sx={{ mt: 1 }}>
              <Typography level="body-sm">
                Select the key to enable IME.
              </Typography>
            </FormHelperText>
          </div>

          <Select
            value={activateKey}
            onChange={async (_e, newKey) => {
              if (!newKey) {
                return;
              }

              await invoke("set_activate_key", { key: newKey });

              await fetchSettings();
            }}
            sx={{ width: 130, height: 40 }}
          >
            {Object.values(Key).map((key) => (
              <Option value={key} key={key}>
                {keyLabels[key as Key]}
              </Option>
            ))}
          </Select>
        </FormControl>

        <Divider />

        <FormControl
          orientation="horizontal"
          sx={{ justifyContent: "space-between" }}
        >
          <div>
            <FormLabel>
              <Typography level="title-md">Deactivate IME</Typography>
            </FormLabel>
            <FormHelperText sx={{ mt: 1 }}>
              Select the key to disable IME.
            </FormHelperText>
          </div>
          <Select
            value={deactivateKey}
            onChange={async (_e, newKey) => {
              if (!newKey) {
                return;
              }

              await invoke("set_deactivate_key", { key: newKey });

              await fetchSettings();
            }}
            sx={{ width: 130, height: 40 }}
          >
            {Object.values(Key).map((key) => (
              <Option value={key} key={key}>
                {keyLabels[key as Key]}
              </Option>
            ))}
          </Select>
        </FormControl>

        <Divider />
      </Stack>
    </Box>
  );
}

export default App;
