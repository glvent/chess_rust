"use client";

import React from 'react';
import { DndProvider, DndProviderProps } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';

interface ClientProviderProps {
  children: React.ReactNode;
}

const ClientProvider: React.FC<ClientProviderProps> = ({ children }) => {
  return (
    <DndProvider backend={HTML5Backend}>
      {children}
    </DndProvider>
  );
};

export default ClientProvider;
