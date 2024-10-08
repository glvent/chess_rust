"use client"

import Image from "next/image";

interface PieceProps {
  type: string;
  color: 'w' | 'b';
}

const Piece: React.FC<PieceProps> = ({ type, color }) => {
  const pieceAsset = `/assets/${color + type}.svg`;

  return (
    <span>
      <img src={pieceAsset} width={48} height={48} alt={color + type} />
    </span>
  );
};

export default Piece;
