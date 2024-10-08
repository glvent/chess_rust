"use client"

import React, { createContext, useEffect, useRef, useState } from 'react';

interface WebSocketCtxValue {
    ws: WebSocket | null;
    sendMessage: (msg: string) => void;
}

export const WebSocketCtx = createContext<WebSocketCtxValue>({
    ws: null,
    sendMessage: () => {},
})

export const WebSocketProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [ws, setWs] = useState<WebSocket | null>(null);

    const wsRef = useRef<WebSocket | null>(null);

    useEffect(() => {
        const wsUrl = "ws://127.0.0.1:8080/ws/";
        const socket = new WebSocket(wsUrl);

        wsRef.current = socket;
        setWs(socket);

        socket.onopen = () => {
            console.log("WebSocket connection established");
          };
      
          socket.onmessage = (event) => {
            console.log("Received message from server:", event.data);
            // handle msgs
          };
      
          socket.onclose = () => {
            console.log("WebSocket connection closed");
          };
      
          socket.onerror = (error) => {
            console.error("WebSocket error:", error);
          };
      
          return () => {
            socket.close();
          };
    }, [])

    const sendMessage = (message: string) => {
        if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
          wsRef.current.send(message);
        }
      };
    
      return (
        <WebSocketCtx.Provider value={{ ws, sendMessage }}>
          {children}
        </WebSocketCtx.Provider>
      );
}