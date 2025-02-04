import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SceneComponent } from '../scene/scene.component';

@Component({
  selector: 'app-communist-session',
  templateUrl: './communist-session.component.html',
  styleUrls: ['./communist-session.component.scss'],
  standalone: true,
  imports: [CommonModule, SceneComponent]
})
export class CommunistSessionComponent {
  @Input() type?: string;
  @Input() phase?: string;

  getMessage(): string {
    if (!this.type || !this.phase) return '';
    
    switch (this.phase) {
      case 'start':
        return this.getStartMessage();
      case 'action':
        return this.getActionMessage();
      case 'result':
        return this.getResultMessage();
      default:
        return '';
    }
  }

  private getStartMessage(): string {
    if (!this.type) return '';
    switch (this.type) {
      case 'CommunistSession':
        return 'Komünist oturum başlıyor!';
      case 'FiveYearPlan':
        return 'Beş yıllık plan başlıyor!';
      case 'Confession':
        return 'İtiraf zamanı!';
      default:
        return '';
    }
  }

  private getActionMessage(): string {
    if (!this.type) return '';
    switch (this.type) {
      case 'CommunistSession':
        return 'Kararlar alınıyor...';
      case 'FiveYearPlan':
        return 'Plan hazırlanıyor...';
      case 'Confession':
        return 'İtiraf bekleniyor...';
      default:
        return '';
    }
  }

  private getResultMessage(): string {
    if (!this.type) return '';
    switch (this.type) {
      case 'CommunistSession':
        return 'Oturum tamamlandı!';
      case 'FiveYearPlan':
        return 'Plan tamamlandı!';
      case 'Confession':
        return 'İtiraf alındı!';
      default:
        return '';
    }
  }

  getPhaseMessage(): string {
    return 'Komünist Oturum';
  }

  getPhaseDescription(): string {
    return 'Komünist oturum devam ediyor...';
  }
} 