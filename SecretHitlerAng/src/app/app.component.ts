import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
 
import { GameStateManagerService } from './services/game-state-manager.service';
import { SoundService } from './services/sound.service';
import { AnimationService } from './services/animation.service';
import { GameStateType, ErrorState } from './dm/state';
import { LiveHeaderComponent } from './components/live-header/live-header.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
  imports: [
    RouterOutlet,
    LiveHeaderComponent
  ],
  standalone: true
})
export class AppComponent implements OnInit {
  gameState?: GameStateType | ErrorState;

  constructor(
    private gameStateManager: GameStateManagerService,
    private soundService: SoundService,
    private animationService: AnimationService
  ) {}

  ngOnInit(): void {
    this.gameStateManager.gameState$.subscribe(state => {
      this.gameState = state;
      if (state.type === 'board' && state.prompt) {
        switch (state.prompt.type) {
          case 'Election':
            this.soundService.playSound('vote');
            break;
          case 'CardReveal':
            this.soundService.playSound('policy');
            break;
          case 'Execution':
            this.soundService.playSound('execute');
            break;
          case 'GameOver':
            this.soundService.playSound(state.prompt.result === 'victory' ? 'victory' : 'defeat');
            break;
        }
      }
    });
  }

  get isConnecting(): boolean {
    return this.gameState?.type === 'connecting';
  }

  get hasError(): boolean {
    return (this.gameState as ErrorState)?.type === 'error';
  }

  getErrorMessage(): string {
    const errorState = this.gameState as ErrorState;
    if (errorState?.type !== 'error') return 'Bilinmeyen hata';
    
    switch (errorState.error) {
      case 'notfound':
        return 'Oyun bulunamadı';
      case 'toomanyplayers':
        return 'Oyun dolu';
      case 'inprogress':
        return 'Oyun devam ediyor';
      default:
        return 'Bilinmeyen hata';
    }
  }
} 