extern crate sdl2;


use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
struct PixelContent {
    x: i32,
    y: i32,
    is_static: bool
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Alignment {
    Nothing,
    Right(i32),
    Left(i32),
}

impl PartialEq for PixelContent {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PixelContent {
    fn new(x: i32, y: i32) -> Self {
        PixelContent { x, y, is_static: false }
    }

    fn as_static(&self) -> Self {
        PixelContent { x: self.x, y: self.y, is_static: true }
    }

    fn has_side_neighbors(&self, pixels: &Vec<Self>) -> (bool, bool) {
        let has_left = pixels.iter().any(|cp| self.x - 10 == cp.x && cp.y == self.y);
        let has_right = pixels.iter().any(|cp| self.x + 10 == cp.x && cp.y == self.y);
        (has_left, has_right)
    }
}


#[allow(dead_code)]
fn move_pixels(pixels: Vec<PixelContent>) -> Vec<PixelContent>
{
    let mut clone: Vec<PixelContent> = vec![];
    for px in pixels.iter() {
        let next_y = px.y + 10;
        if px.is_static || next_y == HEIGHT as i32 {
            clone.push(PixelContent::new(px.x, px.y).as_static());
            continue;
        } 
        let will_hit_any_static = pixels
            .iter()
            .any(|cp| cp.y == next_y && cp.x == px.x && cp.is_static);

        if will_hit_any_static {
            let mut first_three = pixels
                .iter()
                .filter(|cp| cp.y <= next_y + 20 && cp.x == px.x && cp.is_static && *cp != px)
                .collect::<Vec::<&PixelContent>>();

            first_three.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
            let mut neigh: Vec<(bool, bool)> = vec![];
            for pixel in first_three.iter() {
                neigh.push(pixel.has_side_neighbors(&pixels))
            }
            let mut alignment = Alignment::Nothing;
            let mut last: Option<PixelContent> = None;
            let any_neighbor = neigh.iter().any(|(left, right)| *left && *right);
            if any_neighbor {
                clone.push(PixelContent::new(px.x, px.y).as_static());
                continue
            }
            for (i, (left,right)) in neigh.iter().enumerate() {
                let px = first_three.get(i).unwrap();
                last = Some(**px);
                if *left && !right {
                    alignment = Alignment::Right(px.y);
                    break;
                } else if !left && *right {
                    alignment = Alignment::Left(px.y);
                    break;
                }
            }

            if let Some(px) = last {
                if alignment == Alignment::Nothing {
                    let mut rng = rand::thread_rng();
                    alignment = if rng.gen::<bool>() { Alignment::Right(px.y) } else { Alignment::Left(px.y) };
                }
            }
            let pixel_to_add = match alignment {
                Alignment::Right(_) => { PixelContent::new(px.x + 10, px.y)}
                _ => PixelContent::new(px.x - 10, px.y)
            };
            clone.push(pixel_to_add);
        } else {
            clone.push(PixelContent::new(px.x, next_y));
        }
    }
    clone
}

static WIDTH: u32 = 1280;
static HEIGHT: u32 = 720;

fn round(n: i32) -> i32{
    return n + (10 - (n % 10))
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Pixel cascade", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let speed = 5;
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut pixels: Vec<PixelContent> = vec![];
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
        pixels = move_pixels(pixels);
        canvas.present();
        ::std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_in_empty_vec() {
        let px = PixelContent::new(10, 10);
        let vec: Vec<PixelContent> = vec![px];
        assert_eq!(move_pixels(vec), vec![PixelContent::new(10, 20)])
    }

    #[test]
    fn test_fix_simple_collision() {
        let crr = PixelContent::new(10, 20).as_static();
        let new = PixelContent::new(10, 00);
        let vec: Vec<PixelContent> = vec![crr, new];
        assert_eq!(move_pixels(vec), vec![crr, PixelContent::new(10, 10)])
    }

    #[test]
    fn test_moves_in_the_right_direction() {
        let crr = PixelContent::new(10, 30);
        let new = PixelContent::new(10, 10);
        let vec: Vec<PixelContent> = vec![crr, new];
        assert_eq!(move_pixels(vec), vec![PixelContent::new(10, 40), PixelContent::new(10, 20)])
    }

    #[test]
    fn test_adds_adjacent() {
        let crr = PixelContent::new(10, 30).as_static();
        let new = PixelContent::new(20, 20);
        let vec: Vec<PixelContent> = vec![crr, new];
        assert_eq!(move_pixels(vec), vec![crr, PixelContent::new(20, 30)])
    }

    #[test]
    fn test_adds_piles() {
        let crr_1 = PixelContent::new(10, 50).as_static();
        let crr_2 = PixelContent::new(10, 40).as_static();
        let crr_3 = PixelContent::new(10, 30).as_static();
        let new = PixelContent::new(10, 10);
        let vec: Vec<PixelContent> = vec![crr_1, crr_2, crr_3, new];
        assert_eq!(move_pixels(vec), vec![crr_1, crr_2, crr_3, PixelContent::new(10, 20)]);
    }

    #[test]
    fn test_returns_static_point_when_hit_the_ground() {
        let y = HEIGHT as i32 - 10;
        let new = PixelContent::new(10, y);
        let vec: Vec<PixelContent> = vec![new];
        assert_eq!(move_pixels(vec), vec![PixelContent::new(10, y).as_static()])
    }

    #[test]
    fn test_does_nothing_with_static_point() {
        let px = PixelContent::new(30, 70).as_static();
        assert_eq!(move_pixels(vec![px]), vec![px]);
    }

    #[test]
    fn test_add_to_some_direction_when_pile_is_too_large() {
        let crr_1 = PixelContent::new(30, 70).as_static();
        let crr_2 = PixelContent::new(30, 60).as_static();
        let crr_3 = PixelContent::new(30, 50).as_static();
        let new = PixelContent::new(30, 40);
        let vec: Vec<PixelContent> = vec![crr_1, crr_2, crr_3, new];
        let result = move_pixels(vec);
        let inserted_pixel = result.last().unwrap();
        assert_eq!(true, inserted_pixel.x == 40);
    }

    #[test]
    fn test_add_to_some_direction_when_pile_is_too_large_left() {
        let crr_1 = PixelContent::new(30, 70).as_static();
        let crr_2 = PixelContent::new(30, 60).as_static();
        let crr_3 = PixelContent::new(30, 50).as_static();
        let crr_4 = PixelContent::new(40, 70).as_static();
        let new = PixelContent::new(30, 40);
        let vec: Vec<PixelContent> = vec![crr_1, crr_2, crr_3, crr_4, new];
        let result = move_pixels(vec);
        assert_eq!(vec![crr_1], result);
    
        let inserted_pixel = result.last().unwrap();
        assert_eq!(true, inserted_pixel.x == 20);
    }

    #[test]
    fn test_add_to_simple() {
        let crr_1 = PixelContent::new(30, 70).as_static();
        let new = PixelContent::new(30, 60);
        let vec: Vec<PixelContent> = vec![crr_1, new];
        let result = move_pixels(vec);
        let inserted_pixel = result.last().unwrap();
        let expected: Vec<PixelContent> = vec![crr_1, new.as_static()];
        assert_eq!(expected, result);
    }
}
