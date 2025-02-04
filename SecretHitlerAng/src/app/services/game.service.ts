import { Injectable } from '@angular/core';
import { WebSocketService } from './websocket.service';
import { BehaviorSubject } from 'rxjs';
import { GameState } from '../dm/state';

@Injectable({
  providedIn: 'root'
})
export class GameService {
  private gameStateSubject = new BehaviorSubject<GameState | undefined>(undefined);
  gameState$ = this.gameStateSubject.asObservable();

  constructor(private ws: WebSocketService) {
    this.ws.state$.subscribe((state: GameState | undefined) => {
      this.gameStateSubject.next(state);
    });
  }

  createGame(): void {
    this.ws.createGame();
  }

  joinGame(gameId: string): void {
    this.ws.joinAsBoard(gameId);
  }

  performAction(action: unknown): void {
    this.ws.boardAction(action);
  }

  getPlayerName(index: number): string | undefined {
    return this.gameStateSubject.value?.players[index]?.name;
  }

  isPlayerConnected(index: number): boolean {
    return this.gameStateSubject.value?.players[index]?.connected ?? false;
  }

  getCurrentState(): GameState | undefined {
    return this.gameStateSubject.value;
  }
} 