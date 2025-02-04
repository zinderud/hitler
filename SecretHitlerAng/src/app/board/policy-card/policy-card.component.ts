import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-policy-card',
  templateUrl: './policy-card.component.html',
  styleUrls: ['./policy-card.component.scss'],
  standalone: true,
  imports: [CommonModule]
})
export class PolicyCardComponent {
  @Input() party?: string;
  @Input() position?: { x: number, y: number };
  @Input() scale: number = 1;
  @Input() isRevealed: boolean = false;

  getPartyClass(): string {
    return this.party?.toLowerCase() || '';
  }

  getPartyIcon(): string {
    switch (this.party) {
      case 'Liberal':
        return '⚖️';
      case 'Fascist':
        return '⚡';
      case 'Communist':
        return '☭';
      default:
        return '';
    }
  }

  getStyles() {
    if (!this.position) return {};
    return {
      transform: `translate(${this.position.x}px, ${this.position.y}px) scale(${this.scale})`,
      opacity: this.isRevealed ? '1' : '0.5'
    };
  }
} 