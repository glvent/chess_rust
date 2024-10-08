"use client";

import React, { useRef } from 'react';
import { useDrop } from 'react-dnd';

interface SquareProps {
  isBlack: boolean;
  isFlipped: boolean;
  row: number;
  col: number;
  onPieceMove: (fromPosition: number, toPosition?: number) => void;
  isSelected: boolean;
  children?: React.ReactNode;
}

const Square: React.FC<SquareProps> = ({
  isBlack,
  isFlipped,
  children,
  row,
  col,
  onPieceMove,
  isSelected,
}) => {
  const ref = useRef<HTMLDivElement>(null);

  const [{ isOver }, drop] = useDrop({
    accept: 'piece',
    drop: (item: { type: string; color: 'w' | 'b'; position: number }) => {
      const toPosition = (7 - row) * 8 + col;
      onPieceMove(item.position, toPosition);
    },
    collect: (monitor) => ({
      isOver: !!monitor.isOver(),
    }),
  });

  drop(ref);

  const backgroundColor = isBlack ? 'bg-green-700' : 'bg-green-200';
  const textColor = isBlack ? 'text-green-200' : 'text-green-700';

  const squareHighlight = isSelected
    ? 'bg-yellow-300'
    : isOver
    ? 'bg-yellow-500'
    : backgroundColor;

  const fileLabels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

  const rankLabel = isFlipped ? row + 1 : 8 - row;
  const fileLabel = isFlipped ? fileLabels[7 - col] : fileLabels[col];

  const isFirstCol = isFlipped ? col === 7 : col === 0;
  const isFirstRow = isFlipped ? row === 0 : row === 7;

  const handleClick = () => {
    const squarePosition = (7 - row) * 8 + col;
    onPieceMove(squarePosition);
  };

  return (
    <div
      ref={ref}
      className={`relative w-16 h-16 ${squareHighlight} ${textColor} flex items-center justify-center`}
      onClick={handleClick}
    >
      {isFirstCol && (
        <span className="absolute top-1 left-1 text-xs">
          {rankLabel}
        </span>
      )}
      {isFirstRow && (
        <span className="absolute bottom-1 right-1 text-xs">
          {fileLabel}
        </span>
      )}
      {children}
    </div>
  );
};

export default Square;
