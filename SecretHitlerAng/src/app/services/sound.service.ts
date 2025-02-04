import { Injectable } from '@angular/core';
import { environment } from '../../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class SoundService {
  private sounds: { [key: string]: HTMLAudioElement } = {};
  private enabled: boolean = environment.soundEnabled;

  constructor() {
    this.preloadSounds();
  }

  private preloadSounds() {
    const soundFiles = {
      vote: 'assets/sounds/vote.mp3',
      policy: 'assets/sounds/policy.mp3',
      execute: 'assets/sounds/execute.mp3',
      victory: 'assets/sounds/victory.mp3',
      defeat: 'assets/sounds/defeat.mp3',
      notification: 'assets/sounds/notification.mp3'
    };

    Object.entries(soundFiles).forEach(([key, path]) => {
      const audio = new Audio(path);
      audio.preload = 'auto';
      this.sounds[key] = audio;
    });
  }

  playSound(soundName: string) {
    if (!this.enabled) return;

    const sound = this.sounds[soundName];
    if (sound) {
      sound.currentTime = 0;
      sound.play().catch(error => {
        console.error('Ses çalma hatası:', error);
      });
    }
  }

  toggleSound(enabled: boolean) {
    this.enabled = enabled;
  }
} 