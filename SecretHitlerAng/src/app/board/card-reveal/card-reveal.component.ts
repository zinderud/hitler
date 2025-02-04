import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { PolicyCardComponent } from '../policy-card/policy-card.component';
import { Policy } from '../../dm/state';

@Component({
  selector: 'app-card-reveal',
  templateUrl: './card-reveal.component.html',
  styleUrls: ['./card-reveal.component.scss'],
  standalone: true,
  imports: [CommonModule, PolicyCardComponent]
})
export class CardRevealComponent {
  @Input() reveal: any;
  @Input() scale: number = 1;
  @Output() done = new EventEmitter<void>();

  currentIndex: number = 0;
  isRevealing: boolean = false;
  policies: Policy[] = [];

  ngOnInit() {
    if (this.reveal.type === 'CardReveal' && this.reveal.party) {
      this.policies = [this.reveal.party as Policy];
      this.startReveal();
    }
  }

  startReveal() {
    this.isRevealing = true;
    this.currentIndex = 0;
    setTimeout(() => {
      this.isRevealing = false;
      setTimeout(() => {
        this.done.emit();
      }, 1500);
    }, 1000);
  }

  getPosition(index: number) {
    return {
      x: 0,
      y: 0
    };
  }
} 