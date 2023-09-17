export interface FileInfo {
    path: string
    size: number
}

export type FormatType = 'text' | 'html' | 'image' | 'files'

export interface ClipboardData {
    key: string
    format_types: FormatType[]
    time: number
    text: string
    html: string
    image: string
    files: FileInfo[]
}
