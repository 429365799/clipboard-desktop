import { ShortcutManager } from './shortcut-manager'

export class Application {
    
    #shortcutManager: ShortcutManager

    constructor() {
        this.#shortcutManager = new ShortcutManager()
    }

    getShortcutManager() {
        return this.#shortcutManager
    }
}
