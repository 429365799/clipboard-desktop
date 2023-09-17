import { globalShortcut } from '@tauri-apps/api'
import type { Application } from './application'

export class ShortcutManager {
    constructor(private app: Application) {
        this.registerShortcuts()
    }

    registerShortcuts() {
        globalShortcut.register('Ctrl+Shift+V', () => {
            this.app.getCommandManager().toggleMainWindow()
        })
        // globalShortcut.register('Left', (e) => {
        //     console.log('left', e)
        // })
    }
}
