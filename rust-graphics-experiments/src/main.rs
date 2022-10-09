use macroquad::prelude::*;
mod chaos_game;
use chaos_game::*;
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let size = 10;
        //clear_background(BLACK);
        //let perms = get_perms(size);

        let triangle: Vec<(f32, f32)> = vec![(1800.0, 1000.0), (900.0, 000.0), (100.0, 1000.0)];
        let hexagon = create_hexagon(6);
        //play_chaos_game(, 3.0).await;
        play_chaos_game(create_triangle(), 3.0).await;
        next_frame().await;
        /*
        for perm in perms {
            // Sleep for...
            //let ten_millis = time::Duration::from_millis(10);
            //thread::sleep(ten_millis);
            // End sleeping.

            let eventy = get_inv(perm.as_slice().to_vec());
            // printing sum ov inversions to screen
            draw_text(
                &eventy.to_string(),
                100.0 * ((size as f32) + 1.0),
                100.0 * ((size as f32) + 1.0),
                200.0,
                WHITE,
            );
        */
        //  let permutation = perm.as_slice().to_vec();
        // Drawing matrix elements as dots
        // draw_matrix(size, props.clone()).await;
        // Connecting multiplicated points with lines
        //draw_layer(get_cords(permutation, props.clone()), eventy % 2 == 0).await;
    }
}
