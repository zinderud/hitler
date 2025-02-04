import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GameStateManagerService } from '../../services/game-state-manager.service';
import { GameStateType } from '../../dm/state';

@Component({
  selector: 'app-player-rail',
  templateUrl: './player-rail.component.html',
  styleUrls: ['./player-rail.component.scss'],
  standalone: true,
  imports: [CommonModule]
})
export class PlayerRailComponent {
  state: GameStateType;

  constructor(private gameStateManager: GameStateManagerService) {
    this.state = this.gameStateManager.getCurrentState();
    this.gameStateManager.gameState$.subscribe(state => {
      this.state = state;
    });
  }

  getPlayerStatus(i: number): string {
    const state = this.gameStateManager.getCurrentState();
    return this.getPlayerStatusFromState(state, i);
  }

  getPlayerName(i: number): string {
    const state = this.gameStateManager.getCurrentState();
    return this.getPlayerNameFromState(state, i);
  }

  getElectionTracker(): number {
    return this.gameStateManager.getElectionTracker();
  }

  getDeckSize(): number {
    return this.gameStateManager.getDeckSize();
  }

  private getPlayerStatusFromState(state: GameStateType, index: number): string {
    if (state.type !== 'board') return '';
    if (!state.players[index].isAlive) return 'dead';
    if (state.players[index].isPresident) return 'president';
    if (state.players[index].isChancellor) return 'chancellor';
    return '';
  }

  private getPlayerNameFromState(state: GameStateType, index: number): string {
    if (state.type !== 'board') return `Oyuncu ${index + 1}`;
    return state.players?.[index]?.name || `Oyuncu ${index + 1}`;
  }
} 