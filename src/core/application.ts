import { CommandManager, Command, Commands } from './command-manager'
import { ShortcutManager } from './shortcut-manager'
import { EventHandler } from './event-handler'
// import { useClipboardList } from '../states/clipboard-list'
// import { ClipboardData } from '../interfaces'

export class Application {
    #shortcutManager: ShortcutManager
    #commandManager: CommandManager
    #backendEventHandler: EventHandler

    constructor() {
        this.#shortcutManager = new ShortcutManager(this)
        this.#commandManager = new CommandManager(this)
        this.#backendEventHandler = new EventHandler(this)

        this.#initApp()
    }

    #initApp() {
        this.getCommandManager().initClipboardList()

        this.#backendEventHandler.startListeners()
    }

    getShortcutManager() {
        return this.#shortcutManager
    }

    getCommandManager() {
        return this.#commandManager
    }

    invokeCommand<Response>(command: Command, args?: any) {
        return this.#commandManager.invoke<Response>(command, args)
    }
}
