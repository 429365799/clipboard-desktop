import { listen } from '@tauri-apps/api/event'
import type { Application } from './application'

export class EventHandler {
    constructor(private app: Application) {

    }

    startListeners() {
        listen('CLIPBOARD_UPDATE', this.#handleClipboardChange);
    }

    #handleClipboardChange = () => {
        this.app.getCommandManager().initClipboardList()
    }
}
