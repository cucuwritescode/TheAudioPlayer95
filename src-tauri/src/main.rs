use tauri::{Builder, generate_context, generate_handler};
use serde::{Deserialize,Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

//definir structs para el track de audio del reproductor
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AudioTrack {
    id: String,
    name: String,
    path: String,

}

//estado para manipular los tracks de audio que vaya cargando usuario
struct AppState{
    tracks: Arc<Mutex<String, AudioTrack>>,

}
#[tauri::command]
fn play_audio(file_path: String) {
    println!("Playing audio: {}", file_path);
    
}

#[tauri::command]
fn stop_audio() {
    println!("Stopping audio");
    
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![play_audio, stop_audio])
        .run(generate_context!())
        .expect("error while running tauri application");
}