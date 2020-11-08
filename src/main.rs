extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

#[derive(Debug)]
struct PixelContent {
    x: i32,
    y: i32,
    is_static: bool
}

impl PixelContent {
    fn new(x: i32, y: i32) -> Self {
        PixelContent { x, y, is_static: false }
    }

    fn as_static(&self) -> Self {
        PixelContent { x: self.x, y: self.y, is_static: true }
    }
}

static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let speed = 5;
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut pixels: Vec<PixelContent> = vec![PixelContent::new(290, 10)];
    fn round(n: i32) -> i32{
        return n + (10 - (n % 10))
    }
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonUp { x, y, .. } => {
                    pixels.push(PixelContent::new(round(x), round(y)))
                },
                Event::MouseMotion { x, y, mousestate, .. } => {
                    if mousestate.left() {
                        pixels.push(PixelContent::new(round(x), round(y)))
                    }
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        i = (i + speed) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // draw every pixel
        for pixel in pixels.iter() {
            canvas.fill_rect(Rect::new(pixel.x, pixel.y, 10, 10)).unwrap();
        }
        // compute next pixels value
        pixels = pixels
            .iter()
            .map(|p| {
                let next_y = p.y + 10;
                let will_overlay = pixels.iter().any(|cp| cp.y == next_y && cp.x == p.x && cp.is_static);

                println!("{:?}", next_y);
                if p.is_static || will_overlay || next_y == HEIGHT as i32 {
                    return p.as_static();
                }
                return PixelContent::new(p.x, next_y);
            })
            .collect();

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(20));
    }
}
