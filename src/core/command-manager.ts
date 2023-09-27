import { invoke } from '@tauri-apps/api'
import { ClipboardData } from '../interfaces'
import type { Application } from './application'
import { ValueOf } from '../global'
import { useClipboardList } from '../states/clipboard-list'
import { useSelection } from '../states/selection'

export const Commands = {
    GetClipboardList: 'get_clipboard_list',
    SelectClipboardData: 'select_clipboard_data',
    ShowMainWindow: 'show_main_window',
    HideMainWindow: 'hide_main_window',
    ToggleMainWindow: 'toggle_main_window',
} as const

export type Command = ValueOf<typeof Commands>

export class CommandManager {
    constructor(private app: Application) {}

    async invoke<Response>(command: Command, args?: any) {
        return await invoke<Response>(command, args)
    }

    async initClipboardList() {
        const list = await this.invoke<ClipboardData[]>(
            Commands.GetClipboardList
        )
        useClipboardList.getState().reset(list)
        useSelection.getState().setCurrent(list[0].key)
    }

    async toggleMainWindow() {
        return await this.invoke(Commands.ToggleMainWindow)
    }

    async selectClipboardData(key: string) {
        return await this.invoke(Commands.SelectClipboardData, { key })
    }
}
