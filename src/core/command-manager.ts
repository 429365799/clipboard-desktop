import { invoke } from '@tauri-apps/api'
import { ClipboardData } from '../interfaces'
import type { Application } from './application'
import { ValueOf } from '../global'

export const Commands = {
    GetClipboardList: 'get_clipboard_list',
    ShowMainWindow: 'show_main_window',
    HideMainWindow: 'hide_main_window',
    ToggleMainWindow: 'toggle_main_window',
} as const

export type Command = ValueOf<typeof Commands>

export class CommandManager {
    constructor(private app: Application) {}

    async invoke(command: Command, args?: any) {
        return await invoke<ClipboardData[]>(command, args)
    }
}
