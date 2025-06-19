# GrayMatter: AI-Assisted Mind Map

An offline, privacy-first mind map and idea organizer with integrated AI assistance. Built in Rust using egui for a fast, modern, and cross-platform native experience.

100% Vibe Coded

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/D1D61442TG)

## Features

### Core (Free/Base Version)
- Create and edit mind maps (nodes and edges)
- Local save/load (planned: SQLite or JSON-based storage)
- Simple visual UI with drag-and-drop node positioning
- Color coding, labels, and basic organization (coming soon)

### AI-Powered Features (Planned)
- Auto-grouping: Suggests logical clusters or subtopics
- Connection suggestions: AI recommends connections based on semantic similarity
- Auto-naming: Proposes better names or summaries for nodes/groups
- Idea expansion: User types a seed idea, and AI suggests related nodes
- Search & summarize: Find related ideas and present a high-level overview of a map

### Privacy & Offline Design
- No account, no sync. Local-only data
- AI runs on-device: Use lightweight models (planned: distilled BERT or GPT2)
- Export/import as JSON or PDF for backups or sharing (planned)
- Optional encryption for maps (planned)

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (with Cargo)

### Running the App
```sh
cargo run
```

### Project Structure
```
src/
  main.rs    # Entry point, launches the app
  app.rs     # Main app logic and state
  node.rs    # Node data structure and logic
```

## Roadmap
- [x] Modular codebase with comments
- [x] Node creation and drag-and-drop
- [ ] Node editing (labels, colors)
- [ ] Edge (connection) creation
- [ ] Local save/load
- [ ] AI-powered features
- [ ] Export/import

## License
This project is licensed under the **GrayMatter AI-Assisted Mind Map Custom License**. See [LICENSE.txt](LICENSE.txt) for details.

---

*This README will be updated as new features are added.* 