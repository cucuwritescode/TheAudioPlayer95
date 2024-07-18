#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, State};
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Make sure to import the `AudioStreamer` and other necessary types
mod player;
use player::AudioStreamer;

struct AudioStreamerState {
    streamer: Arc<Mutex<AudioStreamer>>,
}

#[tauri::command]
fn play_audio(state: State<'_, AudioStreamerState>, file_path: String) {
    let streamer = state.streamer.lock().unwrap();
    // Add code to play audio using the streamer and file_path
}

#[tauri::command]
fn stop_audio(state: State<'_, AudioStreamerState>) {
    let streamer = state.streamer.lock().unwrap();
    // Add code to stop audio using the streamer
}

fn main() {
    tauri::Builder::default()
        .manage(AudioStreamerState {
            streamer: Arc::new(Mutex::new(AudioStreamer::new())),
        })
        .invoke_handler(tauri::generate_handler![play_audio, stop_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}