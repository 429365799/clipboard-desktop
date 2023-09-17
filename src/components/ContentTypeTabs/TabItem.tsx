import { Colors } from "../../constants"
import { FormatType } from "../../interfaces"

interface IProps {
    label: FormatType
    active: boolean
    value?: FormatType
    onClick?: (value: FormatType) => void
}

export const TabItem = (props: IProps) => {
    const { onClick, label, value, active } = props

    const handleClick = () => {
        onClick?.(value || label)
    }

    const classNames = [
        'type-tab',
        active ? 'active' : ''
    ].filter(Boolean).join(' ')

    const color = active ? Colors[(value || label) as keyof typeof Colors] : '#ffffff'
    const styles: React.CSSProperties = {
        // borderBottom: `4px solid ${color}`
        backgroundColor: color,
        color: active ? '#ffffff' : '#0f0f0f',
    }

    return <div className={classNames} style={styles} onClick={handleClick}>{label}</div>
}
