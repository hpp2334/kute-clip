import { useEffect, useState } from "react";
import { isNil } from "../utils";
import { RootView } from "./type";

export class ViewService {
    private _view: RootView = {
        clipboard_history: null,
        active_clipboard_item: null,
        preference: null,
    }
    private _map: Map<keyof RootView, Set<() => void>> = new Map()

    view(): RootView {
        return this._view
    }

    registerView(key: keyof RootView, notify: () => void) {
        if (!this._map.has(key)) {
            this._map.set(key, new Set())
        }
        const set = this._map.get(key)!
        set.add(notify)

        return () => {
            set.delete(notify)
        }
    }

    update(changed_view: RootView) {
        const changed: Array<keyof RootView> = []
        for (const key in changed_view) {
            const value = (changed_view as any)[key]
            if (!isNil(value)) {
                (this._view as any)[key] = value;
                changed.push(key as keyof RootView)
            }
        }
        changed.forEach(key => {
            this._notifyViews(key)
        })
    }

    private _notifyViews(key: keyof RootView) {
        const set = this._map.get(key)
        if (set) {
            set.forEach(f => f())
        }
    }
}

export const viewService = new ViewService()

export function useView<K extends keyof RootView, P extends RootView[K]>(key: K): P {
    const [state, setState] = useState<P>(viewService.view()[key] as P)

    useEffect(() => {
        return viewService.registerView(key, () => {
            setState(viewService.view()[key] as P)
        })
    }, [])

    return state
}