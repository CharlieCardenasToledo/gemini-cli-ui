import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

export interface Session {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface Message {
  id: number;
  session_id: number;
  text: string;
  from_user: string;
  timestamp: string;
}

export interface AppConfig {
  gemini_cli_path?: string;
  gemini_cli_args: string[];
}

@Injectable({
  providedIn: 'root'
})
export class GeminiService {

  constructor() { }

  runCommand(prompt: string): Promise<string> {
    return invoke<string>('run_gemini_command', { prompt })
      .catch((error: any) => {
        if (error === 'GEMINI_QUOTA_EXCEEDED') {
          throw { code: 'QUOTA_EXCEEDED', message: 'Quota exceeded for Gemini API. Would you like to switch to gemini-2.5-flash and resend the request?' };
        }
        throw error; // Re-throw other errors
      });
  }

  async createSession(name: string): Promise<number> {
    return invoke('create_session', { name });
  }

  async getSessions(): Promise<Session[]> {
    return invoke('get_sessions');
  }

  async getMessages(sessionId: number): Promise<Message[]> {
    return invoke('get_messages', { sessionId });
  }

  async saveMessage(sessionId: number, text: string, fromUser: string): Promise<void> {
    return invoke('save_message', { sessionId, text, fromUser });
  }

  async getAppConfig(): Promise<AppConfig> {
    return invoke('get_config');
  }

  async setAppConfig(config: AppConfig): Promise<void> {
    return invoke('set_config', { config });
  }

  async exportSession(sessionId: number, format: string): Promise<string> {
    return invoke('export_session', { sessionId, format });
  }
}