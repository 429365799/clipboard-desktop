import { create } from 'zustand'

import type { ClipboardData } from '../interfaces'

export interface ClipboardList {
    list: ClipboardData[]
    reset: (data: ClipboardData[]) => void
}

export const useClipboardList = create<ClipboardList>()((set) => {
    return {
        list: [],
        reset: (data) => {
            set(() => ({
                list: data.sort((a, b) => b.time - a.time)
            }))
        },
    }
})
