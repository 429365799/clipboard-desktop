import "./style.less";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ClipboardData, FormatType } from "../../interfaces";
import { TabItem } from "./TabItem";
import { TypeOrder } from "../../constants";

interface IProps {
    data: ClipboardData
    defaultActiveTab?: FormatType
    onActiveChange: (key: string, activeTab: FormatType) => void
}

export const ContentTypeTabs = (props: IProps) => {
    const { data, defaultActiveTab, onActiveChange } = props
    const [activeTab, setActiveTab] = useState('')
    
    const handleTabChange = useCallback((tab: FormatType) => {
        setActiveTab(tab)
        onActiveChange(data.key, tab)
    }, [])
    
    useEffect(() => {
        defaultActiveTab && setActiveTab(defaultActiveTab)
    }, [defaultActiveTab])

    const keys = useMemo(() => {
        return data.format_types.sort((a, b) => TypeOrder.indexOf(a) - TypeOrder.indexOf(b))
    }, [data])

    return (
        <div className="clipboard-type-tabs">
            {
                keys.map(type => (
                    <TabItem key={type} label={type} active={type === activeTab} onClick={handleTabChange} />
                ))
            }
        </div>
    )
}
