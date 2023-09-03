
export interface FileInfo {
    path: string
    size: number
}

export interface ClipboardData {
    text: string
    html: string
    image: string
    files: FileInfo[]
}