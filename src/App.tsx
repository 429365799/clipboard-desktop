import "./App.less";

import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";
import { ContentTypeTabs } from './components/ContentTypeTabs/ContentTypeTabs'
import { ClipboardData } from "./interfaces";
import { ClipboardContent } from "./components/ClipboardContent/ClipboardContent";
import { TypeOrder } from './constants'

async function getClipboardList() {
  return await invoke<ClipboardData[]>('get_clipboard_list')
}

function App() {
  const [list, setList] = useState<ClipboardData[]>([])
  const [activeTab, setActiveTab] = useState<keyof ClipboardData>()

  useEffect(() => {
    getList()
  }, [])

  const getList = () => {
    getClipboardList().then(res => {
      setList([res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], res[0], ])
      
      if (res.length > 0) {
        const keys = Object.keys(res[0]) as (keyof ClipboardData)[]
        setActiveTab(keys.sort((a, b) => TypeOrder.indexOf(a) - TypeOrder.indexOf(b))[0])
      }
    })
  }

  return (
    <div className="container">
      {
        list.map((item, index) => (
          <div key={index} className="clipboard-item">
            <ContentTypeTabs data={item} defaultActiveTab={activeTab} onActiveChange={setActiveTab} />

            {
              activeTab && (
                <ClipboardContent type={activeTab} content={item[activeTab]} />
              )
            }
          </div>
        ))
      }
    </div>
  );
}

export default App;
