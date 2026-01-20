use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use ruhear::{rucallback, RUBuffers, RUHear};
use std::thread;

fn main() -> Result<()> {
    //Set up device
    //let host = cpal::default_host();
    //let device = host.default_input_device().expect("failed to find input device");
    // Set up config
    //let config: cpal::StreamConfig = device.default_input_config()?.into();
    // Set up input stream
    //let input_stream = device.build_input_stream(&config, parse_stream, err_fn, None)?;
    // Main loop
    println!("Starting input stream ");
    // ruhear capture audio output
    let callback = |audio_buffers: RUBuffers| {
        parse_stream(audio_buffers.clone());
    };
    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback);

    ruhear.start();
    std::thread::sleep(std::time::Duration::from_secs(100));
    ruhear.stop();

    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {err}");
}

fn parse_stream(data: Vec<Vec<f32>>) {
    println!("Samples: {:?}", data[0].len());
    println!("Value Channel 0: {:?}", data[0].last());
    println!("Value Channel 1: {:?}", data[1].last());
    std::thread::sleep(std::time::Duration::from_secs(1));
}

