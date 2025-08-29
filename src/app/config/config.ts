import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { GeminiService, AppConfig } from '../services/gemini.service';

@Component({
  selector: 'app-config',
  standalone: true,
  imports: [FormsModule, CommonModule],
  templateUrl: './config.html',
  styleUrl: './config.css'
})
export class ConfigComponent implements OnInit {
  config: AppConfig = {
    gemini_cli_path: '',
    gemini_cli_args: []
  };
  newArg: string = '';

  constructor(private geminiService: GeminiService) { }

  async ngOnInit(): Promise<void> {
    try {
      this.config = await this.geminiService.getAppConfig();
      // Ensure gemini_cli_path is a string for ngModel
      if (this.config.gemini_cli_path === null || this.config.gemini_cli_path === undefined) {
        this.config.gemini_cli_path = '';
      }
    } catch (error) {
      console.error('Error loading config:', error);
    }
  }

  addArg(): void {
    if (this.newArg.trim() && !this.config.gemini_cli_args.includes(this.newArg.trim())) {
      this.config.gemini_cli_args.push(this.newArg.trim());
      this.newArg = '';
    }
  }

  removeArg(index: number): void {
    this.config.gemini_cli_args.splice(index, 1);
  }

  async saveConfig(): Promise<void> {
    try {
      // Convert empty string back to undefined/null for Rust Option
      const configToSave: AppConfig = {
        gemini_cli_path: this.config.gemini_cli_path === '' ? undefined : this.config.gemini_cli_path,
        gemini_cli_args: this.config.gemini_cli_args
      };
      await this.geminiService.setAppConfig(configToSave);
      alert('Configuration saved successfully!');
    } catch (error) {
      console.error('Error saving config:', error);
      alert('Failed to save configuration.');
    }
  }
}