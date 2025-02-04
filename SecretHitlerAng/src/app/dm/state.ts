export type Policy = 'Liberal' | 'Fascist' | 'Communist';

export interface Player {
  name: string;
  isAlive: boolean;
  isPresident: boolean;
  isChancellor: boolean;
  connected: boolean;
}

export interface BoardState {
  type: 'board';
  players: Player[];
  liberal_cards: number;
  fascist_cards: number;
  communist_cards: number;
  election_tracker: number;
  deck_size: number;
  prompt?: BoardPrompt;
}

export interface LobbyState {
  type: 'lobby';
  game_id: string;
}

export interface ConnectingState {
  type: 'connecting';
}

export type GameStateType = BoardState | LobbyState | ConnectingState;

export interface GameAction {
  type: string;
  playerId?: number;
  vote?: boolean;
  policies?: Policy[];
  target?: number;
}

export interface BoardPrompt {
  type: 'Night' | 'Election' | 'LegislativeSession' | 'PolicyPeak' | 
        'InvestigatePlayer' | 'SpecialElection' | 'Execution' |
        'CommunistSession' | 'FiveYearPlan' | 'Confession' | 'GameOver' |
        'CardReveal';
  president?: number;
  chancellor?: number;
  outcome?: boolean;
  chosen_player?: number;
  phase?: string;
  party?: string;
  result?: 'victory' | 'defeat';
}

export type ErrorType = 'notfound' | 'toomanyplayers' | 'inprogress';

export interface ErrorState {
  type: 'error';
  error: ErrorType;
} 