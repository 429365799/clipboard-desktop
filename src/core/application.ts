import { CommandManager, Command } from './command-manager'
import { ShortcutManager } from './shortcut-manager'

export class Application {
    #shortcutManager: ShortcutManager
    #commandManager: CommandManager

    constructor() {
        this.#shortcutManager = new ShortcutManager(this)
        this.#commandManager = new CommandManager(this)
    }

    getShortcutManager() {
        return this.#shortcutManager
    }

    getCommandManager() {
        return this.#commandManager
    }

    invokeCommand(command: Command, args?: any) {
        return this.#commandManager.invoke(command, args)
    }
}
