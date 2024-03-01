import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { bridgeService } from "./core/bridge.service";
import { keyboardService } from "./core/keyboard.service";
import { getCurrent } from '@tauri-apps/api/webview';
import { preventDefaultBrowserShortcuts } from "./utils";

declare global {
  let __APP_VERSION__: string
}


document.addEventListener('contextmenu', event => event.preventDefault());
document.addEventListener('keydown', preventDefaultBrowserShortcuts)

keyboardService.initialize();
bridgeService.initialize();
getCurrent().setFocus()

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

setTimeout(() => {
  bridgeService.initApplication()
  bridgeService.flushSchedule();
}, 10)