import styles from './typography.module.scss'
import classNames from 'classnames'

export interface TypographyProps {
    text: string
    className?: string
}

export function Typography({ text, className }: TypographyProps) {
    return (
        <div className={classNames(styles.text, className)}>{text}</div>
    )
}