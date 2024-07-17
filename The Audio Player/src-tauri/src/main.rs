#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::sync::{Arc, Mutex};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use player::AudioStreamer;

#[tauri::command]
fn play_audio(streamer: State<'_, Arc<Mutex<AudioStreamer>>>, file_path: String) {
    let mut streamer = streamer.lock().unwrap();
    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);

    *streamer = AudioStreamer::new(reader);
    let device = output::initialize_output();
    streamer.start(&device);
}

#[tauri::command]
fn stop_audio(streamer: State<'_, Arc<Mutex<AudioStreamer>>>) {
    let mut streamer = streamer.lock().unwrap();
    streamer.stop();
}

fn main() {
    let streamer = Arc::new(Mutex::new(AudioStreamer::new(std::io::empty())));

    tauri::Builder::default()
        .manage(streamer)
        .invoke_handler(tauri::generate_handler![play_audio, stop_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}