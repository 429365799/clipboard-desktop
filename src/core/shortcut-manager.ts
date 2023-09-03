import { globalShortcut } from "@tauri-apps/api";

export class ShortcutManager {
    constructor() {
        this.registerShortcuts()
    }

    registerShortcuts() {
        globalShortcut.register('Ctrl+Shift+V', () => {
            console.log('Ctrl+Shift+V!')
        })
    }
}
 