import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { InvokeReturn, ShortcutInfo } from "./type";
import { viewService } from "./view.service";
import { resourceService } from "./resource.service";

export class BridgeService {
    constructor() {
    }

    public initialize() {
        listen("signal:schedule", () => {
            this.flushSchedule();
        })
    }

    public initApplication() {
        this._invoke("init_application", null)
    }

    public change_clipboard_active(id: number) {
        this._invoke('change_clipboard_active', { id });
    }
    public handle_keydown(key: string) {
        this._invoke("handle_keydown", { key });
    }
    public paste_clipboard_if_active(id: number) {
        this._invoke("paste_clipboard_if_active", { id });
    }
    public next_clipboard_item() {
        this._invoke("next_clipboard_item", null)
    }
    public previous_clipboard_item() {
        this._invoke("previous_clipboard_item", null)
    }
    public save_clipboard_item(id: number) {
        this._invoke("save_clipboard_item", { id })
    }
    public change_shortcut(info: ShortcutInfo) {
        return this._invoke("change_shortcut", { info })
    }
    public change_limit(limit: number) {
        return this._invoke("change_limit", { limit })
    }
    public change_hide_app_losing_focus(value: boolean) {
        this._invoke("change_hide_app_losing_focus", { value })
    }
    public remove_clipboard_history_item(id: number) {
        this._invoke("remove_clipboard_history_item", { id })
    }
    public remove_all_clipboard_history_items() {
        this._invoke("remove_all_clipboard_history_items", null)
    }

    public flushSchedule() {
        this._invoke("flush_schedule", null);
    }

    private async _invoke(cmd: string, args: any) {
        const ret = await invoke<InvokeReturn>(cmd, args)
        this._applyInvokeRet(ret)
    }

    private _applyInvokeRet(ret: InvokeReturn) {
        if (ret.changed_view) {
            viewService.update(ret.changed_view)
        }
        resourceService.applyResourcesChanged(ret.changed_resources)
    }
}

export const bridgeService = new BridgeService();