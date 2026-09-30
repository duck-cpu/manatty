# Manatty

**Manatty** is a terminal-first Magic: The Gathering duel simulator built in Rust.

The project is based on the rules engine from [`phase-rs/phase`](https://github.com/phase-rs/phase), but is being developed as a divergent fork focused on deterministic local play, terminal interaction, and AI opponents that make decisions without owning game rules.

## Goals

Manatty is intended to provide a fast, inspectable way to play and study Magic from the terminal.

The core design is simple:

- **Rust owns the game.** Rules, legality, state transitions, card behavior, and resolution are deterministic engine responsibilities.
- **AI only chooses.** An AI opponent receives a set of legal engine-issued choices and selects one; it does not invent actions or modify game state directly.
- **Terminal first.** The primary interface is plain textual input and output rather than a graphical client.
- **Local friendly.** The project is designed to work with local models through OpenAI-compatible endpoints such as `llama.cpp`.
- **Useful for learning.** Game output can expose card names, costs, rules text, and relevant state so repeated play also reinforces card knowledge.

## Current Status

Manatty is currently in the engine-extraction and application-bootstrap stage.

The original web/Tauri client and obsolete draft/frontend integration surfaces have been removed. The retained engine has been decoupled from those components, and its full test suite currently passes in the stripped-down tree.

Current retained core crates include:

```text
crates/
├── engine/       # deterministic MTG rules engine
├── phase-ai/     # engine-aware AI decision layer
└── phase-llm/    # LLM decision contracts and request rendering
