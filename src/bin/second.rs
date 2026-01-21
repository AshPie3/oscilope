use std::array::repeat;
use std::f32;
use std::{iter, sync::{Arc, Mutex}, usize};
use raylib::ffi::Fade;
use ruhear::{rucallback, RUBuffers, RUHear};
//use wgpu::Color;
use raylib::ffi::Color;
use std::sync::mpsc;
use raylib::prelude::*;

#[derive(Clone)]
pub struct Buffer {
    x: Vec<f32>,
    y: Vec<f32>,
    samples: i32,
}

pub struct Opt {
    sampling: i32,
    latency: i16,
    volume: i32,            

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
        volume: 40000,
    };
    
    let (tx, rx) = mpsc::channel();
    let callback = move |audio_buffers: RUBuffers| {
        tx.send(audio_buffers);
    };
    let trail_len = 200 as usize;
    let mut trail_pos = Vec::new();
    for i in 0..trail_len {
        trail_pos.push(Vector2 { x: 320.0, y: 320.0 });
    }
    let point_radius = 2.5;
    let (mut rl, thread) = raylib::init()
        .size(640, 640)
        .title("Hello, World")
        .build();
    let mut ball_pos = Vector2 {x: 320.0, y: 240.0};
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
         
        d.clear_background(Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
        });
        buffer.samples = rx.recv().unwrap()[0].len() as i32;            
        buffer.x = parse_samples(rx.recv().unwrap()[0].clone(), buffer.samples, opt.sampling, opt.volume);
        buffer.y = parse_samples(rx.recv().unwrap()[1].clone(), buffer.samples, opt.sampling, opt.volume);
        let mut i = 0;
        //d.draw_circle_v(Vector2 {x: buffer.x.pop().expect("Reason") + 320.0, y: buffer.y.pop().expect("Reason") + 320.0, }, point_radius, ball_color);
        while buffer.x.len()>0 {
            let ratio = ((opt.sampling - i) / opt.sampling) as f32;
            let trail_color = ball_color;
            //trail_color.a * ratio as u8 ^ 3 ;
            let trail_radius = point_radius; // * ratio ;
            d.draw_circle_v(Vector2 {x: buffer.x.pop().expect("Reason") + 320.0, y: buffer.y.pop().expect("Reason") + 320.0, }, trail_radius, trail_color);
            i= i + 1;
        };

        //while buffer.x.len()>0 {
        //   
        //};
        //ball_pos.x = buffer.x.pop().unwrap() + 320.0;
        //ball_pos.y = buffer.y.pop().unwrap() + 240.0;
        //d.draw_text(buffer.x.last().unwrap().to_string().as_str() , 12, 12, 20, Color::BLACK);
        //d.draw_text(buffer.y.last().unwrap().to_string().as_str() , 12, 32, 20, Color::BLACK);
        //d.draw_circle_v(ball_pos, 10.0, Color::BLUE);
        //show_raw(buffer.clone());
    }
}
fn parse_samples(samp: Vec<f32>, samples: i32, sampling: i32, volume: i32) -> Vec<f32> {
    let mut output = vec![];
    let skip_val = samples / sampling;
    let mut x = 0 as usize;
    while output.len() <= sampling as usize && samp.len() >= x {
        let val = samp[x] * (volume/100) as f32;
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


