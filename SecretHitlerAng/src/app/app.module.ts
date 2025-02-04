import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { AppRoutingModule } from './app-routing.module';
import { WebSocketService } from './services/websocket.service';
import { GameStateManagerService } from './services/game-state-manager.service';
import { GameActionService } from './services/game-action.service';
import { SoundService } from './services/sound.service';
import { AnimationService } from './services/animation.service';

@NgModule({
  imports: [
    BrowserModule,
    RouterModule,
    FormsModule,
    AppRoutingModule
  ],
  providers: [
    WebSocketService,
    GameStateManagerService,
    GameActionService,
    SoundService,
    AnimationService
  ],
})
export class AppModule { } 