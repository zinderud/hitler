import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SceneComponent } from '../scene/scene.component';
import { GameStateManagerService } from '../../../services/game-state-manager.service';
import { GameActionService } from '../../../services/game-action.service';
import { SoundService } from '../../../services/sound.service';

@Component({
  selector: 'app-election',
  templateUrl: './election.component.html',
  styleUrls: ['./election.component.scss'],
  standalone: true,
  imports: [CommonModule, SceneComponent]
})
export class ElectionComponent {
  @Input() president?: number;
  @Input() chancellor?: number;
  @Input() outcome?: boolean;

  constructor(
    private gameStateManager: GameStateManagerService,
    private gameAction: GameActionService,
    private soundService: SoundService
  ) {}

  get showPresident(): boolean {
    return this.president !== undefined;
  }

  get showChancellor(): boolean {
    return this.chancellor !== undefined;
  }

  get showVoting(): boolean {
    return this.outcome === undefined;
  }

  get election() {
    return {
      president: this.president,
      chancellor: this.chancellor,
      outcome: this.outcome
    };
  }

  getPlayerName(index: number | undefined): string {
    if (index === undefined) return '';
    const state = this.gameStateManager.getCurrentState();
    return state.type === 'board' ? state.players[index]?.name || '' : '';
  }

  vote(vote: boolean): void {
    this.soundService.playSound('vote');
    this.gameAction.vote(vote);
  }
} 