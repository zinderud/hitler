import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { GameStateManagerService } from '../../services/game-state-manager.service';
import { GameActionService } from '../../services/game-action.service';

@Component({
  selector: 'app-join-game',
  templateUrl: './join-game.component.html',
  styleUrls: ['./join-game.component.css'],
  standalone: true,
  imports: [CommonModule, FormsModule]
})
export class JoinGameComponent {
  playerName: string = '';
  gameId: string = '';
  error: string = '';

  constructor(
    private gameStateManager: GameStateManagerService,
    private gameAction: GameActionService
  ) {}

  createGame() {
    if (!this.playerName.trim()) {
      this.error = 'Lütfen bir oyuncu adı girin';
      return;
    }
    this.error = '';
    this.gameAction.createGame(this.playerName);
  }

  joinGame() {
    if (!this.playerName.trim()) {
      this.error = 'Lütfen bir oyuncu adı girin';
      return;
    }
    if (!this.gameId.trim()) {
      this.error = 'Lütfen bir oyun ID girin';
      return;
    }
    this.error = '';
    this.gameAction.joinGame(this.gameId, this.playerName);
  }
} 