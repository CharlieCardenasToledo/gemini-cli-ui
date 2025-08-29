import { Component, EventEmitter, Input, Output } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-chat-input',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './chat-input.component.html',
  styleUrl: './chat-input.component.css'
})
export class ChatInputComponent {
  @Output() userInput = new EventEmitter<string>();
  @Input() isLoading: boolean = false;
  inputValue = '';

  sendMessage() {
    if (this.inputValue.trim()) {
      this.userInput.emit(this.inputValue);
      this.inputValue = '';
    }
  }
}
''