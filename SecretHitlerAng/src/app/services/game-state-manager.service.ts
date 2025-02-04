import { Injectable } from '@angular/core';
import { BehaviorSubject, Subject } from 'rxjs';
import { WebSocketService } from './websocket.service';
import { GameStateType, BoardPrompt, BoardState } from '../dm/state';
import { filter } from 'rxjs/operators';

interface GameStateMessage {
  type: 'GameState';
  state: GameStateType;
}

@Injectable({
  providedIn: 'root'
})
export class GameStateManagerService {
  private gameStateSubject = new BehaviorSubject<GameStateType>({ type: 'connecting' });
  private promptSubject = new Subject<BoardPrompt>();

  gameState$ = this.gameStateSubject.asObservable();
  prompt$ = this.promptSubject.asObservable();

  constructor(private ws: WebSocketService) {
    this.ws.messages$.pipe(
      filter((message): message is GameStateMessage => message.type === 'GameState')
    ).subscribe((message) => {
      if (message.type === 'GameState') {
        this.gameStateSubject.next(message.state);
        if (message.state.type === 'board' && message.state.prompt) {
          this.promptSubject.next(message.state.prompt as BoardPrompt);
        }
      }
    });
  }

  getCurrentState(): GameStateType {
    return this.gameStateSubject.getValue();
  }

  updateState(message: any): void {
    if (message.state) {
      this.gameStateSubject.next(message.state);
      if (message.state.type === 'board' && message.state.prompt) {
        this.promptSubject.next(message.state.prompt as BoardPrompt);
      }
    }
  }

  isGameActive(): boolean {
    const state = this.getCurrentState();
    return state.type === 'board';
  }

  getElectionTracker(): number {
    const state = this.getCurrentState();
    return state.type === 'board' ? state.election_tracker : 0;
  }

  getDeckSize(): number {
    const state = this.getCurrentState();
    return state.type === 'board' ? state.deck_size : 0;
  }

  getLiberalPolicies(): number {
    const state = this.getCurrentState();
    return state.type === 'board' ? state.liberal_cards : 0;
  }

  getFascistPolicies(): number {
    const state = this.getCurrentState();
    return state.type === 'board' ? state.fascist_cards : 0;
  }

  getCommunistPolicies(): number {
    const state = this.getCurrentState();
    return state.type === 'board' ? (state.communist_cards || 0) : 0;
  }
} 