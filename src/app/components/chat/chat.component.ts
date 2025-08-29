import { Component, OnInit } from '@angular/core';
import { GeminiService, Session, Message } from '../../services/gemini.service';
import { MessageComponent } from '../message/message.component';
import { ChatInputComponent } from '../chat-input/chat-input.component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-chat',
  standalone: true,
  imports: [CommonModule, MessageComponent, ChatInputComponent, FormsModule],
  templateUrl: './chat.component.html',
  styleUrl: './chat.component.css'
})
export class ChatComponent implements OnInit {
  messages: { text: string, from: 'user' | 'gemini', status?: 'sending' | 'loading' | 'sent' | 'error', errorMessage?: string }[] = [];
  sessions: Session[] = [];
  currentSessionId: number | null = null;
  newSessionName: string = '';
  exportFormat: string = 'markdown'; // Default export format
  isSendingMessage: boolean = false;

  constructor(private geminiService: GeminiService) { }

  ngOnInit(): void {
    this.loadSessions();
  }

  async loadSessions() {
    try {
      this.sessions = await this.geminiService.getSessions();
      if (this.sessions.length > 0) {
        // Load the first session by default if available
        this.loadSession(this.sessions[0].id.toString()); // Corrected: pass as string
      }
    } catch (error) {
      console.error('Error loading sessions:', error);
    }
  }

  async createNewSession() {
    if (this.newSessionName.trim()) {
      try {
        const sessionId = await this.geminiService.createSession(this.newSessionName.trim());
        this.currentSessionId = sessionId;
        this.messages = [];
        this.loadSessions();
        this.newSessionName = '';
      } catch (error) {
        console.error('Error creating session:', error);
      }
    }
  }

  async loadSession(eventOrSessionId: Event | string) {
    try {
      let id: number;
      if (typeof eventOrSessionId === 'string') {
        id = parseInt(eventOrSessionId, 10);
      } else {
        const target = eventOrSessionId.target as HTMLSelectElement;
        id = parseInt(target.value, 10);
      }

      this.currentSessionId = id;
      const loadedMessages = await this.geminiService.getMessages(id);
      this.messages = loadedMessages.map(msg => ({
        text: msg.text,
        from: msg.from_user === 'user' ? 'user' : 'gemini'
      }));
    } catch (error) {
      console.error('Error loading session:', error);
    }
  }

  async exportCurrentSession() {
    if (this.currentSessionId === null) {
      alert('Please select a session to export.');
      return;
    }

    try {
      const filePath = await this.geminiService.exportSession(this.currentSessionId, this.exportFormat);
      alert(`Session exported successfully to: ${filePath}`);
    } catch (error) {
      console.error('Error exporting session:', error);
      alert(`Failed to export session: ${error}`);
    }
  }

  get currentSessionName(): string {
    if (this.currentSessionId === null) {
      return 'None';
    }
    const session = this.sessions.find(s => s.id === this.currentSessionId);
    return session ? session.name : 'Unknown';
  }

  async handleUserInput(prompt: string) {
    if (this.currentSessionId === null) {
      console.warn('No active session. Please create or load a session.');
      return;
    }

    this.isSendingMessage = true; // Set loading state

    const userMessage: { text: string, from: 'user' | 'gemini', status?: 'sending' | 'loading' | 'sent' | 'error', errorMessage?: string } = { text: prompt, from: 'user', status: 'sending' };
    this.messages.push(userMessage);

    // Add a placeholder for Gemini's response
    const geminiMessagePlaceholder: { text: string, from: 'user' | 'gemini', status?: 'sending' | 'loading' | 'sent' | 'error', errorMessage?: string } = { text: '', from: 'gemini', status: 'loading' };
    this.messages.push(geminiMessagePlaceholder);

    try {
      await this.geminiService.saveMessage(this.currentSessionId, prompt, 'user');
      userMessage.status = 'sent';
    } catch (error: any) {
      console.error('Error saving user message:', error);
      userMessage.status = 'error';
      userMessage.errorMessage = `Failed to save user message: ${error.message || error}`;
    }

    try {
      const response = await this.geminiService.runCommand(prompt);
      console.log('Gemini API Response:', response); // Log the response
      geminiMessagePlaceholder.text = response;
      geminiMessagePlaceholder.status = 'sent';
      try {
        await this.geminiService.saveMessage(this.currentSessionId, response, 'gemini');
      } catch (error: any) {
        console.error('Error saving Gemini message:', error);
        geminiMessagePlaceholder.status = 'error';
        geminiMessagePlaceholder.errorMessage = `Failed to save Gemini response: ${error.message || error}`;
      }
    } catch (error: any) {
      console.error('Error running Gemini command:', error);
      geminiMessagePlaceholder.text = 'An error occurred while processing your request.';
      geminiMessagePlaceholder.status = 'error';
      geminiMessagePlaceholder.errorMessage = `Error from Gemini: ${error.message || error}`;
      try {
        await this.geminiService.saveMessage(this.currentSessionId, geminiMessagePlaceholder.text, 'gemini');
      } catch (saveError: any) {
        console.error('Error saving error message:', saveError);
        geminiMessagePlaceholder.errorMessage += `\nFailed to save error message: ${saveError.message || saveError}`;
      }
    } finally {
      this.isSendingMessage = false; // Reset loading state
    }
  }
}