import './App.less'

import { useState, useEffect, useCallback } from 'react'
import { ContentTypeTabs } from './components/ContentTypeTabs/ContentTypeTabs'
import { ClipboardData, FormatType } from './interfaces'
import { ClipboardContent } from './components/ClipboardContent/ClipboardContent'
import { useClipboardList } from './states/clipboard-list'

interface TabData {
    active: FormatType
    data: ClipboardData
}

function App() {
    const clipoardList = useClipboardList(state => state.list)
    const [tabs, setTabs] = useState<Record<string, TabData> | null>(null)
    const [indicatorIndex, setIndicatorIndex] = useState(0)

    useEffect(() => {
        const tabData: Record<string, TabData> = {}
        if (clipoardList.length > 0) {
            for (const item of clipoardList) {
                tabData[item.key] = {
                    active: item.format_types[0],
                    data: item,
                }
            }
            setTabs(tabData)
        }
    }, [clipoardList])

    const changeActiveTab = useCallback((key: string, type: FormatType) => {
        setTabs((tabs) => {
            if (!tabs) {
                return null
            }
            return {
                ...tabs,
                [key]: {
                    ...tabs[key],
                    active: type,
                },
            }
        })
    }, [])

    return (
        <div className='container'>
            {tabs && clipoardList.map((item) => (
                <div className='clipboard-item-wrap'>
                    <div key={item.key} className='clipboard-item'>
                        <ContentTypeTabs
                            data={item}
                            defaultActiveTab={item.format_types[0]}
                            onActiveChange={changeActiveTab}
                        />

                        {
                            tabs[item.key] && (
                                <ClipboardContent
                                    type={tabs[item.key].active}
                                    content={item[tabs[item.key].active]}
                                />
                            )
                        }
                    </div>
                    <div className={'indicator ' + (indicatorIndex)}></div>
                </div>
            ))}
        </div>
    )
}

export default App
