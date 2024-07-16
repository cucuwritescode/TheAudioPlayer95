// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use symphonia::default::{get_probe, get_codecs};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::conv::Convertible;

// Struct to manage audio state
struct AudioState {
    sink: Option<Arc<Mutex<Sink>>>,
}

#[tauri::command]
fn play_audio(file_path: String, state: tauri::State<'_, AudioState>) -> Result<(), String> {
    let (stream, stream_handle) = OutputStream::try_default().map_err(|e| e.to_string())?;
    let file = File::open(&file_path).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    
    let format = get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| e.to_string())?;

    let track = format.default_track().ok_or("No default track found")?;
    let codec_params = &track.codec_params;
    let mut decoder = get_codecs()
        .make(&codec_params, &Default::default())
        .map_err(|e| e.to_string())?;

    let spec = *decoder.codec_params().sample_rate.unwrap();
    let mut sample_buffer = SampleBuffer::<f32>::new(codec_params.frames, spec);

    let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

    while !format.is_at_end() {
        let packet = format.next_packet().map_err(|e| e.to_string())?;
        decoder.decode(&packet).map_err(|e| e.to_string())?;
        
        if let Some(decoded) = decoder.last_decoded() {
            sample_buffer.copy_interleaved_ref(decoded);
            let source = rodio::buffer::SamplesBuffer::new(
                decoded.spec().channels.count() as u16,
                decoded.spec().rate,
                sample_buffer.samples(),
            );
            sink.append(source);
        }
    }

    let sink = Arc::new(Mutex::new(sink));
    *state.sink.lock().unwrap() = Some(sink.clone());

    Ok(())
}

#[tauri::command]
fn pause_audio(state: tauri::State<'_, AudioState>) {
    if let Some(sink) = &*state.sink.lock().unwrap() {
        sink.lock().unwrap().pause();
    }
}

#[tauri::command]
fn resume_audio(state: tauri::State<'_, AudioState>) {
    if let Some(sink) = &*state.sink.lock().unwrap() {
        sink.lock().unwrap().play();
    }
}

#[tauri::command]
fn stop_audio(state: tauri::State<'_, AudioState>) {
    if let Some(sink) = &*state.sink.lock().unwrap() {
        sink.lock().unwrap().stop();
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AudioState { sink: None })
        .invoke_handler(tauri::generate_handler![play_audio, pause_audio, resume_audio, stop_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}