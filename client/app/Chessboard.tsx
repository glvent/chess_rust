// components/Chessboard.tsx
'use client';

import { useState } from 'react';
import Square from './Square';
import Piece from './Piece';

interface PieceData {
  piece_type: string;
  color: 'w' | 'b';
}

// Initial board state
const initialBoardState: (PieceData | null)[][] = [
  [
    { piece_type: 'r', color: 'b' },
    { piece_type: 'n', color: 'b' },
    { piece_type: 'b', color: 'b' },
    { piece_type: 'q', color: 'b' },
    { piece_type: 'k', color: 'b' },
    { piece_type: 'b', color: 'b' },
    { piece_type: 'n', color: 'b' },
    { piece_type: 'r', color: 'b' },
  ],
  Array(8).fill({ piece_type: 'p', color: 'b' }),
  ...Array(4).fill(Array(8).fill(null)),
  Array(8).fill({ piece_type: 'p', color: 'w' }),
  [
    { piece_type: 'r', color: 'w' },
    { piece_type: 'n', color: 'w' },
    { piece_type: 'b', color: 'w' },
    { piece_type: 'q', color: 'w' },
    { piece_type: 'k', color: 'w' },
    { piece_type: 'b', color: 'w' },
    { piece_type: 'n', color: 'w' },
    { piece_type: 'r', color: 'w' },
  ],
];

const Chessboard: React.FC = () => {
  const [boardState] = useState<(PieceData | null)[][]>(initialBoardState);
  const [isFlipped, setIsFlipped] = useState<boolean>(false);

  const renderSquare = (row: number, col: number) => {
    const actualRow = isFlipped ? 7 - row : row;
    const actualCol = isFlipped ? 7 - col : col;

    const isBlack = (actualRow + actualCol) % 2 === 1;
    const piece = boardState[actualRow][actualCol];

    return (
      <Square
        key={`${row}-${col}`}
        isBlack={isBlack}
        isFlipped={isFlipped}
        row={actualRow}
        col={actualCol}
      >
        {piece && <Piece type={piece.piece_type} color={piece.color} />}
      </Square>
    );
  };

  const board = [];
  // Do not reverse the rows
  for (let row = 0; row < 8; row++) {
    const squares = [];
    // Do not reverse the squares
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
