"use client";

import React, { useRef } from 'react';
import Image from 'next/image';
import { useDrag } from 'react-dnd';

interface PieceProps {
  type: string;
  color: 'w' | 'b';
  position: number;
}

const Piece: React.FC<PieceProps> = ({ type, color, position }) => {
  const pieceAsset = `/assets/${color}${type}.svg`;

  const ref = useRef<HTMLSpanElement>(null);

  const [{ isDragging }, drag] = useDrag({
    type: 'piece',
    item: { type, color, position },
    collect: (monitor) => ({
      isDragging: !!monitor.isDragging(),
    }),
  });

  drag(ref);

  return (
    <span
      ref={ref}
      style={{
        opacity: isDragging ? 0.5 : 1,
        cursor: 'move',
      }}
    >
      <Image src={pieceAsset} width={64} height={64} alt={`${color}${type}`} />
    </span>
  );
};

export default Piece;
