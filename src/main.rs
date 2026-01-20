use core::f32;
use std::{borrow::Borrow, ops::Div, sync::{Arc, Mutex}, time::Duration};
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
        //buffer = parse_stream(audio_buffers);
        //buffer.x.push(audio_buffers[0].pop().expect("Not possible"));
        //buffer.y.push(audio_buffers[1].pop().expect("Not possible"));
        //parse_stream(audio_buffers.clone());
        tx.send(audio_buffers);
    };


    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback);
    

    //ruhear.start();
    //std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    //ruhear.stop();
    loop {
        ruhear.start();
        buffer.x = rx.recv().unwrap()[0].clone();
        println!("X: {:?}", buffer.x.last());

        buffer.y = rx.recv().unwrap()[1].clone();
        println!("Y: {:?}", buffer.y.last());
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

