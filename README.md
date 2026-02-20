[![Release](https://github.com/AlbertSanIza/octo-spawn/actions/workflows/release.yml/badge.svg)](https://github.com/AlbertSanIza/octo-spawn/actions/workflows/release.yml)

# 🐙 Octo-Spawn

A simple macOS menubar app to spawn multiple GitHub Desktop windows.

## Install

Download the latest `.dmg` from [Releases](https://github.com/AlbertSanIza/octo-spawn/releases).

Since the app is not notarized, macOS may block it on first launch. To allow it:

1. Go to **System Settings > Privacy & Security** and click **Open Anyway**

Or if that doesn't work, run:

```bash
xattr -cr /Applications/octo-spawn.app
```

## Development

```bash
bun install
bun run dev
```

## Build

```bash
bun run build
```

Output is in `src-tauri/target/release/bundle/`.

## How it works

GitHub Desktop enforces a single window via singleton lock files. Octo-Spawn copies the app's user data to `/tmp/github-desktop-clone`, removes the lock files (`SingletonLock`, `SingletonCookie`, `SingletonSocket`), and launches a new instance pointing at the cloned data directory.
