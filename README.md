[![Release](https://github.com/AlbertSanIza/octo-spawn/actions/workflows/release.yml/badge.svg)](https://github.com/AlbertSanIza/octo-spawn/actions/workflows/release.yml)

# 🐙 Octo-Spawn

A simple macOS menubar app to spawn multiple GitHub Desktop windows.

## Install

Download the latest `.dmg` from [Releases](https://github.com/AlbertSanIza/octo-spawn/releases).

Since the app is unsigned, macOS may show a "damaged" warning. To fix this, run:

```bash
xattr -cr /Applications/octo-spawn.app
```

## Development

```bash
npm install
npm run dev
```

## Build

```bash
npm run build
```

Output is in `src-tauri/target/release/bundle/`.

## How it works

GitHub Desktop enforces a single window via singleton lock files. Octo-Spawn copies the app's user data to `/tmp/github-desktop-clone`, removes the lock files (`SingletonLock`, `SingletonCookie`, `SingletonSocket`), and launches a new instance pointing at the cloned data directory.
