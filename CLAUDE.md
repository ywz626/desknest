# DeskNest

DeskNest is a lightweight AI-powered desktop organizer for Windows.

Tech Stack:

* Tauri v2
* Vue 3
* TypeScript
* Rust (system bridge only)

Core Principles:

* Lightweight first
* Modern minimal UI
* Windows-native experience
* Virtual organization before real file movement
* Small incremental changes only

Current MVP Goals:

* Scan desktop icons/files
* Display icons in grid layout
* Create virtual folder groups
* Support drag & drop grouping
* Open real files from virtual UI
* Do NOT move real files yet

Architecture Rules:

* UI logic in Vue/TypeScript
* Rust only for system APIs
* Avoid heavy dependencies
* Avoid overengineering
* Keep components small and maintainable

UI Style:

* Windows 11
* Raycast
* Arc Browser
* Mobile folder style
* Rounded corners
* Semi-transparent surfaces
* Minimal and clean

Code Requirements:

* Use TypeScript strict mode
* Avoid any
* Add comments for important logic
* Add logs for important operations
* Use Composition API and script setup

Project Structure:

src/
components/
views/
services/
stores/
types/
utils/

src-tauri/src/
commands/
services/
models/

When modifying code:

1. Explain the plan first
2. Make small focused changes
3. Explain what changed
4. Suggest next steps
