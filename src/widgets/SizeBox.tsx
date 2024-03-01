import React, { useMemo } from 'react'
import { isNil } from '../utils'

export interface SizeBoxProps {
    width?: number
    height?: number
}

export const SizeBox = React.memo(({ width, height }: SizeBoxProps) => {
    const style = useMemo(() => {
        const style: React.CSSProperties = {}
        if (!isNil(width)) {
            style.width = width
        }
        if (!isNil(height)) {
            style.height = height
        }
        return style
    }, [width, height])

    return <div style={style} />
})
