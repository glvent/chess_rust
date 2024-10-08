import Chessboard from "./Chessboard";
import ClientProvider from "./ClientProvider";
import { WebSocketProvider } from "./contexts/WebSocketContext";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";

export default function Home() {
  // make sure to move to Chessboard page so WS ctx is not always open...
  return (
    <ClientProvider>
      <WebSocketProvider>
        <div className="flex justify-center items-center min-h-screen bg-neutral-900 text-white">
          <Chessboard />
        </div>
      </WebSocketProvider>
    </ClientProvider>
  );
}
