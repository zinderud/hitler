import { Component, OnInit } from '@angular/core';
import { CommonModule, NgIf } from '@angular/common';
import { GameStateManagerService } from '../services/game-state-manager.service';
import { GameStateType, BoardPrompt } from '../dm/state';

// Bileşen importları
import { JoinGameComponent } from './join-game/join-game.component';
import { LobbyComponent } from './lobby/lobby.component';
import { BoardContentComponent } from './board-content/board-content.component';
import { PlayerRailComponent } from './player-rail/player-rail.component';
import { SceneComponent } from './modals/scene/scene.component';
import { NightRoundComponent } from './modals/night-round/night-round.component';
import { ElectionComponent } from './modals/election/election.component';
import { LegislativeSessionComponent } from './modals/legislative-session/legislative-session.component';
import { ExecutiveActionComponent } from './modals/executive-action/executive-action.component';
import { CommunistSessionComponent } from './modals/communist-session/communist-session.component';
import { GameOverComponent } from './modals/game-over/game-over.component';

@Component({
  selector: 'app-board',
  templateUrl: './board.component.html',
  styleUrls: ['./board.component.scss'],
  standalone: true,
  imports: [
    CommonModule,
    JoinGameComponent,
    LobbyComponent,
    BoardContentComponent,
    PlayerRailComponent,
    SceneComponent,
    NightRoundComponent,
    ElectionComponent,
    LegislativeSessionComponent,
    ExecutiveActionComponent,
    CommunistSessionComponent,
    GameOverComponent
  ]
})
export class BoardComponent implements OnInit {
  gameState?: GameStateType;
  currentPrompt?: BoardPrompt;

  constructor(private gameStateManager: GameStateManagerService) {}

  ngOnInit(): void {
    this.gameStateManager.gameState$.subscribe((state: GameStateType) => {
      this.gameState = state;
    });

    this.gameStateManager.prompt$.subscribe((prompt: BoardPrompt | null) => {
      this.currentPrompt = prompt || undefined;
    });
  }

  get isConnecting(): boolean {
    return this.gameState?.type === 'connecting';
  }

  get isLobby(): boolean {
    return this.gameState?.type === 'lobby';
  }

  get isBoard(): boolean {
    return this.gameState?.type === 'board';
  }

  get lobbyGameId(): string {
    return this.gameState?.type === 'lobby' ? this.gameState.game_id : '';
  }

  get electionTracker(): number {
    return this.gameStateManager.getElectionTracker();
  }

  get deckSize(): number {
    return this.gameStateManager.getDeckSize();
  }

  get liberalPolicies(): number {
    return this.gameStateManager.getLiberalPolicies();
  }

  get fascistPolicies(): number {
    return this.gameStateManager.getFascistPolicies();
  }

  get communistPolicies(): number {
    return this.gameStateManager.getCommunistPolicies();
  }

  get showNightRound(): boolean {
    return this.currentPrompt?.type === 'Night';
  }

  get showElection(): boolean {
    return this.currentPrompt?.type === 'Election';
  }

  get showLegislativeSession(): boolean {
    return this.currentPrompt?.type === 'LegislativeSession';
  }

  get showExecutiveAction(): boolean {
    const execTypes = ['PolicyPeak', 'InvestigatePlayer', 'SpecialElection', 'Execution'];
    return this.currentPrompt ? execTypes.includes(this.currentPrompt.type) : false;
  }

  get showCommunistSession(): boolean {
    const commieTypes = ['CommunistSession', 'FiveYearPlan', 'Confession'];
    return this.currentPrompt ? commieTypes.includes(this.currentPrompt.type) : false;
  }

  get showGameOver(): boolean {
    return this.currentPrompt?.type === 'GameOver';
  }
} 