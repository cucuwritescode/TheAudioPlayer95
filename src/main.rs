//arrancamos importando los modulos que voy a necesitar 
//del crate de symophonia para manejar archivos wav

use symphonia::core::codecs::DecoderOptions;
use symphonia::core::FormatOptions;
use symphonia::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

//estas proximas dos son librerias estandar
use std::io::BufReader;
use std::fs::File;




