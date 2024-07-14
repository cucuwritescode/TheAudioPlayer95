//arrancamos importando los modulos que voy a necesitar 
//del crate de symophonia para manejar archivos wav
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::FormatOptions;
use symphonia::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
//modulos necesarios de rodio para el playback de audio
use rodio::{Decoder as RodioDecoder, OutputStream, Source}; 

//estas proximas dos son librerias estandar
use std::io::BufReader;
use std::fs::File;
use std::time::Duration;

fn main() {
    let file = File::open("audio.wav").expect("Failed to open file");
    let buf_reader= BufReader::new(File);
    let media_source_stream= MediaSourceStream::new(Box::new(buf_reader)),Default::default();
    let hint= Hint::new();
    let probed= get_probe().format( //para descifrar el formato del archivo de audio
        &hint,
        media_source_stream,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ).expect("Failed to probe format");
    let format= probed.format; //extraemos el formato (lo almaceno en una variable)
    let track= format.tracks().iter()



    
}



