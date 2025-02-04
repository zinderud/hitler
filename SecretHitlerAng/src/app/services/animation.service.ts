import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';

export interface AnimationState {
  type: string;
  params?: Record<string, unknown>;
}

@Injectable({
  providedIn: 'root'
})
export class AnimationService {
  private animationStateSubject = new BehaviorSubject<AnimationState>({ type: '' });
  animationState$: Observable<AnimationState> = this.animationStateSubject.asObservable();

  setAnimationState(type: string, params?: Record<string, unknown>): void {
    this.animationStateSubject.next({ type, params });
  }

  getCurrentState(): AnimationState {
    return this.animationStateSubject.value;
  }

  resetState(): void {
    this.animationStateSubject.next({ type: '' });
  }

  isEnabled(): boolean {
    return true; // veya istediğiniz mantığı buraya ekleyin
  }
} 