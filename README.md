# iqra-tui

A distraction-free Qur'an reading experience for the terminal.

## Features

- Arabic text, English translation, and Romanized Arabic
- Paginated & Scroll reading modes
- Selective Surah downloads for offline usage
- Bookmarks with quick navigation
- Salah (prayer) timings in status bar
- Multiple color themes (Dark, Light, Terminal)
- Fully configurable keybindings
- Keyboard-first navigation

## Installation

Requires Rust nightly (edition 2024):

```sh
rustup default nightly
cargo install --path .
```

## Usage

```sh
iqra
```

### Keybindings

| Key | Action |
|---|---|
| `s` | Open Surah list |
| `←` / `→` | Previous / Next ayah |
| `↑` / `↓` | Scroll up / down |
| `Home` / `End` | First / Last ayah |
| `PgUp` / `PgDn` | Jump 10 ayahs |
| `d` | Download current Surah |
| `x` | Delete downloaded Surah |
| `b` | Toggle bookmark |
| `B` | Open bookmark list |
| `m` | Toggle scroll mode |
| `a` | Toggle Arabic text |
| `r` | Toggle Romanized text |
| `e` | Toggle English text |
| `?` | Help overlay |
| `q` | Quit |

### Surah List

Press `s` to open the Surah selector. Type to search by name or number. Status indicators:

- 🌐 Not downloaded (requires internet)
- 💾 Downloaded (available offline)
- ⏳ Downloading
- ❌ Failed

## Configuration

Copy the example config:

```sh
mkdir -p ~/.config/iqra
cp config.example.toml ~/.config/iqra/config.toml
```

Edit `~/.config/iqra/config.toml` to customize:
- Theme (`Dark`, `Light`, `Terminal`)
- Default reading mode
- Visible text layers (Arabic, Romanized, English)
- Keybindings
- Prayer time location
- TTS reciter

## Data Storage

- Database: `~/.local/share/iqra/iqra.db`
- Config: `~/.config/iqra/config.toml`

## Architecture

```
src/
├── api/        # HTTP clients (Al Quran Cloud, AlAdhan)
├── config/     # TOML config & keybindings
├── core/       # Data models & service layer
├── salah/      # Prayer time service
├── storage/    # SQLite database layer
├── theme/      # Color theme system
├── tts/        # Audio playback (stub)
├── ui/         # Ratatui rendering & event handling
│   └── widgets/
├── lib.rs
└── main.rs
```

## APIs Used

- [Al Quran Cloud](https://alquran.cloud) — Qur'an text, translations, transliteration
- [AlAdhan](https://aladhan.com) — Prayer times

## License

MIT
