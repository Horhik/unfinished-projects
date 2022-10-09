extern crate rand;
use macroquad::prelude::*;
use rand::Rng;
use std::cmp::min;
use std::f64::consts::PI;
pub struct Points(Vec<(f32, f32)>);
pub fn create_triangle() -> Points {
    let sw = screen_width();
    let sh = screen_height();
    let max_width = screen_height();
    Points(vec![
        (sw / 2.0, 0.0),
        ((sw / 2.0) - max_width / 2.0, sh),
        ((sw / 2.0) + max_width / 2.0, sh),
    ])
}
pub fn create_hexagon(number_of_vertexes: u32) -> Points {
    let sw = screen_width();
    let sh = screen_height();
    let radius: f32 = sh;
    let center = (sw, sh);
    let mut points: Vec<(f32, f32)> = vec![];
    let step = 2.0 * PI / (number_of_vertexes as f64);
    for i in 0..number_of_vertexes {
        let vertex: (f32, f32) = (
            center.0 + ((step * (i as f64).cos()) as f32) * radius,
            // vertical center of the screen + changed radius for the angle `step*i`
            center.1 + (((step * (i as f64)).sin()) as f32) * radius,
        );
        points.push(vertex);
    }
    //println!("HEXAGON OF {:?}", points);
    Points(points)
}

pub async fn play_chaos_game(_figure: Points, rule: f32) {
    let figure = _figure.0;
    let mut rng = rand::thread_rng();
    let mut i: usize = 2;
    let n: usize = 900000;
    let kek: u32 = rng.gen_range(0..(i as u32));
    let colors = vec![RED, GREEN, BLUE, YELLOW, MAGENTA, ORANGE, PURPLE];
    let mut prev = figure[kek as usize];
    while i < n {
        let n1: u32 = rng.gen_range(0..(figure.len() as u32));
        let color = colors[rng.gen_range(0..7)];
        let c1 = figure[n1 as usize];
        let (x1, y1) = c1;
        let (x2, y2) = prev;
        let (x, y) = ((x1 + x2) / rule, (y1 + y2) / rule);
        //println!("{:?}", c1);

        let next = (x, y);
        prev = next;
        i += 1;
        draw_circle(x + 10.0, y + 10.0, 1.0, color);
    }
}
