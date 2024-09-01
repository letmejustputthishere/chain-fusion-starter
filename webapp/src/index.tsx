import "./index.css";
import "./polyfills";
import React from "react";
import App from "./components/App";
import { WagmiProvider } from "wagmi";
import ReactDOM from "react-dom/client";
import "@rainbow-me/rainbowkit/styles.css";
import { gnosis, sepolia } from "wagmi/chains";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import {
  darkTheme,
  getDefaultConfig,
  RainbowKitProvider,
} from "@rainbow-me/rainbowkit";

const config = getDefaultConfig({
  appName: "ReTransICP",
  projectId: process.env.REACT_APP_WALLET_CONNECT_PROJECT_ID as string,
  chains: [gnosis, sepolia],
});

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

const queryClient = new QueryClient();

root.render(
  <React.StrictMode>
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider
          theme={darkTheme({
            accentColor: "#7b3fe4",
          })}
        >
          <App />
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  </React.StrictMode>
);
