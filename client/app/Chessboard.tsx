// components/Chessboard.tsx

"use client";

import { useState, useContext, useEffect } from "react";
import Square from "./Square";
import Piece from "./Piece";
import { WebSocketCtx } from "./contexts/WebSocketContext";

interface PieceData {
  piece_type: string;
  color: "w" | "b";
  position: number; // 0 to 63
}

const Chessboard: React.FC = () => {
  const [pieces, setPieces] = useState<PieceData[] | null>(null);
  const [isFlipped, setIsFlipped] = useState<boolean>(false);
  const [selectedPiece, setSelectedPiece] = useState<PieceData | null>(null);

  const { ws } = useContext(WebSocketCtx);

  useEffect(() => {
    if (!ws) return;

    const handleMessage = (event: MessageEvent) => {
      try {
        const message = JSON.parse(event.data);

        if (message.type === "init" || message.type === "update") {
          setPieces(message.data);
          console.log(message.data);
        }
      } catch (error) {
        console.error("Error parsing message:", error);
      }
    };

    ws.addEventListener("message", handleMessage);

    return () => {
      ws.removeEventListener("message", handleMessage);
    };
  }, [ws]);

  if (!pieces) {
    return <div>Loading...</div>;
  }

  const boardState: (PieceData | null)[][] = Array.from({ length: 8 }, () =>
    Array(8).fill(null)
  );

  pieces.forEach((piece) => {
    const row = 7 - Math.floor(piece.position / 8); // invert row idx
    const col = piece.position % 8;
    boardState[row][col] = piece;
  });

  const handlePieceMove = (fromOrToPosition: number, toPosition?: number) => {
    console.log(`fromOrToPosition: ${fromOrToPosition}, toPosition: ${toPosition}`);
    if (fromOrToPosition == selectedPiece?.position) {
      setSelectedPiece(null);
      return;
    }
  
    if (toPosition !== undefined) {
      const moveMessage = {
        type: 'move',
        data: {
          from: fromOrToPosition,
          to: toPosition,
        },
      };
      ws?.send(JSON.stringify(moveMessage));
      setSelectedPiece(null);
    } else if (selectedPiece) {
      const fromPosition = selectedPiece.position;
      const toPosition = fromOrToPosition;
      const moveMessage = {
        type: 'move',
        data: {
          from: fromPosition,
          to: toPosition,
        },
      };
      ws?.send(JSON.stringify(moveMessage));
      setSelectedPiece(null);
    } else {
      const piece = pieces.find((p) => p.position === fromOrToPosition);
      if (piece) {
        setSelectedPiece(piece);
      } else {
        setSelectedPiece(null);
      }
    }
  };
  

  const renderSquare = (row: number, col: number) => {
    const actualRow = isFlipped ? 7 - row : row;
    const actualCol = isFlipped ? 7 - col : col;

    const isBlack = (actualRow + actualCol) % 2 === 1;
    const piece = boardState[actualRow][actualCol];

    const position = (7 - actualRow) * 8 + actualCol;

    const isSelected = selectedPiece?.position === position;

    return (
      <Square
        key={`${row}-${col}`}
        isBlack={isBlack}
        isFlipped={isFlipped}
        row={actualRow}
        col={actualCol}
        onPieceMove={handlePieceMove}
        isSelected={isSelected}
      >
        {piece && (
          <Piece
            type={piece.piece_type}
            color={piece.color}
            position={piece.position}
          />
        )}
      </Square>
    );
  };

  const board = [];
  for (let row = 0; row < 8; row++) {
    const squares = [];
    for (let col = 0; col < 8; col++) {
      squares.push(renderSquare(row, col));
    }
    board.push(
      <div key={row} className="flex">
        {squares}
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center">
      <div>{board}</div>
      <button
        onClick={() => setIsFlipped(!isFlipped)}
        className="mt-4 px-4 py-2 bg-blue-500 text-white rounded"
      >
        Flip Board
      </button>
    </div>
  );
};

export default Chessboard;
