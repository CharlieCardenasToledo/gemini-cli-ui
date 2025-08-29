import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MarkdownModule } from 'ngx-markdown'; // Import MarkdownModule

@Component({
  selector: 'app-message',
  standalone: true,
  imports: [CommonModule, MarkdownModule], // Add MarkdownModule here
  templateUrl: './message.component.html',
  styleUrl: './message.component.css'
})
export class MessageComponent {
  @Input() message!: { text: string, from: 'user' | 'gemini', status?: 'sending' | 'loading' | 'sent' | 'error', errorMessage?: string };
}