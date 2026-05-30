<div align="center">

# Tic-Tac-Toe 3X

### A Gobblet-style tic-tac-toe game with stackable pieces, built in Rust + Bevy

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/Bevy-232326?style=for-the-badge&logoColor=white)](https://bevyengine.org/)
[![Repo](https://img.shields.io/badge/GitHub-jackinf%2Ftic--tac--toe--3x-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/jackinf/tic-tac-toe-3x)

</div>

## Overview

Tic-Tac-Toe 3X is a 2D desktop game written in Rust on top of the [Bevy](https://bevyengine.org/) game engine. It plays out on a 3x3 board, but with a twist inspired by Gobblet: pieces come in three sizes (Small, Medium, Large), and a larger piece can be placed on top of a smaller one, covering it. The first player to line up three of their own pieces wins.

## Features

- 3x3 board rendered with Bevy's 2D sprite pipeline and a custom font.
- Three piece sizes (Small, Medium, Large) — a bigger piece can "gobble" a smaller one on the same tile.
- Move validation that only allows placing a piece on an empty tile or over a strictly smaller piece.
- Turn-based two-player flow tracked via a Bevy resource (`PlayerTurn`).
- Event-driven architecture using Bevy events (`TileChosenEvent`, `GameOverEvent`) and dedicated handlers.
- Application state management (`AppState`) gating gameplay systems to the `Playing` state.
- Win detection across the board after each move.

## Tech Stack

| Area | Technology |
| --- | --- |
| Language | Rust (edition 2021) |
| Engine | Bevy 0.13.2 |
| Rendering | Bevy 2D sprites, custom TTF font |

## Getting Started

### Prerequisites

- A recent [Rust toolchain](https://www.rust-lang.org/tools/install) (`cargo` + `rustc`).
- Bevy's platform dependencies for your OS — see the [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/).

### Installation

```bash
git clone https://github.com/jackinf/tic-tac-toe-3x.git
cd tic-tac-toe-3x
```

### Running

```bash
cargo run
```

To produce an optimized build:

```bash
cargo build --release
```

The `assets/` directory (sprites and font) is loaded at runtime and must remain alongside the executable.

## Project Structure

```
tic-tac-toe-3x/
├── assets/              # Sprites and fonts loaded by Bevy
│   ├── sprites/         # Board tiles and piece textures
│   └── fonts/           # TTF font
└── src/
    ├── main.rs          # App setup: plugins, systems, events, resources
    ├── constants.rs     # Board/tile/piece sizing constants
    ├── actions/         # Pure logic: move validation, win check, positioning
    ├── components/       # ECS components (e.g. Piece)
    ├── events/           # TileChosenEvent, GameOverEvent
    ├── event_handlers/   # Handlers reacting to game events
    ├── resources/        # Shared state such as PlayerTurn
    ├── states/           # AppState (e.g. Playing)
    ├── systems/          # Board spawning and move handling
    └── types/            # PieceSize, Player, TileCoord
```
