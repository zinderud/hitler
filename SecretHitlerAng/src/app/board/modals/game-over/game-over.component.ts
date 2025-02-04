import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SceneComponent } from '../scene/scene.component';

@Component({
  selector: 'app-game-over',
  templateUrl: './game-over.component.html',
  styleUrls: ['./game-over.component.scss'],
  standalone: true,
  imports: [CommonModule, SceneComponent]
})
export class GameOverComponent {
  @Input() result?: 'victory' | 'defeat';

  get outcome(): string {
    return this.result || '';
  }
} 