use std::f32;
use std::{sync::{Arc, Mutex}, usize};
use ruhear::{rucallback, RUBuffers, RUHear};
use raylib::ffi::Color;
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
    latency: i16,
    volume: f32,
    max_size: bool,
    screen_width: i32,
    screen_height: i32,

}
fn main() {
    let mut buffer = Buffer{
        x: vec![],
        y: vec![],
        samples: 1882,
    };
    
    let mut opt = Opt {
        sampling: 1000,
        latency: 0,
        volume: 400.0,
        max_size: false,
        screen_width: 640,// raylib::window::get_monitor_width(),
        screen_height: 640,
    };
    let mut deque: VecDeque<(f32, f32)> = VecDeque::new();
    
    let (tx, rx) = mpsc::channel();
    let callback = move |audio_buffers: RUBuffers| {
        tx.send(audio_buffers);
    };
    let trail_len = 200 as usize;
    let point_radius = 2.5;
    let (mut rl, thread) = raylib::init()
        .size(opt.screen_width, opt.screen_height)
        .title("Hello, World")
        .build();
    let mut last_clear_frame = 0;
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
        for i in 0..buffer.x.len() {
            //deque.push_back((buffer.x.pop().unwrap(), buffer.y.pop().unwrap()));
            deque.push_back((buffer.x[i], buffer.y[i]));
        };
        d.clear_background(Color {
        r: 10,
        g: 10,
        b: 10,
        a: 255,
        });
        let mut i: usize = 0;
        for i in 0..opt.sampling as usize{
            let ratio = ((opt.sampling as f32 - i as f32) / opt.sampling as f32) ;
            let mut trail_color = ball_color;
            trail_color.r = (ball_color.b as f32 * ratio) as u8;
            trail_color.g = (ball_color.g as f32 * ratio) as u8;
            trail_color.b = (ball_color.b as f32 * ratio) as u8;
            let trail_radius = point_radius; // * ratio ;
            let curr_item = deque.pop_front().unwrap();
            if trail_color.r ==0 && trail_color.g == 0 {
            } else {
              //  d.draw_line_v(Vector2 { x: deque.pop_front().unwrap().0, y: deque.pop_front().unwrap().1}, Vector2 { x: curr_item.0, y: curr_item.1 }, trail_color);
                
            d.draw_circle_v(Vector2 {x: curr_item.0/*buffer.x[i]*/ + 320.0, y: curr_item.1/*buffer.y[i]*/ + 320.0, }, trail_radius, trail_color);
            }
        };
        show_raw(buffer.clone());
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
        let val = samp[x] * volume;
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


