"use client"

import React from 'react';

interface SquareProps {
  isBlack: boolean;
  isFlipped: boolean;
  row: number;
  col: number;
  children?: React.ReactNode;
}

const Square: React.FC<SquareProps> = ({ isBlack, isFlipped, children, row, col }) => {
  const backgroundColor = isBlack ? 'bg-green-700' : 'bg-green-200';
  const textColor = isBlack ? 'text-green-200' : 'text-green-700';

  const fileLabels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

  const rankLabel = 8 - row;
  const fileLabel = fileLabels[col];

  const isFirstCol = isFlipped ? col === 7 : col === 0;
  const isFirstRow = isFlipped ? row === 0 : row === 7;

  return (
    <div
      className={`relative w-16 h-16 ${backgroundColor} ${textColor} flex items-center justify-center`}
    >
      {isFirstCol && <span className='absolute top-1 left-1 text-xs'>{rankLabel}</span>}
      {isFirstRow && <span className='absolute bottom-1 right-1 text-xs'>{fileLabel}</span>}
      {children}
    </div>
  );
};

export default Square;
