use core::f32;
use std::sync::{Arc, Mutex};
use ruhear::{rucallback, RUBuffers, RUHear};
use std::sync::mpsc;


#[derive(Clone)]
struct Buffer {
    x: Vec<f32>,
    y: Vec<f32>,
}


fn main() {
    let mut buffer = Buffer{
        x: vec![],
        y: vec![],
    };

    let (tx, rx) = mpsc::channel();
    let callback = move |audio_buffers: RUBuffers| {
        tx.send(audio_buffers);
    };


    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback);
    

    loop {
        ruhear.start();
        buffer.x = rx.recv().unwrap()[0].clone();
        buffer.y = rx.recv().unwrap()[1].clone();
        show_raw(buffer.clone());
    }
    //ruhear.start();
    //std::thread::sleep(std::time::Duration::from_secs_f32(10.0));
    //ruhear.stop(); 

}


fn parse_stream(data: Vec<Vec<f32>>) -> Buffer {
    println!("Samples: {:?}", &data[0].len());
    let mut buff = Buffer {x: vec![], y: vec![]};
    let value = data.clone();
        for element in &value {
        buff.x.push(element[0]);
        //println!("X: {:?}", buffer.x.last());
        buff.y.push(element[1]);
        }
    println!("Value Channel 0: {:?}", data[0].last());
    println!("Value Channel 1: {:?}", data[1].last());
    buff
}
fn show_raw(buffer: Buffer) {
    println!("X: {:?}, Samples: {:?}", buffer.x.last(), buffer.x.len());
    println!("Y: {:?}, Samples: {:?}", buffer.y.last(), buffer.y.len());
    print!("{}[2J", 27 as char);
}
