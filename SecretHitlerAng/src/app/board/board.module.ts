import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

// Tüm bileşenler standalone olduğu için artık bunları import etmemize gerek yok
// import { BoardComponent } from './board.component';
// import { BoardContentComponent } from './board-content/board-content.component';
// ... diğer bileşen importları

@NgModule({
  imports: [
    CommonModule,
    RouterModule,
    // Standalone bileşenleri buraya import ediyoruz
  ],
  // declarations ve exports artık gerekli değil çünkü bileşenler standalone
})
export class BoardModule { } 