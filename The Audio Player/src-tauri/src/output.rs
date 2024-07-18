use cpal::traits::{DeviceTrait, HostTrait};

pub fn initialize_output() -> cpal::Device {
    let host = cpal::default_host();
    host.default_output_device().expect("no output device available")
}