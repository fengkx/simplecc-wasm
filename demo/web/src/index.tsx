import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { Theme } from "@radix-ui/themes";
import "@radix-ui/themes/styles.css";

const rootEl = document.getElementById("root");
if (rootEl) {
  const root = ReactDOM.createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <Theme>
        <App />
      </Theme>
    </React.StrictMode>
  );
}
