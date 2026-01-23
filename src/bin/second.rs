use std::f32;
use std::{sync::{Arc, Mutex}, usize};
use anyhow::Ok;
use raylib::color::Color;
use ruhear::{rucallback, RUBuffers, RUHear};
use std::sync::mpsc;
use raylib::prelude::*;

#[derive(Clone)]
pub struct Buffer {
    x: Vec<f32>,
    y: Vec<f32>,
    samples_x: i32,
    samples_y: i32,
}

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
fn main() {
    let mut opt =  Opt{
    sampling: 1000,
    decay: 15.0,
    latency: 0.0,
    volume: 350.0,
    max_size: false,
    screen_width: 800,
    screen_height: 800,
    input_device: None,//Some("Monitor of Bose QC45".to_string()),
    };
    

    let mut buffer = Buffer{
        x: vec![],
        y: vec![],
        samples_x: 1500,
        samples_y: 1500
    };
    
    let (tx, rx) = mpsc::channel();
    let callback = move |audio_buffers: RUBuffers| {
        tx.send(audio_buffers);
    };
    let point_radius = 2.5;
    let (mut rl, thread) = raylib::init()
        .size(opt.screen_width, opt.screen_height)
        .resizable()
        .title("Osciloscope")
        .build();
    opt.screen_width = rl.get_screen_width();
    opt.screen_height = rl.get_screen_height();
    let mut ball_color = Color {
        r: 67,
        g: 255,
        b: 40,
        a: 255,
    };

    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback);
    ruhear.start();

    while !rl.window_should_close() {
        opt.screen_width = rl.get_screen_width();
        opt.screen_height = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);
        let mut recv = rx.recv().unwrap_or_default();

        //buffer.samples_x = recv[1].len() as i32;  
        //buffer.samples_y = recv[0].len() as i32;  

        buffer.x =  std::mem::replace(&mut recv[0], vec![]);
        buffer.y =  std::mem::replace(&mut recv[1], vec![]);
        
        buffer.x = parse_samples(buffer.x, opt.volume, opt.screen_width, opt.max_size);
        buffer.y = parse_samples(buffer.y, opt.volume, opt.screen_height, opt.max_size);

        //show_raw(buffer.clone());
        d.draw_rectangle(0,0, opt.screen_width, opt.screen_height, Color::BLACK.alpha(0.8));
        let mut last: Option<Vector2>= None;
        while buffer.x.len()>0 && buffer.y.len()>0{
            //show_raw(buffer.clone());
            let sample = Vector2 {x: buffer.x.pop().unwrap(), y: buffer.y.pop().unwrap()};
            let mut trail_color = ball_color;
            let previous_sample = last.unwrap_or(sample);
            trail_color.a = ((opt.screen_width * opt.screen_height) as f32 * (1.0/opt.decay) /
                ((sample.x - previous_sample.x).powi(2) + (sample.y - previous_sample.y).powi(2)) as f32).clamp(10.0, 255.0) as u8 ;
            d.draw_line_ex(sample, previous_sample, point_radius, trail_color);
            //d.draw_circle_v(sample, point_radius, ball_color);
            last = Some(sample)
            //} else {}
        };
    }
    
}
fn parse_samples(samp: Vec<f32>, mut volume: f32, screen: i32, max_size: bool) -> Vec<f32> {
    if max_size{
        let mut max = 0.0;
        samp.iter().for_each(|val| max = f32::max(val.abs(), max)); 
        volume = (screen as f32 /2.0) / max;
    } 
    samp.iter().map(|val| val * volume + (screen as f32/2.0)).collect::<Vec<f32>>()
}

fn show_raw(mut buffer: Buffer) {
    println!("X: {:?} ", buffer.x.pop());
    println!("Y: {:?} ", buffer.y.pop());
    println!("Buff len: {:?}", buffer.x.len());
    //print!("{}[2J", 27 as char);
}


