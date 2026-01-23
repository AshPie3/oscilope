use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};

pub struct Opt {
    sampling: i32,
    decay: f32,
    latency: f32,
    volume: f32,
    max_size: bool,
    screen_width: i32,
    screen_height: i32,
    input_device: Option<String>,

}

fn main() -> anyhow::Result<()> {
    let mut opt =  Opt{
        sampling: 1000,
        decay: 2.0,
        latency: 0.0,
        volume: 400.0,
        max_size: false,
        screen_width: 640,// raylib::window::get_monitor_width(),
        screen_height: 640,
        input_device: None,//Some("Monitor of Bose QC45".to_string()),
    };
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("failed to find input device");
    println!("Using input device: \"{}\"", input_device.id()?);
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    let latency_frames = (opt.latency / 1_000.0) * config.sample_rate as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    let ring = HeapRb::<f32>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.try_push(0.0).unwrap();
    };
    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.try_push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }

    };


    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;

}
fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {err}");
}
