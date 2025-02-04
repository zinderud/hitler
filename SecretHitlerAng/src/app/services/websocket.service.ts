import { Injectable } from '@angular/core';
import { Subject } from 'rxjs';
import { environment } from '../../environments/environment';
import { GameAction } from '../dm/state';

interface WebSocketMessage {
  type: string;
  state?: any;
  error?: string;
}

@Injectable({
  providedIn: 'root'
})
export class WebSocketService {
  private ws: WebSocket | null = null;
  private messagesSubject = new Subject<WebSocketMessage>();
  messages$ = this.messagesSubject.asObservable();

  constructor() {
    this.connect();
  }

  private connect() {
    this.ws = new WebSocket(environment.wsUrl);

    this.ws.onopen = () => {
      console.log('WebSocket bağlantısı kuruldu');
    };

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.messagesSubject.next(message);
    };

    this.ws.onclose = () => {
      console.log('WebSocket bağlantısı kapandı, yeniden bağlanılıyor...');
      setTimeout(() => this.connect(), 1000);
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket hatası:', error);
    };
  }

  send(message: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    } else {
      console.error('WebSocket bağlantısı kapalı');
    }
  }

  boardAction(action: GameAction) {
    this.send({
      type: 'BoardAction',
      action
    });
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
    }
  }
} 