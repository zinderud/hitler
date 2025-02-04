import { Component, OnInit } from '@angular/core';
import { GameStateManagerService } from '../../services/game-state-manager.service';

@Component({
  selector: 'app-live-header',
  templateUrl: './live-header.component.html',
  styleUrls: ['./live-header.component.css']
})
export class LiveHeaderComponent implements OnInit {
  isConnected: boolean = false;

  constructor(private gameStateManager: GameStateManagerService) {}

  ngOnInit() {
    this.gameStateManager.gameState$.subscribe(state => {
      this.isConnected = state.type !== 'connecting';
    });
  }
} 