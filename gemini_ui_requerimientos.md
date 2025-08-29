# Documento de Requerimientos para una UI de Escritorio de **Gemini CLI**

## 1. Introducción
Este documento detalla los requerimientos funcionales y no funcionales para el desarrollo de una aplicación de escritorio similar a **Claude Code (Gooey)**, pero diseñada para interactuar con **Gemini CLI**. El objetivo es ofrecer una experiencia visual, moderna y productiva que complemente la potencia del CLI con una interfaz amigable.

---

## 2. Objetivos
- Crear una aplicación de escritorio multiplataforma (Windows, macOS, Linux).
- Proporcionar una interfaz visual para interactuar con **Gemini CLI**.
- Gestionar múltiples sesiones de conversación.
- Integrar exploración de proyectos, gestión de agentes personalizados y paneles de control.
- Facilitar la productividad con herramientas visuales adicionales (historial, favoritos, métricas, etc.).

---

## 3. Alcance del sistema
La aplicación permitirá:
- Ingresar prompts a Gemini y recibir respuestas en una vista tipo chat.
- Administrar proyectos locales y sus archivos relacionados.
- Definir agentes personalizados con roles, estilos o funciones.
- Guardar, reanudar y exportar sesiones.
- Visualizar métricas de uso (tokens, costo, tiempo de ejecución).
- Configurar parámetros de Gemini CLI (modelo, flags, rutas, etc.).

---

## 4. Requerimientos Funcionales

### 4.1. Chat y Sesiones
- RF-01: El sistema debe permitir enviar prompts a Gemini CLI.
- RF-02: El sistema debe mostrar la salida de Gemini en formato de chat.
- RF-03: El sistema debe guardar el historial de conversaciones en local (JSON o SQLite).
- RF-04: El sistema debe permitir exportar sesiones (Markdown, TXT, JSON).
- RF-05: El sistema debe soportar múltiples sesiones simultáneas.

### 4.2. Gestión de Proyectos
- RF-06: El sistema debe permitir abrir carpetas locales como “proyectos”.
- RF-07: El sistema debe mostrar la estructura de archivos y permitir búsqueda.
- RF-08: El sistema debe permitir editar archivos directamente en la interfaz (editor embebido).
- RF-09: El sistema debe permitir enviar archivos/código a Gemini para análisis.

### 4.3. Agentes Personalizados
- RF-10: El sistema debe permitir crear y gestionar agentes con prompts iniciales (ejemplo: *traductor, analista de código, generador de pruebas*).
- RF-11: El sistema debe permitir asignar un agente a una sesión.

### 4.4. Métricas y Analíticas
- RF-12: El sistema debe mostrar número de tokens utilizados por sesión.
- RF-13: El sistema debe mostrar costos estimados.
- RF-14: El sistema debe registrar estadísticas de uso histórico.

### 4.5. Configuración
- RF-15: El sistema debe permitir configurar la ruta del binario de Gemini CLI.
- RF-16: El sistema debe permitir configurar flags de ejecución (ej. `--model`, `--temperature`).
- RF-17: El sistema debe permitir configurar temas visuales (oscuro/claro).
- RF-18: El sistema debe soportar configuración de proxy.

---

## 5. Requerimientos No Funcionales

- RNF-01: La aplicación debe desarrollarse con **Tauri 2** (Rust + frontend web: Angular/React/Vue).
- RNF-02: El frontend debe ser responsive y con diseño moderno.
- RNF-03: La aplicación debe ser multiplataforma (Windows, Linux, macOS).
- RNF-04: Los tiempos de respuesta deben ser menores a 2 segundos (excluyendo tiempo de Gemini CLI).
- RNF-05: Los datos deben almacenarse localmente en formato seguro (SQLite encriptado o similar).
- RNF-06: La aplicación debe permitir actualizaciones automáticas.

---

## 6. Casos de Uso Principales

1. **Enviar Prompt**: El usuario ingresa un texto, la UI lo envía a Gemini CLI, recibe la respuesta y la muestra en el chat.
2. **Reanudar Sesión**: El usuario abre una sesión guardada y continúa la conversación.
3. **Crear Agente**: El usuario define un nuevo agente con instrucciones predefinidas.
4. **Explorar Proyecto**: El usuario abre una carpeta local y navega por sus archivos.
5. **Analizar Archivo**: El usuario selecciona un archivo y lo envía a Gemini para análisis.
6. **Ver Métricas**: El usuario consulta las estadísticas de tokens y costos de una sesión.

---

## 7. Tecnologías Propuestas
- **Frontend**: Angular + TailwindCSS (para UI moderna y modular).
- **Backend**: Tauri (Rust) con comandos personalizados para ejecutar Gemini CLI.
- **Base de datos local**: SQLite + Prisma/Drift (gestión de sesiones e historial).
- **Editor embebido**: Monaco Editor (VS Code editor).

---

## 8. Roadmap de Desarrollo

### Fase 1 – MVP
- Chat básico con Gemini CLI.
- Guardar y cargar sesiones.
- Configuración mínima (binario y modelo).

### Fase 2 – Funcionalidades intermedias
- Explorador de proyectos.
- Editor embebido.
- Agentes personalizados.
- Temas oscuro/claro.

### Fase 3 – Avanzado
- Dashboard de métricas.
- Exportación de sesiones.
- Integración con proxy y ajustes avanzados.
- Actualizaciones automáticas.

---

## 9. Entregables
- Código fuente del proyecto (repositorio GitHub).
- Documentación técnica (README, guía de instalación, manual de usuario).
- Binarios instalables para Windows, macOS y Linux.

---

## 10. Conclusión
Este sistema busca brindar a los usuarios de **Gemini CLI** una experiencia visual, intuitiva y productiva, similar a la de Claude Code/Gooey, pero adaptada al ecosistema de Gemini. Con una base modular (Tauri + Angular), permitirá extender fácilmente la funcionalidad en el futuro.

