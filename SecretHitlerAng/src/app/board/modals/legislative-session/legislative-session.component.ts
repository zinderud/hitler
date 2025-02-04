import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SceneComponent } from '../scene/scene.component';
import { GameStateManagerService } from '../../../services/game-state-manager.service';
import { GameActionService } from '../../../services/game-action.service';
import { SoundService } from '../../../services/sound.service';
import { Policy } from '../../../dm/state';

@Component({
  selector: 'app-legislative-session',
  templateUrl: './legislative-session.component.html',
  styleUrls: ['./legislative-session.component.scss'],
  standalone: true,
  imports: [CommonModule, SceneComponent]
})
export class LegislativeSessionComponent {
  @Input() president?: number;
  @Input() chancellor?: number;
  @Input() phase?: string;

  selectedPolicies: Policy[] = [];

  constructor(
    private gameStateManager: GameStateManagerService,
    private gameAction: GameActionService,
    private soundService: SoundService
  ) {}

  getPlayerName(index: number | undefined): string {
    if (index === undefined) return '';
    const state = this.gameStateManager.getCurrentState();
    return state.type === 'board' ? state.players[index]?.name || '' : '';
  }

  getCurrentActor(): string {
    if (!this.phase) return '';
    switch (this.phase) {
      case 'president_draw':
      case 'president_discard':
        return this.getPlayerName(this.president);
      case 'chancellor_discard':
        return this.getPlayerName(this.chancellor);
      default:
        return '';
    }
  }

  getPhaseMessage(): string {
    if (!this.phase) return '';
    switch (this.phase) {
      case 'president_draw':
        return 'Başkan 3 politika kartı çekiyor...';
      case 'president_discard':
        return 'Başkan bir kart seçip atmalı';
      case 'chancellor_discard':
        return 'Şansölye bir kart seçip atmalı';
      default:
        return '';
    }
  }

  onPolicySelected(policies: Policy[]): void {
    this.selectedPolicies = policies;
    this.soundService.playSound('policy');
    this.gameAction.selectPolicies(policies);
  }
} 