import type { Application } from './core/application'

declare global {
    interface Window {
        APP: Application
    }
}

declare type ValueOf<T> = T[keyof T]
