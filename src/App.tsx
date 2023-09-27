import './App.less'

import { useState, useEffect, useCallback, KeyboardEventHandler } from 'react'
import { ContentTypeTabs } from './components/ContentTypeTabs/ContentTypeTabs'
import { ClipboardData, FormatType } from './interfaces'
import { ClipboardContent } from './components/ClipboardContent/ClipboardContent'
import { useClipboardList } from './states/clipboard-list'
import { useSelection } from './states/selection'

interface TabData {
    active: FormatType
    data: ClipboardData
}

function App() {
    const clipoardList = useClipboardList((state) => state.list)
    const { selectedKey, next, prev, select, setCurrent } = useSelection(
        (state) => state
    )
    const [tabs, setTabs] = useState<Record<string, TabData> | null>(null)

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

    useEffect(() => {
        const index = clipoardList.findIndex((item) => item.key === selectedKey)
        document
            .querySelectorAll('.clipboard-item-wrap')
            .item(index)
            ?.scrollIntoView({ behavior: 'smooth' })
    }, [selectedKey, clipoardList])

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

    const handleKeyEvent: KeyboardEventHandler<HTMLDivElement> = useCallback(
        (e) => {
            console.log(e.target)

            switch (e.key) {
                case 'ArrowRight':
                case 'ArrowDown':
                    next()
                    e.preventDefault()
                    break
                case 'ArrowLeft':
                case 'ArrowUp':
                    prev()
                    e.preventDefault()
                    break
                case 'Enter':
                    select()
                    break
            }
        },
        []
    )

    const handleMouseEnter = useCallback((target: ClipboardData) => {
        setCurrent(target.key)
    }, [])

    return (
        <div className='container' tabIndex={0} onKeyDown={handleKeyEvent}>
            {tabs &&
                clipoardList.map((item) => (
                    <div
                        key={item.key}
                        className='clipboard-item-wrap'
                        onMouseEnter={() => handleMouseEnter(item)}
                    >
                        <div className='clipboard-item'>
                            <ContentTypeTabs
                                data={item}
                                defaultActiveTab={item.format_types[0]}
                                onActiveChange={changeActiveTab}
                            />

                            {tabs[item.key] && (
                                <ClipboardContent
                                    type={tabs[item.key].active}
                                    content={item[tabs[item.key].active]}
                                />
                            )}
                        </div>
                        <div
                            className={
                                'indicator ' +
                                (selectedKey === item.key ? 'active' : '')
                            }
                        ></div>
                    </div>
                ))}
        </div>
    )
}

export default App
