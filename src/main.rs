// Import the necessary traits from the cpal crate
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Import the Decoder and OutputStream structs from the rodio crate
use rodio::{Decoder, OutputStream};

// Import the BufReader struct from the standard library's io module
use std::io::BufReader;

// Import the File struct from the standard library's fs module
use std::fs::File;

// Define the main function, which is the entry point of the program
fn main() {
    // Open an audio file
    let file = File::open("audio.mp3").unwrap();
    
    // Create a new Decoder object to decode the audio file
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Get the default audio output device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Play the decoded audio
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Keep the main thread alive while the audio plays
    std::thread::sleep(std::time::Duration::from_secs(5));
}



