import type { Application } from "./core/application"

declare global {
    interface Window {
        APP: Application
    }
}

