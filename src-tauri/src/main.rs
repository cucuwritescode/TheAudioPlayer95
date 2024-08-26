use tauri::{Builder, generate_context, generate_handler};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define the structure for the audio track
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AudioTrack {
    id: String,
    name: String,
    path: String,
}

// State for managing audio tracks
struct AppState {
    tracks: Arc<Mutex<HashMap<String, AudioTrack>>>,
}

#[tauri::command]
fn play_audio(state: tauri::State<AppState>, file_path: String) {
    println!("Playing audio: {}", file_path);
    let tracks = state.tracks.lock().unwrap();
    if let Some(track) = tracks.get(&file_path) {
        println!("Playing track: {:?}", track);
        // Add logic to play the audio file using the path in the `track`
    }
}

#[tauri::command]
fn stop_audio() {
    println!("Stopping audio");
    // Add logic to stop the audio playback
}

#[tauri::command]
fn add_audio_track(state: tauri::State<AppState>, id: String, name: String, path: String) {
    let mut tracks = state.tracks.lock().unwrap();
    let track = AudioTrack { id, name, path };
    tracks.insert(track.id.clone(), track);
    println!("Added track: {:?}", tracks);
}

#[tauri::command]
fn remove_audio_track(state: tauri::State<AppState>, id: String) {
    let mut tracks = state.tracks.lock().unwrap();
    if tracks.remove(&id).is_some() {
        println!("Removed track with id: {}", id);
    } else {
        println!("No track found with id: {}", id);
    }
}

#[tauri::command]
fn list_audio_tracks(state: tauri::State<AppState>) -> Vec<AudioTrack> {
    let tracks = state.tracks.lock().unwrap();
    tracks.values().cloned().collect()
}

fn main() {
    let state = AppState {
        tracks: Arc::new(Mutex::new(HashMap::new())),
    };

    Builder::default()
        .manage(state)
        .invoke_handler(generate_handler![
            play_audio,
            stop_audio,
            add_audio_track,
            remove_audio_track,
            list_audio_tracks
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}