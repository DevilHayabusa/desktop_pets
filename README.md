# Desktop Pets Engine

A cross-platform, highly optimized desktop pet application that lives on your screen. Built with a focus on low resource consumption, system awareness, and engaging interactive mechanics.

## 🏗️ Architecture and Tech Stack

This project is divided into a robust, high-performance core and a lightweight frontend:

- **Core Engine (Backend):** Rust + Tauri
  - Handles system telemetry (CPU/RAM monitoring).
  - Manages circadian rhythms based on local OS time.
  - Safely reads/writes persistent states using local JSON storage.
  - Native window manipulation (transparent, always-on-top, click-through).
- **Interface (Frontend):** React (TypeScript) + Vite *(Pending Initialization)*
  - Renders pet sprites and animations.
  - Handles user interactions (drag & drop, petting).

## ✨ Key Features

- **System Awareness:** Pets react to your computer's resource usage. High CPU loads might make them tired or anxious.
- **Dynamic Circadian Rhythms:** Behavior changes based on the real-world time and the pet's specific species (e.g., nocturnal cats vs. diurnal dogs).
- **Persistent State:** Pet growth, energy levels, and rarities are safely stored in local JSON files.
- **Unobtrusive Execution:** Runs silently on startup (auto-start) with borderless, transparent windows.

## 🗂️ Project Structure

/src_tauri
  /src
    /models       # Data structures (Pet, Species, Rarity)
    /services     # Core logic (SystemMonitor, Behavior)
    /utils        # File management and OS utilities
/frontend         # React UI components and assets
/local_storage    # Encrypted/Hashed JSON save files
/docs             # Extended architectural documentation
/tests            # Integration and unit tests

## 🚀 Roadmap

- [x] Initialize project structure and Git repository.
- [x] Define core Rust data models.
- [x] Implement hardware monitoring service.
- [x] Create JSON file manager for persistent state.
- [ ] Connect Rust core to Tauri commands (`main.rs`).
- [ ] Initialize React frontend and establish IPC communication.
- [ ] Implement network trading system via WebSockets.

## 🛡️ License and Security
This project implements data hashing to prevent unauthorized manipulation of local save files (e.g., cheating pet rarity levels).
