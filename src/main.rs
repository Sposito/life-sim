extern crate sdl2;


use std::sync::Arc;
use std::string::String;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

// There will be 16 particles:
// coding: a, t, g
// encoded: aine, bine, cine, dine, eine, fine, gine, hine, gine
// energy: sugar, oil

pub struct Particle{
    id : i16,
    kind: i16,
    position: (i16, i16),
    velocity: (i16, i16),
    color: Color,
    charge: i8,
    chain0: Option<Arc<Particle>>,
    chain1: Option<Arc<Particle>>,
    chain2: Option<Arc<Particle>>,
}

pub fn get_a(pos:(i16, i16)) -> Particle {
    let a: Particle = Particle {
        id: 0,
        kind: 0 << 0,
        position: pos,
        velocity: (0, 0),
        color: Color::RGB(255, 255, 0),
        charge: 2,
        chain0 : None,
        chain1: None,
        chain2: None,
    };
    a


}


pub fn main() {
    let m:Particle = get_a((0,0));
    print!("{:?}", m.position);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut pi = (20,20);
    let mut pf = (200,200);
    let mut pi_s =(1,1);
    let mut pf_s =(1,1);

    let mut px = 0;
    let mut py = 0;

    'running: loop {
        i = (i + 1) % 255;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

        canvas.set_draw_color(Color::RGB(0, 0, 0));
       // canvas.clear();
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 127));
        pi = (pi.0 + pi_s.0, pi.1 + pi_s.1);
        // pf = (pf.0 + pf_s.0, pf.1 + pf_s.1);


        for t in 0..1000{
            px = rand::thread_rng().gen_range(0.. 800);
            py = rand::thread_rng().gen_range(0..600);
            canvas.draw_point((px, py));
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100));
    }
}