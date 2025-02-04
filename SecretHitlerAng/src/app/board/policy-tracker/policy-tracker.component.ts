import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-policy-tracker',
  templateUrl: './policy-tracker.component.html',
  styleUrls: ['./policy-tracker.component.css'],
  standalone: true,
  imports: [CommonModule]
})
export class PolicyTrackerComponent {
  @Input() party!: string;
  @Input() numPlayers: number = 0;
  @Input() scale: number = 1;

  getPartyClass(): string {
    return this.party.toLowerCase();
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

  getSlots(): number[] {
    const numSlots = this.party === 'Fascist' ? 6 : 5;
    return Array(numSlots).fill(0).map((_, i) => i);
  }

  getActionText(index: number): string {
    if (this.party === 'Fascist') {
      if (this.numPlayers >= 9) {
        switch (index) {
          case 1: return 'Araştır';
          case 2: return 'Özel Seçim';
          case 3: return 'İdam';
          case 4: return 'İdam';
          default: return '';
        }
      } else {
        switch (index) {
          case 2: return 'Araştır';
          case 3: return 'Özel Seçim';
          case 4: return 'İdam';
          default: return '';
        }
      }
    }
    return '';
  }
} 