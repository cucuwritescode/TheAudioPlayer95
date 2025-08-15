use tauri::{Builder, generate_context, generate_handler};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

mod decoder;
mod output;
mod player;
mod resampler;

use player::{AudioPlayer, PlayerState};

// Define the structure for the audio track
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AudioTrack {
    id: String,
    name: String,
    path: String,
}

// State for managing audio tracks and player
struct AppState {
    tracks: Arc<Mutex<HashMap<String, AudioTrack>>>,
    player: Arc<tokio::sync::Mutex<AudioPlayer>>,
}

#[tauri::command]
async fn play_track(state: tauri::State<'_, AppState>, file_path: String) -> Result<(), String> {
    println!("Loading track: {}", file_path);
    
    let path = PathBuf::from(file_path);
    
    // Load the track
    let mut player = state.player.lock().await;
    player.load_track(path).await.map_err(|e| e.to_string())?;
    
    // Start playing
    player.play().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn play_audio(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let player = state.player.lock().await;
    player.play().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn pause_audio(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let player = state.player.lock().await;
    player.pause().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn stop_audio(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut player = state.player.lock().await;
    player.stop().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn set_volume(state: tauri::State<'_, AppState>, volume: u8) -> Result<(), String> {
    let player = state.player.lock().await;
    player.set_volume(volume).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_player_state(state: tauri::State<'_, AppState>) -> Result<PlayerStateResponse, String> {
    let player = state.player.lock().await;
    let player_state = player.get_state();
    
    Ok(PlayerStateResponse {
        is_playing: player_state.is_playing,
        current_track: player_state.current_track.and_then(|p| p.to_str().map(String::from)),
        position: player_state.position,
        duration: player_state.duration,
        volume: player_state.volume,
        title: player_state.metadata.as_ref().map(|m| m.title.clone()),
        artist: player_state.metadata.as_ref().map(|m| m.artist.clone()),
        album: player_state.metadata.as_ref().map(|m| m.album.clone()),
    })
}

#[derive(Serialize)]
struct PlayerStateResponse {
    is_playing: bool,
    current_track: Option<String>,
    position: f64,
    duration: f64,
    volume: u8,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
}

#[tauri::command]
fn add_audio_track(state: tauri::State<AppState>, id: String, name: String, path: String) {
    let mut tracks = state.tracks.lock().unwrap();
    let track = AudioTrack { id, name, path };
    tracks.insert(track.id.clone(), track);
    println!("Added track to library");
}

#[tauri::command]
fn remove_audio_track(state: tauri::State<AppState>, id: String) {
    let mut tracks = state.tracks.lock().unwrap();
    if tracks.remove(&id).is_some() {
        println!("Removed track with id: {}", id);
    }
}

#[tauri::command]
fn list_audio_tracks(state: tauri::State<AppState>) -> Vec<AudioTrack> {
    let tracks = state.tracks.lock().unwrap();
    tracks.values().cloned().collect()
}

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Create audio player
    let player = AudioPlayer::new().expect("Failed to initialize audio player");

    let state = AppState {
        tracks: Arc::new(Mutex::new(HashMap::new())),
        player: Arc::new(tokio::sync::Mutex::new(player)),
    };

    Builder::default()
        .manage(state)
        .invoke_handler(generate_handler![
            play_track,
            play_audio,
            pause_audio,
            stop_audio,
            set_volume,
            get_player_state,
            add_audio_track,
            remove_audio_track,
            list_audio_tracks
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}