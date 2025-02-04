import { Component, Input } from '@angular/core';
import { GameStateManagerService } from '../../services/game-state-manager.service';

@Component({
  selector: 'app-lobby',
  templateUrl: './lobby.component.html',
  styleUrls: ['./lobby.component.css']
})
export class LobbyComponent {
  @Input() gameId!: string;
  
  constructor(private gameStateManager: GameStateManagerService) {}

  getConnectedPlayers(): number {
    const state = this.gameStateManager.getCurrentState();
    if (state.type !== 'board') return 0;
    return state.players.filter(p => p.connected).length;
  }

  getTotalPlayers(): number {
    const state = this.gameStateManager.getCurrentState();
    if (state.type !== 'board') return 0;
    return state.players.length;
  }

  copyGameId() {
    navigator.clipboard.writeText(this.gameId);
  }
} 