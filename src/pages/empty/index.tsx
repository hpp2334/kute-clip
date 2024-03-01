import styles from './style.module.scss'
import CenterIcon from './assets/icon.svg'
import B1 from './assets/b1.svg'
import { Typography } from '../../widgets/Typography'
import { SizeBox } from '../../widgets/SizeBox'
import { useNavigate } from '../../widgets/Router'
import SettingIcon from '../../assets/setting.svg'
import { RouteKey } from '../../core/router.service'

export function EmptyPage() {
    const navigate = useNavigate()

    return (
        <div className={styles.emptyContainer}>
            <div className={styles.content}>
                <CenterIcon />
                <SizeBox height={10} />
                <Typography className={styles.title} text="No Record" />
                <SizeBox height={2} />
                <Typography className={styles.subTitle} text="Try to copy any text or image to generate a record!" />
            </div>
            <B1 className={styles.b1} />
            <div className={styles.settingIcon} onClick={() => navigate(RouteKey.Setting)}>
                <SettingIcon />
            </div>
        </div>
    )
}