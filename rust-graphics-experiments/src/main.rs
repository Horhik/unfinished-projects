use macroquad::prelude::*;
mod chaos_game;
use std::f64::consts::PI;




#[macroquad::main("BasicShapes")]
async fn main() {

  let mut cx = -0.6;
    let mut cy = 0.07015;

    loop {
    let colors = vec![RED, GREEN, BLUE, YELLOW, MAGENTA, ORANGE, PURPLE];

 // 4 : 3 ratio is nice
    let width = 1920;
    let height = 1080;


    let iterations = 100;


    for x in (0..width).step_by(4) {
        for y in (0..height).step_by(8) {
            let inner_height = height as f32;
            let inner_width = width as f32;
            let inner_y = y as f32;
            let inner_x = x as f32;

            let mut zx = 3.0 * (inner_x - 0.5 * inner_width) / (inner_width);
            let mut zy = 2.0 * (inner_y - 0.5 * inner_height) / (inner_height);

            let mut i = iterations;

            
            while zx * zx + zy * zy < 4.0 && i > 1 {
                let tmp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = tmp;
                i -= 1;
            }

            // guesswork to make the rgb color values look okay
            let r = (i << 4) as u8;
            let g = (i << 3) as u8;
            let b = (i * 5) as u8;
            //let pixel = Rgb::from_channels(r, g, b, 0);
            //img.put_pixel(x as u32, y as u32, pixel);
            let color = color_u8!(r,g,b,255);
            draw_circle(x as f32 + 10.0, y as f32 + 10.0, 4.0, color);


        //clear_background(BLACK);
        //let perms = get_perms(size);

        //let triangle: Vec<(f32, f32)> = vec![(1800.0, 1000.0), (900.0, 000.0), (100.0, 1000.0)];
        //let hexagon = create_hexagon(6);
        //let trg = create_triangle();
        //play_chaos_game(, 3.0).await;
        //play_chaos_game(hexagon, 3.0).await;
        //        play_chaos_game(trg, 2.0).await;
        /*
        for perm in perms {
            // Sleep for...
            //let ten_millis = time::Duration::from_millis(10);
            //thread::sleep(ten_millis);
            // End sleeping.

            let eventy = get_inv(perm.as_slice().to_vec());
            // printing sum ov inversions to screen
            
        */
        //  let permutation = perm.as_slice().to_vec();
        // Drawing matrix elements as dots
        // draw_matrix(size, props.clone()).await;
        // Connecting multiplicated points with lines
        //draw_layer(get_cords(permutation, props.clone()), eventy % 2 == 0).await;
    }


    }
        next_frame().await;
        cx = cx + 0.0131;
        cy = cy + 0.0131;

    }
    }
