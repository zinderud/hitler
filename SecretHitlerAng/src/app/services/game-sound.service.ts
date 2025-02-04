import { Injectable } from '@angular/core';
import { SoundService } from './sound.service';
import { environment } from '../../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class GameSoundService {
  private readonly SOUNDS = {
    vote: 'assets/sounds/vote.mp3',
    policy: 'assets/sounds/policy.mp3',
    execute: 'assets/sounds/execute.mp3',
    victory: 'assets/sounds/victory.mp3',
    defeat: 'assets/sounds/defeat.mp3',
    notification: 'assets/sounds/notification.mp3'
  };

  constructor(private soundService: SoundService) {}

  playVoteSound(): void {
    if (environment.soundEnabled) {
      this.soundService.playSound(this.SOUNDS.vote, { volume: environment.defaultVolume });
    }
  }

  playPolicySound(): void {
    if (environment.soundEnabled) {
      this.soundService.playSound(this.SOUNDS.policy, { volume: environment.defaultVolume });
    }
  }

  playExecuteSound(): void {
    if (environment.soundEnabled) {
      this.soundService.playSound(this.SOUNDS.execute, { volume: environment.defaultVolume });
    }
  }

  playVictorySound(): void {
    if (environment.soundEnabled) {
      this.soundService.playSound(this.SOUNDS.victory, { volume: environment.defaultVolume });
    }
  }

  playDefeatSound(): void {
    if (environment.soundEnabled) {
      this.soundService.playSound(this.SOUNDS.defeat, { volume: environment.defaultVolume });
    }
  }

  playNotificationSound(): void {
    if (environment.soundEnabled) {
      this.soundService.playSound(this.SOUNDS.notification, { volume: environment.defaultVolume });
    }
  }

  stopAllSounds(): void {
    this.soundService.stopAllSounds();
  }
} 