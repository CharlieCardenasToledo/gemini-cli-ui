import { provideMarkdown } from "ngx-markdown"; // Import provideMarkdown
import { HIGHLIGHT_OPTIONS } from "ngx-highlightjs"; // Import HIGHLIGHT_OPTIONS

import {
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
  provideZoneChangeDetection,
} from "@angular/core";
import { provideRouter } from "@angular/router";

import { routes } from "./app.routes";

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    provideMarkdown(), // Add provideMarkdown configuration
    { // Provide HIGHLIGHT_OPTIONS for highlight.js
      provide: HIGHLIGHT_OPTIONS,
      useValue: {
        // You can specify a theme here, e.g., 'github'
        // theme: 'github',
        // You can also specify the path to the highlight.js library
        // coreLibrary: () => import('highlight.js/lib/core'),
        // languages: {
        //   typescript: () => import('highlight.js/lib/languages/typescript'),
        //   css: () => import('highlight.js/lib/languages/css'),
        //   xml: () => import('highlight.js/lib/languages/xml')
        // }
      },
    },
  ],
};
