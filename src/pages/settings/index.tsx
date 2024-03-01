import { ShortcutInfo } from '../../core/type';
import { useView } from '../../core/view.service';
import styles from './style.module.scss'
import BackIcon from './assets/back.svg'
import LandscapeBgIcon from './assets/landscape.svg'
import CheckboxIcon from './assets/checkbox.svg'
import { bridgeService } from '../../core/bridge.service';
import classNames from 'classnames';
import React, { useRef, useState } from 'react';
import { isNil, preventDefaultBrowserShortcuts, useForceUpdate } from '../../utils';
import { useNavigate } from '../../widgets/Router';
import logoUrl from '../../assets/logo.png'
import { SizeBox } from '../../widgets/SizeBox';
import { RouteKey } from '../../core/router.service';


function keyboardEventToShortcut(e: React.KeyboardEvent<HTMLInputElement>): ShortcutInfo {
    let code = e.code
    if (['ControlLeft', 'ControlRight', 'ShiftLeft', 'ShiftRight', 'AltLeft', 'AltRight', 'MetaLeft', 'MetaRight'].includes(code)) {
        code = ''
    }

    return {
        ctrl: e.ctrlKey,
        alt: e.altKey,
        shift: e.shiftKey,
        meta: e.metaKey,
        code: code
    }
}

function fmtShortcut(shortcut: ShortcutInfo | null): string {
    if (!shortcut) {
        return ''
    }

    let keys: string[] = []
    if (shortcut.ctrl) {
        keys.push('CTRL')
    }
    if (shortcut.alt) {
        keys.push('ALT')
    }
    if (shortcut.shift) {
        keys.push('SHIFT')
    }
    if (shortcut.meta) {
        keys.push('META')
    }
    if (shortcut.code) {
        keys.push(shortcut.code)
    }

    return keys.join(' + ')
}

function Checkbox({
    checked,
}: {
    checked: boolean,
}) {
    return (
        <div>
            <CheckboxIcon className={classNames(styles.checkbox, checked && styles.checked)} />
        </div>
    )
}

const Input = React.forwardRef<HTMLInputElement,
    {
        value: string | number,
        type?: React.HTMLInputTypeAttribute
        className?: string
        onChange?: React.ChangeEventHandler<HTMLInputElement>
        onKeyUp?: React.KeyboardEventHandler<HTMLInputElement>
        onKeyDown?: React.KeyboardEventHandler<HTMLInputElement>
        onBlur?: React.FocusEventHandler<HTMLInputElement>
    }
>(function Input(props, ref) {
    const { type, value, className, onChange, onBlur, onKeyDown, onKeyUp } = props
    const [focused, setFocused] = useState(false)

    return (
        <div className={classNames(styles.shortcutInputContainer, focused && styles.focused)}>
            <LandscapeBgIcon />
            <input
                ref={ref}
                type={type}
                className={classNames(styles.baseInput, className)}
                value={value}
                onChange={onChange}
                onKeyUp={onKeyUp}
                onKeyDown={(ev) => {
                    preventDefaultBrowserShortcuts(ev)
                    onKeyDown?.(ev)
                }}
                onBlur={(e) => {
                    onBlur?.(e)
                    setFocused(false)
                }}
                onFocus={() => {
                    setFocused(true)
                }}
            />
        </div>
    )
})

function Shortcut({
    shortcut,
    success
}: {
    shortcut: ShortcutInfo | null,
    success: boolean,
}) {
    const editingShortcutRef = useRef<ShortcutInfo | null>(null)
    const keyPressedRef = useRef<Set<string>>(new Set())
    const inputRef = useRef<HTMLInputElement>(null)
    const forceUpdate = useForceUpdate()

    const scheduleBlurEditing = () => {
        setTimeout(() => {
            inputRef.current?.blur()
        }, 100)
    }

    const clearEditing = () => {
        editingShortcutRef.current = null
        keyPressedRef.current.clear()
        forceUpdate()
    }

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        const shortcut = keyboardEventToShortcut(e);

        editingShortcutRef.current = shortcut
        keyPressedRef.current.add(e.code)
        forceUpdate()
    }

    const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
        keyPressedRef.current.delete(e.code)

        if (keyPressedRef.current.size === 0) {

            const editingShortcut = editingShortcutRef.current;
            if (!editingShortcut || !editingShortcut.code) {
                editingShortcutRef.current = null
                scheduleBlurEditing()
                forceUpdate()
            } else {
                bridgeService.change_shortcut(editingShortcut).then(() => {
                    scheduleBlurEditing()
                })
            }
        }
    }

    const shortcutStr = fmtShortcut(shortcut)

    return (
        <div className={styles.shortcutContainer}>
            <div className={styles.shortcutTitle}>Hotkey</div>
            <Input
                ref={inputRef}
                type='tel'
                className={styles.shortcutInput}
                value={editingShortcutRef.current ? fmtShortcut(editingShortcutRef.current) : shortcutStr}
                onChange={() => { /** noop */ }}
                onKeyUp={handleKeyUp}
                onKeyDown={handleKeyDown}
                onBlur={clearEditing}
            />
            {!success && <div>Fail to register hotkey.</div>}
        </div>
    )
}

function Limit({
    limit,
}: {
    limit: number
}) {
    const [editingValue, setEditingValue] = useState<string | null>()

    return (
        <div className={styles.shortcutContainer}>
            <div className={styles.shortcutTitle}>Limit (10-999)</div>
            <Input
                className={styles.limitInput}
                value={editingValue ?? limit}
                onChange={e => {
                    setEditingValue(e.target.value)
                }}
                onBlur={() => {
                    if (isNil(editingValue)) {
                        return;
                    }
                    let value = parseInt(editingValue)

                    if (!Number.isFinite(value) || Number.isNaN(value)) {
                        return;
                    }
                    value = Math.max(10, Math.min(999, value))

                    bridgeService.change_limit(value).then(() => {
                        setEditingValue(null)
                    })
                }}
            />
        </div>
    )
}

export function SettingPage() {
    const navigate = useNavigate()
    const view = useView('preference')

    const hideAppWhenLosingFocus = Boolean(view?.data.hide_app_when_losing_focus)

    return (
        <div className={styles.screenContainer}>
            <div className={styles.backBtn} onClick={() => navigate(RouteKey.Main)}>
                <BackIcon />
            </div>
            <div className={styles.settingContainer}>
                <SizeBox height={45} />
                <div className={styles.sectionTitle}>GENERAL</div>
                <div className={styles.divider} />
                <SizeBox height={20} />
                <div className={styles.panel}>
                    <Shortcut shortcut={view?.data.shortcut ?? null} success={Boolean(view?.shortcut_success)} />
                    <Limit limit={view?.data.limit ?? 0} />
                </div>
                <SizeBox height={20} />
                <div className={styles.options}>
                    <div className={styles.option} onClick={() => {
                        bridgeService.change_hide_app_losing_focus(!hideAppWhenLosingFocus)
                    }}>
                        <Checkbox checked={hideAppWhenLosingFocus} />
                        <div className={styles.checkboxDesc}>Hide the app when losing focus</div>
                    </div>
                    {/* <div className={styles.option}>
                        <Checkbox checked={false} onCheck={() => { }} />
                        <div className={styles.checkboxDesc}>Send notification when copy</div>
                    </div> */}
                </div>
                <SizeBox height={26} />
                <div className={styles.sectionTitle}>ABOUT</div>
                <div className={styles.divider} />
                <div className={styles.aboutContent}>
                    <img className={styles.aboutLogo} src={logoUrl} width={128} height={128} />
                    <div className={styles.desc}>
                        <div>
                            KuteClip is a clipboard history application written with tauri.
                        </div>
                        <div>
                            Github: https://github.com/hpp2334/kute-clip
                        </div>
                        <div>
                            Version: {__APP_VERSION__}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}