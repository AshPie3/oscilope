use std::f32;
use std::{sync::{Arc, Mutex}, usize};
use raylib::color::Color;
use ruhear::{rucallback, RUBuffers, RUHear};
use raylib::ffi::Color as OtherColor;
use std::sync::mpsc;
use raylib::prelude::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Buffer {
    x: Vec<f32>,
    y: Vec<f32>,
    samples: i32,
}

pub struct Opt {
    sampling: i32,
    decay: f32,
    latency: i16,
    volume: f32,
    max_size: bool,
    screen_width: i32,
    screen_height: i32,

}
static opt: Opt = Opt{
    sampling: 1000,
    decay: 1.8,
    latency: 0,
    volume: 400.0,
    max_size: false,
    screen_width: 640,// raylib::window::get_monitor_width(),
    screen_height: 640,
};

fn main() {
    let mut buffer = Buffer{
        x: vec![],
        y: vec![],
        samples: 1882,
    };
    
    let (tx, rx) = mpsc::channel();
    let callback = move |audio_buffers: RUBuffers| {
        tx.send(audio_buffers);
    };
    let point_radius = 2.5;
    let (mut rl, thread) = raylib::init()
        .size(opt.screen_width, opt.screen_height)
        .title("Osciloscope")
        .build();
    let mut ball_color = Color {
        r: 57,
        g: 89,
        b: 20,
        a: 255,
    };

    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback);
    ruhear.start();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        buffer.samples = rx.recv().unwrap()[0].len() as i32;            
        buffer.x = parse_samples(rx.recv().unwrap()[0].clone(), buffer.samples, opt.sampling, opt.volume, opt.screen_width, opt.max_size);
        buffer.y = parse_samples(rx.recv().unwrap()[1].clone(), buffer.samples, opt.sampling, opt.volume, opt.screen_height, opt.max_size);
        let mut top_item = Vector2 {x: buffer.x.pop().unwrap(), y: buffer.y.pop().unwrap()};
        d.clear_background(Color {
            r: 10,
            g: 10,
            b: 10,
            a: 255,
        });
        d.draw_circle_v(top_item, point_radius, Color::BLUE);
        for i in 1..opt.sampling as usize-1 {
            show_raw(buffer.clone());
            let ratio = 1.0 - (opt.sampling as f32 - i as f32) / (opt.sampling as f32 * opt.decay);
            let mut trail_color = ball_color;
            trail_color.r = (ball_color.b as f32 * ratio) as u8;
            trail_color.g = (ball_color.g as f32 * ratio) as u8;
            trail_color.b = (ball_color.b as f32 * ratio) as u8;
            let next_item = Vector2 {x: buffer.x.pop().unwrap(), y: buffer.y.pop().unwrap()};
            d.draw_circle_v(next_item, point_radius, trail_color);
            d.draw_line_ex(next_item, top_item, point_radius, trail_color);
            top_item = next_item;

                
        };
    }
}
fn parse_samples(samp: Vec<f32>, samples: i32, sampling: i32, mut volume: f32, screen: i32, max_size: bool) -> Vec<f32> {
    if max_size{
    let mut max = 0.0;
    for i in 0..samp.len(){
        if  max < samp[i]{
                max = samp[i]; 
        }
        };
    volume = (screen as f32 /2.0) / max;
    } else {
    } 
    let mut output = vec![];
    let skip_val = samples / sampling;
    let mut x = 0 as usize;
    while output.len() < sampling as usize && samp.len() >= x {
        let val = samp[x] * volume + screen as f32/2.0 ;
        output.push(val);
        x += skip_val as usize;
    }
    //println!("Buff len: {:?}", output.len());
    output
}

fn show_raw(mut buffer: Buffer) {
    println!("X: {:?}, Samples: {:?}", buffer.x.pop(), buffer.samples);
    println!("Y: {:?}, Samples: {:?}", buffer.y.pop(), buffer.samples);
    println!("Buff len: {:?}", buffer.x.len());


    //print!("{}[2J", 27 as char);
}


