import "./styles.less";

import { Application } from "./core/application";

import * as React from "react";
import * as ReactDOM from "react-dom/client";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

function initCore() {
  const app = new Application()
  
  window.APP = app;
}

initCore()
