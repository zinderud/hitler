import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SceneComponent } from '../scene/scene.component';
import { GameStateManagerService } from '../../../services/game-state-manager.service';
import { GameActionService } from '../../../services/game-action.service';
import { SoundService } from '../../../services/sound.service';

@Component({
  selector: 'app-executive-action',
  templateUrl: './executive-action.component.html',
  styleUrls: ['./executive-action.component.scss'],
  standalone: true,
  imports: [CommonModule, SceneComponent]
})
export class ExecutiveActionComponent {
  @Input() type?: string;
  @Input() chosenPlayer?: number;
  showTimeout: boolean = false;

  constructor(
    private gameStateManager: GameStateManagerService,
    private gameAction: GameActionService,
    private soundService: SoundService
  ) {}

  get title(): string {
    if (!this.type) return '';
    switch (this.type) {
      case 'PolicyPeak':
        return 'Politika İnceleme';
      case 'InvestigatePlayer':
        return 'Oyuncu İnceleme';
      case 'SpecialElection':
        return 'Özel Seçim';
      case 'Execution':
        return 'İdam';
      default:
        return '';
    }
  }

  get subtitle(): string {
    if (!this.type) return '';
    switch (this.type) {
      case 'PolicyPeak':
        return 'Gelecek 3 politikayı görün';
      case 'InvestigatePlayer':
        return 'Bir oyuncunun parti kimliğini görün';
      case 'SpecialElection':
        return 'Bir sonraki başkanı seçin';
      case 'Execution':
        return 'Bir oyuncuyu idam edin';
      default:
        return '';
    }
  }

  get player(): string {
    return this.chosenPlayer !== undefined ? `Oyuncu ${this.chosenPlayer + 1}` : '';
  }

  get actionType(): string {
    return this.type || '';
  }

  getPlayerName(index: number | undefined): string {
    if (index === undefined) return '';
    const state = this.gameStateManager.getCurrentState();
    return state.type === 'board' ? state.players[index]?.name || '' : '';
  }

  executeAction(playerId: number): void {
    switch (this.actionType) {
      case 'InvestigatePlayer':
        this.gameAction.investigatePlayer(playerId);
        break;
      case 'SpecialElection':
        this.gameAction.specialElection(playerId);
        break;
      case 'Execution':
        this.gameAction.executePlayer(playerId);
        break;
    }
  }

  onTimeout(): void {
    this.showTimeout = true;
    this.soundService.playSound('timeout');
  }
} 