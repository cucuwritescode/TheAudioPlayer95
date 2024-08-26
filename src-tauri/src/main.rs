use tauri::{Builder, generate_context, generate_handler};

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