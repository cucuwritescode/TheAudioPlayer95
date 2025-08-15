# The Audio Player95

Retro-inspired audio player that emulates the beloved classic Windows 95 UI. Made to enjoy your favourite tunes while reliving the charm of the past.

<img width="816" alt="Screenshot 2024-07-16 at 02 22 12" src="https://github.com/user-attachments/assets/5390da7b-9f7b-4205-afed-cfed69ae098f">

## Architecture

```
┌─────────────────────────────────────────────┐
│            React95 UI Layer                 │
│  (TreeView, Monitor, Controls, Discover Tab)│
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────┴───────────────────────────┐
│          Tauri Command Layer                │
│  (play, pause, analyse, get_recommendations)│
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────┴───────────────────────────┐
│         Rust Audio Engine                   │
│  ┌──────────────────────────────────┐      │
│  │   Audio Pipeline                  │      │
│  │  (Symphonia → Resampler → CPAL)  │      │
│  └──────────────┬───────────────────┘      │
│                 │                           │
│  ┌──────────────┴───────────────────┐      │
│  │   Audio Analysis Module          │      │
│  │  - BPM Detection                 │      │
│  │  - Key Detection                 │      │
│  │  - Spectral Features             │      │
│  │  - Genre Classification          │      │
│  └──────────────┬───────────────────┘      │
│                 │                           │
│  ┌──────────────┴───────────────────┐      │
│  │   Recommendation Engine          │      │
│  │  - Local similarity matching     │      │
│  │  - Last.fm/MusicBrainz API      │      │
│  │  - User preference learning      │      │
│  └──────────────────────────────────┘      │
└─────────────────────────────────────────────┘
                  │
┌─────────────────┴───────────────────────────┐
│          SQLite Database                    │
│  - Track metadata & features                │
│  - Listening history                        │
│  - Recommendation cache                     │
└─────────────────────────────────────────────┘
```

## Features

- **Windows 95 Aesthetic**: Authentic retro UI using React95 components
- **Rust-Powered Audio**: High-performance audio playback with Symphonia and CPAL
- **Smart Recommendations**: AI-powered music discovery based on audio analysis
- **Real-time Analysis**: BPM detection, key detection, and genre classification
- **Seamless Playback**: Gapless playback with advanced buffering
- **Music Discovery**: Personalised recommendations from local analysis and external APIs
