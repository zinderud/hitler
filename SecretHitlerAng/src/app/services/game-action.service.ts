import { Injectable } from '@angular/core';
import { WebSocketService } from './websocket.service';
import { GameAction, Policy } from '../dm/state';

@Injectable({
  providedIn: 'root'
})
export class GameActionService {
  constructor(private ws: WebSocketService) {}

  createGame(playerName: string) {
    this.ws.send({
      type: 'CreateGame',
      playerName
    });
  }

  joinGame(gameId: string, playerName: string) {
    this.ws.send({
      type: 'JoinGame',
      gameId,
      playerName
    });
  }

  vote(vote: boolean) {
    this.ws.send({
      type: 'Vote',
      vote
    });
  }

  selectPolicies(policies: Policy[]) {
    this.ws.send({
      type: 'SelectPolicies',
      policies
    });
  }

  selectPlayer(playerId: number) {
    this.ws.send({
      type: 'SelectPlayer',
      playerId
    });
  }

  endCardReveal() {
    this.ws.send({
      type: 'EndCardReveal'
    });
  }

  investigatePlayer(playerId: number): void {
    this.sendAction({
      type: 'investigate',
      target: playerId
    });
  }

  executePlayer(playerId: number): void {
    this.sendAction({
      type: 'execute',
      target: playerId
    });
  }

  specialElection(playerId: number): void {
    this.sendAction({
      type: 'special_election',
      target: playerId
    });
  }

  private sendAction(action: GameAction): void {
    this.ws.boardAction(action);
  }
} 