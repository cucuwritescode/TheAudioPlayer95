#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;
use std::fs::File;
use std::sync::Mutex;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::sample::{u24, i24};
use symphonia::default::get_probe;
use tauri::State;

#[derive(Deserialize)]
struct FileInfo {
    path: String,
}

struct AudioPlayer {
    sink: Mutex<Option<rodio::Sink>>,
}

#[tauri::command]
async fn play(file_info: FileInfo, audio_player: State<'_, AudioPlayer>) -> Result<(), String> {
    let file_path = &file_info.path;
    match play_audio(file_path, &audio_player) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

fn u24_to_f32(sample: u24) -> f32 {
    let bytes = sample.to_ne_bytes();
    let value = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    value as f32 / 16777215.0
}

fn i24_to_f32(sample: i24) -> f32 {
    let bytes = sample.to_ne_bytes();
    let value = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    value as f32 / 8388607.0 
}

fn play_audio(file_path: &str, audio_player: &State<'_, AudioPlayer>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let hint = Hint::new();
    let probed = get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;

    let track = format.tracks().iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or("No supported audio tracks found")?;

    let mut decoder = symphonia::default::get_codecs().make(
        &track.codec_params,
        &DecoderOptions::default(),
    )?;

    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&stream_handle)?;
    *audio_player.sink.lock().unwrap() = Some(sink);

    while let Ok(packet) = format.next_packet() {
        if let Ok(decoded) = decoder.decode(&packet) {
            match decoded {
                AudioBufferRef::U8(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| (s as f32 - 128.0) / 128.0).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::U16(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| s as f32 / 65535.0).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::U24(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| u24_to_f32(s)).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::U32(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| s as f32 / u32::MAX as f32).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::F32(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).to_vec();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::F64(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| s as f32).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::S8(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| s as f32 / i8::MAX as f32).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::S16(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::S24(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| i24_to_f32(s)).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
                AudioBufferRef::S32(buffer) => {
                    let samples: Vec<f32> = buffer.chan(0).iter().map(|&s| s as f32 / i32::MAX as f32).collect();
                    let source = rodio::buffer::SamplesBuffer::new(
                        buffer.spec().channels.count() as u16,
                        buffer.spec().rate,
                        samples,
                    );
                    audio_player.sink.lock().unwrap().as_ref().unwrap().append(source);
                },
            }
        }
    }

    audio_player.sink.lock().unwrap().as_ref().unwrap().sleep_until_end();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AudioPlayer { sink: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}