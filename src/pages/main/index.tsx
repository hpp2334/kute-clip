import { ClipboardHistoryItemType, VClipboardHistoryItem } from '../../core/type';
import { useView } from '../../core/view.service';
import styles from './style.module.scss'
import B2 from './assets/b2.svg'
import B3 from './assets/b3.svg'
import DownloadIcon from './assets/download.svg'
import Bgl3 from './assets/bg-l3.svg'
import BgImg from './assets/bg-image.svg'
import SettingIcon from '../../assets/setting.svg'
import { bridgeService } from '../../core/bridge.service';
import classNames from 'classnames';
import React, { useEffect, useRef, useState } from 'react';
import { isNil } from '../../utils';
import { resourceService } from '../../core/resource.service';
import { useNavigate } from '../../widgets/Router';
import { RouteKey } from '../../core/router.service';


const BG_L3_HEIGHT = 43;
const BG_IMAGE_HEIGHT = 112;
const PADDING = 32
const LIST_HEIGHT = 336;


function ListItem({
    item,
    isActive,
    onContextMenu
}: {
    item: VClipboardHistoryItem,
    isActive: boolean,
    onContextMenu: (x: number, y: number, id: number) => void
}) {

    const bgHeight = item.typ === ClipboardHistoryItemType.Text ? BG_L3_HEIGHT : BG_IMAGE_HEIGHT

    const bgElRef = useRef<HTMLDivElement>(null)

    const handleClick = () => {
        bridgeService.change_clipboard_active(item.id)
    }

    useEffect(() => {
        const current = bgElRef.current;
        if (isActive && !isNil(current)) {
            const bounds = current.getBoundingClientRect()
            if (bounds.top < PADDING || bounds.top + bgHeight > document.body.clientHeight - PADDING) {
                current.scrollIntoView()
            }
        }
    }, [isActive])

    return (
        <div
            ref={bgElRef}
            className={classNames(styles.historyItem, item.typ === ClipboardHistoryItemType.Image && styles.imgType, isActive && styles.active)}
            onClick={handleClick}
            onDoubleClick={() => {
                bridgeService.paste_clipboard_if_active(item.id)
            }}
            onContextMenu={(e) => {
                e.preventDefault()
                handleClick()
                onContextMenu(e.clientX, e.clientY, item.id)
            }}>
            {item.typ === ClipboardHistoryItemType.Text && (
                <>
                    <Bgl3 className={styles.background} />
                    <div className={styles.shortcut}>{item.shortcut}</div>
                    <div className={styles.content}>{item.text}</div>
                </>
            )}
            {item.typ === ClipboardHistoryItemType.Image && (
                <>
                    <BgImg className={styles.background} />
                    <div className={styles.shortcut}>{item.shortcut}</div>
                    <div className={styles.content}>
                        <img className={styles.image} src={resourceService.getAsPngBase64(item.resource)} />
                    </div>
                </>
            )}
        </div>
    )
}

function ScrollableHistoryList() {
    const historyList = useView('clipboard_history');
    const activeHistoryItem = useView('active_clipboard_item')
    const listElRef = useRef<HTMLDivElement>(null)
    const [scrollbarStyle, setScrollbarStyle] = useState<React.CSSProperties>({
        display: 'none'
    })
    const [contextMenuContainerStyle, setContextMenuContainerStyle] = useState<React.CSSProperties>({
        display: 'none'
    })
    const [contextMenuStyle, setContextMenuStyle] = useState<React.CSSProperties>({})
    const [contextMenuAttachId, setContextMenuAttachId] = useState<number | null>(null)
    const closeContextMenu = () => {
        setContextMenuContainerStyle({
            display: 'none'
        })
    }
    const openContextMenu = (x: number, y: number, id: number) => {
        setContextMenuContainerStyle({
            display: 'block'
        })
        setContextMenuStyle({
            left: x,
            top: y,
        })
        setContextMenuAttachId(id)
    }


    useEffect(() => {
        const listEl = listElRef.current;
        if (!listEl) {
            return;
        }

        const bounds = listEl.getBoundingClientRect()
        if (bounds.height <= LIST_HEIGHT) {
            setScrollbarStyle({
                display: 'none'
            })
        } else {
            const height = LIST_HEIGHT / bounds.height * LIST_HEIGHT;
            const top = (PADDING - bounds.top) / bounds.height * LIST_HEIGHT + PADDING

            setScrollbarStyle({
                display: 'block',
                top,
                height,
            })
        }
    }, [activeHistoryItem?.id, historyList?.items.length])

    if (!historyList?.items.length) {
        return null
    }

    return (
        <>
            <div className={styles.historyListContainer}>
                <div className={styles.historyList} ref={listElRef}>
                    {historyList.items.map((item) =>
                        <ListItem
                            key={item.id}
                            item={item}
                            isActive={activeHistoryItem?.id === item.id}
                            onContextMenu={(x, y, id) => {
                                openContextMenu(x, y, id)
                            }}
                        />
                    )}
                </div>
            </div>
            <div className={styles.scrollbar} style={scrollbarStyle} />
            <div className={styles.contextMenuContainer} style={contextMenuContainerStyle} onClick={() => {
                closeContextMenu()
            }}>
                <div className={styles.contextMenu} style={contextMenuStyle}>
                    {contextMenuAttachId !== null && <div className={styles.contextMenuItem} onClick={() => { bridgeService.remove_clipboard_history_item(contextMenuAttachId) }}>Remove</div>}
                    <div className={styles.contextMenuItem} onClick={() => { bridgeService.remove_all_clipboard_history_items() }}>Remove All</div>
                </div>
            </div>
        </>
    )
}

function DetailContent() {
    const _item = useView('active_clipboard_item');
    if (!_item || !_item.item) {
        return null
    }
    const { item, id } = _item

    return (
        <div className={styles.historyDetail}>
            <div className={styles.bar}>
                {item.typ === ClipboardHistoryItemType.Text && (
                    <div>{item.chars_len} Characters</div>
                )}
                {item.typ === ClipboardHistoryItemType.Image && (
                    <div>{item.width}×{item.height}  ·  {item.resource_byte_size}</div>
                )}
                <div>
                    <div className={styles.button} onClick={() => {
                        bridgeService.save_clipboard_item(id)
                    }}>
                        <DownloadIcon />
                    </div>
                </div>
            </div>
            {item.typ === ClipboardHistoryItemType.Text && (
                <div className={styles.content}>
                    {item.text}
                </div>
            )}
            {item.typ === ClipboardHistoryItemType.Image && (
                <div className={styles.content}>
                    <img className={styles.image} src={resourceService.getAsPngBase64(item.resource)} />
                </div>
            )}
        </div>
    )
}

export function MainPage() {
    const navigate = useNavigate()

    return (
        <div className={styles.screenContainer} tabIndex={0}>
            <div className={styles.main} onWheel={e => {
                if (e.deltaY > 1) {
                    bridgeService.next_clipboard_item()
                } else if (e.deltaY < -1) {
                    bridgeService.previous_clipboard_item()
                }
            }}>
                <div className={styles.left}>
                    <B2 className={styles.b2} />
                    <ScrollableHistoryList />
                    <SettingIcon className={styles.settingIcon} onClick={() => navigate(RouteKey.Setting)} />
                </div>
                <div className={styles.right}>
                    <B3 className={styles.b3} />
                    <DetailContent />
                </div>
            </div>
        </div>
    )
}