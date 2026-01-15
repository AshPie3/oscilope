use winit::event_loop::EventLoop;

fn main() {
    let amp = 1;
    let mut phase: i32 = 0;
    let freq = 2;
    let mut y_val;
    while true {
        println!("Phase: {}", phase);
        y_val = phase.sin(); 

        phase += 1;
    }

    
}

struct sine_wave {
    amp: i32,
    phase: i32,
    freq: i32,

}

fn sine_wave(amp: i32, phase: i32, freq: i32) -> i32 {
4

}
