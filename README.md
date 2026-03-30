# Desktop Pet Engine

A lightweight, native desktop pet application built purely in Rust. 

This project implements a borderless, transparent window that lives on your desktop. By utilizing a native graphics engine instead of a Webview, it guarantees extreme hardware efficiency, incredibly low RAM consumption, and flawless click-through behavior on the operating system level.

## Architecture

The engine is built on top of [Bevy ECS](https://bevyengine.org/), providing a robust Entity-Component-System architecture. This allows for modular expansion of the pet's logic, states, and physics without performance bottlenecks.

### Core Features

- Native OS transparent windows using `winit`.
- Zero-overhead click-through (interactions bypass the transparent areas completely).
- Asynchronous asset loading and dynamic sprite scaling.
- Native window dragging mechanics.

## Prerequisites

- [Rust Toolchain](https://rustup.rs/) (Cargo, rustc)
- A compatible GPU (Vulkan/DirectX/Metal)

## Getting Started

1. Clone the repository.
2. Ensure you have your initial sprite named `hero.png` inside the `/assets` directory at the root of the project.
3. Run the engine via Cargo:

```bash
cargo run
