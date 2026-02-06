# 🐙 Octo-Spawn

macOS menubar app to spawn multiple GitHub Desktop windows.

## What is this?

GitHub Desktop doesn't natively support opening multiple windows. Octo-Spawn solves this by living in your menubar and letting you spawn new instances with a click!

## Features

- 🎯 Lives in your macOS menubar
- 🚀 Spawn multiple GitHub Desktop windows
- ⚡️ Execute terminal commands from buttons
- 🎨 Clean, native-looking UI

## Tech Stack

- **Tauri 2.0** - Rust + Web for native apps
- **Vanilla JS** - Simple, no frameworks needed
- **Bun** - Fast package manager

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh)
- macOS (for menubar functionality)

### Setup

```bash
# Install dependencies
bun install

# Run in dev mode
bun run tauri dev

# Build for production
bun run tauri build
```

## TODO

- [ ] Implement actual GitHub Desktop spawning logic
- [ ] Add icon for tray
- [ ] Add preferences/settings
- [ ] Support multiple repo quick-launch buttons
- [ ] Add keyboard shortcuts

## License

MIT
