import { useCallback, useState } from "react"

export function isNil(x: unknown): x is null | undefined {
    return x === null || x === undefined
}

export function useForceUpdate() {
    const [_, setX] = useState(0)

    const updater = useCallback(() => {
        setX(x => x + 1)
    }, [])

    return updater
}

let _isApple: boolean | null = null

export function isApple(): boolean {
    if (_isApple !== null) {
        return _isApple
    }

    _isApple = navigator.userAgent.indexOf('Mac OS X') != -1
    return _isApple
}

export function preventDefaultBrowserShortcuts(ev: KeyboardEvent | React.KeyboardEvent<HTMLInputElement>) {
    const enum PlatformBitFlag {
        Win = 1,
        Apple = 2,
    }
    const config: Array<{
        code: string,
        ctrlLike?: true,
        shift?: true,
        platformBits?: PlatformBitFlag,
    }> = [
            // All function codes (F1-F12), including reload
            ...Array.from({ length: 12 }).fill(0).map((_, i) => `F${i + 1}`).map(code => ({ code })),
            // Find
            {
                code: 'KeyF',
                ctrlLike: true,
            },
            {
                code: 'F3',
                platformBits: PlatformBitFlag.Win
            },
            // Print
            {
                code: 'KeyP',
                ctrlLike: true,
            },
            // DevTool
            {
                code: 'KeyC',
                ctrlLike: true,
                shift: true,
            }
        ]

    const _isApple = isApple()
    const currentPlatformBitFlag = !_isApple ? PlatformBitFlag.Win : PlatformBitFlag.Apple
    const ctrlLike = !_isApple ? ev.ctrlKey : ev.metaKey;
    const shift = ev.shiftKey
    const code = ev.code

    if (config.some(item => {
        const codeMatch = code === item.code
        const modsMatch = (!item.ctrlLike || ctrlLike) && (!item.shift || shift)
        const platformMatch = Boolean((item.platformBits ?? (PlatformBitFlag.Win | PlatformBitFlag.Apple)) & currentPlatformBitFlag)

        return codeMatch && modsMatch && platformMatch
    })) {
        ev.preventDefault()
    }
}