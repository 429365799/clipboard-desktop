import { create } from 'zustand'
import { useClipboardList } from './clipboard-list'

export interface Selection {
    selectedKey: string
    setCurrent: (key: string) => void
    next: () => void
    prev: () => void
    select: () => void
}

export const useSelection = create<Selection>()((set, get) => {
    return {
        selectedKey: '',
        setCurrent: (key: string) => {
            set({ selectedKey: key })
        },
        next: () => {
            set((state) => {
                const list = useClipboardList.getState().list
                const currentIndex = list.findIndex(
                    (item) => item.key === state.selectedKey
                )
                if (currentIndex < list.length - 1) {
                    return {
                        selectedKey: list[currentIndex + 1].key,
                    }
                }

                return {
                    selectedKey: state.selectedKey,
                }
            })
        },
        prev: () => {
            set((state) => {
                const list = useClipboardList.getState().list
                const currentIndex = list.findIndex(
                    (item) => item.key === state.selectedKey
                )
                if (currentIndex > 0) {
                    return {
                        selectedKey: list[currentIndex - 1].key,
                    }
                }

                return {
                    selectedKey: state.selectedKey,
                }
            })
        },
        select: () => {
            window.APP.getCommandManager().selectClipboardData(
                get().selectedKey
            )
        },
    }
})
