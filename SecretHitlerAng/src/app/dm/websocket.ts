export interface WebSocketMessage {
  type: 'GameState' | string;
  // ... other properties ...
}

export interface GameStateMessage {
  type: 'GameState';
  state: GameStateType;
} 