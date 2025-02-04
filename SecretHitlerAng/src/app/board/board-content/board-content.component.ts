import { Component, OnInit } from '@angular/core';
import { GameStateManagerService } from '../../services/game-state-manager.service';
import { GameActionService } from '../../services/game-action.service';
import { GameStateType, Policy } from '../../dm/state';
import { CommonModule } from '@angular/common';
import { PolicyTrackerComponent } from '../policy-tracker/policy-tracker.component';
import { PolicyCardComponent } from '../policy-card/policy-card.component';
import { CardRevealComponent } from '../card-reveal/card-reveal.component';
import { LobbyComponent } from '../lobby/lobby.component';
import { NightRoundComponent } from '../modals/night-round/night-round.component';
import { ElectionComponent } from '../modals/election/election.component';
import { LegislativeSessionComponent } from '../modals/legislative-session/legislative-session.component';
import { ExecutiveActionComponent } from '../modals/executive-action/executive-action.component';
import { CommunistSessionComponent } from '../modals/communist-session/communist-session.component';
import { GameOverComponent } from '../modals/game-over/game-over.component';

@Component({
  selector: 'app-board-content',
  templateUrl: './board-content.component.html',
  styleUrls: ['./board-content.component.scss'],
  standalone: true,
  imports: [
    CommonModule,
    PolicyTrackerComponent,
    PolicyCardComponent,
    CardRevealComponent,
    LobbyComponent,
    NightRoundComponent,
    ElectionComponent,
    LegislativeSessionComponent,
    ExecutiveActionComponent,
    CommunistSessionComponent,
    GameOverComponent
  ]
})
export class BoardContentComponent implements OnInit {
  gameState?: GameStateType;
  selectedPolicies: Policy[] = [];
  connectionStatus: 'connecting' | 'connected' | 'error' = 'connecting';

  constructor(
    private gameStateManager: GameStateManagerService,
    private gameAction: GameActionService
  ) {}

  ngOnInit(): void {
    this.gameStateManager.gameState$.subscribe(
      state => {
        this.gameState = state;
        this.connectionStatus = 'connected';
      },
      error => {
        this.connectionStatus = 'error';
        console.error('Bağlantı hatası:', error);
      }
    );
  }

  isConnecting(): boolean {
    return this.connectionStatus === 'connecting';
  }

  isConnectionError(): boolean {
    return this.connectionStatus === 'error';
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

  get electionTracker(): number {
    return this.gameStateManager.getElectionTracker();
  }

  onPolicySelected(policies: Policy[]): void {
    this.selectedPolicies = policies;
    this.gameAction.selectPolicies(policies);
  }

  isGameActive(): boolean {
    return this.gameStateManager.isGameActive();
  }

  getParties() {
    if (this.gameState?.type !== 'board') return ['Liberal', 'Fascist'];
    const hasCommies = this.gameState.communist_cards != null;
    return hasCommies ? ['Liberal', 'Fascist', 'Communist'] : ['Liberal', 'Fascist'];
  }

  getNumPolicies(party: string) {
    if (this.gameState?.type !== 'board') return [];
    let count = 0;
    switch (party) {
      case 'Liberal':
        count = this.gameState.liberal_cards;
        break;
      case 'Fascist':
        count = this.gameState.fascist_cards;
        break;
      case 'Communist':
        count = this.gameState.communist_cards || 0;
        break;
    }
    return Array(count).fill(0).map((_, i) => i);
  }

  getScale() {
    // Implement scale calculation based on container size
    return 1;
  }

  shouldFadeBoard() {
    return this.isPrompt(['Night', 'Election', 'LegislativeSession']);
  }

  isPrompt(types: string | string[]) {
    if (this.gameState?.type !== 'board') return false;
    const prompts = Array.isArray(types) ? types : [types];
    return this.gameState?.prompt && 
           prompts.includes(this.gameState?.prompt.type);
  }

  getCardReveal() {
    if (this.gameState?.type !== 'board' || 
        !this.gameState?.prompt || 
        this.gameState?.prompt.type !== 'CardReveal') {
      return null;
    }
    return this.gameState?.prompt;
  }

  onCardRevealDone() {
    this.gameAction.endCardReveal();
  }

  getChosenPlayer(): number | undefined {
    // ... mantık burada ...
    return undefined;
  }

  onExecutiveActionDone(): void {
    // ... mantık burada ...
  }

  getCardPosition(party: string, index: number): { x: number, y: number } {
    // Kart pozisyonunu hesaplama mantığı
    return { x: 0, y: 0 };
  }

  getElection() {
    if (!this.isPrompt('Election')) return null;
    return {
      president: this.gameState?.type === 'board' ? this.gameState.prompt?.president : undefined,
      chancellor: this.gameState?.type === 'board' ? this.gameState.prompt?.chancellor : undefined,
      outcome: this.gameState?.type === 'board' ? this.gameState.prompt?.outcome : undefined
    };
  }

  getLegislativeSession() {
    if (!this.isPrompt('LegislativeSession')) return null;
    return {
      president: this.gameState?.type === 'board' ? this.gameState.prompt?.president : undefined,
      chancellor: this.gameState?.type === 'board' ? this.gameState.prompt?.chancellor : undefined,
      phase: this.gameState?.type === 'board' ? this.gameState.prompt?.phase : undefined
    };
  }

  onElectionDone(): void {
    // Seçim tamamlandı mantığı
  }

  onLegislativeDone(): void {
    // Yasama oturumu tamamlandı mantığı
  }

  getNumPlayers(): number {
    if (this.gameState?.type !== 'board') return 0;
    return this.gameState.players.length;
  }

  getLobbyGameId(): string {
    if (this.gameState?.type !== 'lobby') return '';
    return this.gameState.game_id;
  }

  getCommunistSessionType(): string {
    if (this.gameState?.type !== 'board' || !this.gameState.prompt) return '';
    return this.gameState.prompt.type;
  }

  getCommunistSessionPhase(): string {
    if (this.gameState?.type !== 'board' || !this.gameState.prompt) return '';
    return this.gameState.prompt.phase || '';
  }

  getGameOverResult(): 'victory' | 'defeat' | undefined {
    if (this.gameState?.type !== 'board' || !this.gameState.prompt) return undefined;
    return this.gameState.prompt.result as 'victory' | 'defeat';
  }

  // Implement other helper methods for elections, legislative sessions, etc.
} 