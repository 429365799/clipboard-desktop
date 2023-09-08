import { globalShortcut } from '@tauri-apps/api'
import type { Application } from './application'
import { Commands } from './command-manager'

export class ShortcutManager {
    constructor(private app: Application) {
        this.registerShortcuts()
    }

    registerShortcuts() {
        globalShortcut.register('Ctrl+Shift+V', () => {
            this.app.invokeCommand(Commands.ToggleMainWindow)
        })
        // globalShortcut.register('Left', (e) => {
        //     console.log('left', e)
        // })
    }
}
