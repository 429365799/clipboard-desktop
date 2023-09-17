import "./style.less";

import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { ClipboardData, FileInfo, FormatType } from "../../interfaces"
import { useMemo, useState } from "react";
import { Colors } from "../../constants";

import fileIcon from '../../assets/react.svg'

interface IProps {
    type: FormatType
    content: any
}

const TextContent = ({ content }: {content: string}) => {
    return <div className="content content-text">{content}</div>
}

const HtmlContent = ({ content }: {content: string}) => {
    const [isFormat, setIsFormat] = useState(true)

    const toggleFormat = () => {
        setIsFormat(!isFormat)
    }

    const html = useMemo(() => {
        const startHtmlIndex = content.match(/StartHTML:(\d*)/)?.[1]
        if (startHtmlIndex) {
            return content.slice(Number(startHtmlIndex))
        } else if (content.includes('<html>')) {
            return content.slice(content.indexOf('<html>'))
        } else {
            return content
        }

    }, [content])

    return <div className="content content-html">
        <div className={ "format-btn " + (isFormat ? 'active' : '') } title="格式化" onClick={toggleFormat}></div>
        {
            isFormat ? <div dangerouslySetInnerHTML={{__html: html}}></div> : <code>{content}</code>
        }
    </div>
}

const ImageContent = ({ content }: {content: string}) => {
    const src = useMemo(() => convertFileSrc(content), [content])

    return (
        <div className="content content-html">
            <img src={src} />
        </div>
    )
}

const FilesContent = ({ content }: {content: FileInfo[]}) => {
    
    const onItemClick = (item: FileInfo) => {
        console.log(item);
        invoke("show_in_folder", { path: item.path });
    }

    return (
        <div className="content content-files">
            {
                content.map((item, index) => (
                    <div key={index} className="file-item" onClick={() => onItemClick(item)}>
                        <img src={fileIcon} />
                        <div className="file-info">
                            <div className="file-name">{item.path}</div>
                            <div className="file-size">{Math.floor(item.size / 1024)} KB</div>
                        </div>
                    </div>
                ))
            }
        </div>
    )
}

export const ClipboardContent = (props: IProps) => {
    const { type, content } = props

    return (
        <div className="clipboard-content" style={{borderColor: Colors[type as keyof typeof Colors]}}>
            {
                type === 'text' && <TextContent content={content} />
            }
            {
                type === 'html' && <HtmlContent content={content} />
            }
            {
                type === 'image' && <ImageContent content={content} />
            }
            {
                type === 'files' && <FilesContent content={content} />
            }
        </div>
    )
}
