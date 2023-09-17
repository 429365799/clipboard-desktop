import { invoke } from '@tauri-apps/api'
import { ClipboardData } from '../interfaces'
import type { Application } from './application'
import { ValueOf } from '../global'
import { useClipboardList } from '../states/clipboard-list'

export const Commands = {
    GetClipboardList: 'get_clipboard_list',
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
        const list = await this.invoke<ClipboardData[]>(Commands.GetClipboardList)
        console.log('list', list)
        useClipboardList.getState().reset(list)
    }

    async toggleMainWindow() {
        return await this.invoke(Commands.ToggleMainWindow)
    }
}
