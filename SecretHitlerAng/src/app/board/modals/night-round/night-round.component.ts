import { Component, OnInit } from '@angular/core';
import { SoundService } from '../../../services/sound.service';
import { AnimationService } from '../../../services/animation.service';

@Component({
  selector: 'app-night-round',
  templateUrl: './night-round.component.html',
  styleUrls: ['./night-round.component.css']
})
export class NightRoundComponent implements OnInit {
  showMessage: boolean = false;

  constructor(
    private soundService: SoundService,
    private animationService: AnimationService
  ) {}

  ngOnInit(): void {
    setTimeout(() => {
      this.showMessage = true;
      if (this.animationService.isEnabled()) {
        this.soundService.playSound('notification');
      }
    }, 1000);
  }
} 