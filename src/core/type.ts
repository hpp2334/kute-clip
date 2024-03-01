
export const enum ClipboardHistoryItemType {
    Text = 0,
    Image = 1,
}

export interface VClipboardHistoryItem {
    typ: ClipboardHistoryItemType,
    id: number,
    text: string,
    width: number,
    height: number,
    resource: number,
    shortcut: string
}

export interface VClipboardHistoryDetailItem {
    typ: ClipboardHistoryItemType,
    text: string,
    width: number,
    height: number,
    resource: number,
    chars_len: number,
    resource_byte_size: string,
}

export interface VClipboardHistory {
    items: VClipboardHistoryItem[]
}

export interface VClipboardActiveHistoryItem {
    id: number,
    item: VClipboardHistoryDetailItem | null,
}

export interface ResourceChange {
    id: number,
    buf: number[] | null,
}

export interface ShortcutInfo {
    ctrl: boolean,
    meta: boolean,
    alt: boolean,
    shift: boolean,
    code: string,
}

export interface PreferenceData {
    shortcut: ShortcutInfo,
    limit: number,
    hide_app_when_losing_focus: boolean,
    send_notification_when_copy: boolean,
}

export interface VPreference {
    data: PreferenceData,
    shortcut_success: boolean,
}

export interface RootView {
    clipboard_history: VClipboardHistory | null
    active_clipboard_item: VClipboardActiveHistoryItem | null
    preference: VPreference | null
}

export interface InvokeReturn {
    changed_view: RootView,
    changed_resources: ResourceChange[],
}