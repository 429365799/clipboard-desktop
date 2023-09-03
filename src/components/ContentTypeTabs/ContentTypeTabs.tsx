import "./style.less";

import { useCallback, useEffect, useMemo, useState } from "react";
import { ClipboardData } from "../../interfaces";
import { TabItem } from "./TabItem";
import { TypeOrder } from "../../constants";

interface IProps {
    data: ClipboardData
    defaultActiveTab?: keyof ClipboardData
    onActiveChange: (activeTab: keyof ClipboardData) => void
}

export const ContentTypeTabs = (props: IProps) => {
    const { data, defaultActiveTab, onActiveChange } = props
    const [activeTab, setActiveTab] = useState('')
    
    const handleTabChange = useCallback((tab: string) => {
        setActiveTab(tab)
        onActiveChange(tab as keyof ClipboardData)
    }, [])
    
    useEffect(() => {
        defaultActiveTab && setActiveTab(defaultActiveTab)
    }, [defaultActiveTab])

    const keys = useMemo(() => {
        const keys = Object.keys(data) as (keyof ClipboardData)[]
        keys.sort((a, b) => TypeOrder.indexOf(a) - TypeOrder.indexOf(b))
        return keys
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
