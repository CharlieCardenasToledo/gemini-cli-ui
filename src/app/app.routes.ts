import { Routes } from "@angular/router";
import { ChatComponent } from "./components/chat/chat.component";
import { ConfigComponent } from "./config/config";

export const routes: Routes = [
    { path: '', redirectTo: '/chat', pathMatch: 'full' },
    { path: 'chat', component: ChatComponent },
    { path: 'config', component: ConfigComponent }
];