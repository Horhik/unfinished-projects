use itertools::Itertools;
use macroquad::prelude::*;

#[derive(Clone)]
struct Props {
    margin: f32,
    scale: f32,
}

fn get_perms(size: u32) -> Vec<Vec<u32>> {
    let items = 0..size;
    let len = items.len();
    let mut perms = vec![];
    for perm in items.into_iter().permutations(len).unique() {
        perms.push(perm);
    }
    perms
}

async fn draw_matrix(size: u32, props: Props) {
    for i in 0..size {
        for j in 0..size {
            let x: f32 = (i as f32) * props.scale + props.margin;
            let y: f32 = (j as f32) * props.scale + props.margin;
            draw_circle(x, y, 5.0, YELLOW);
        }
    }
}

fn get_cords(initial: Vec<u32>, props: Props) -> Vec<(f32, f32)> {
    let mut kek: Vec<(f32, f32)> = vec![];
    for (n, _) in initial.iter().enumerate() {
        kek.push((
            (n as f32) * props.scale + props.margin,
            (initial[n] as f32) * props.scale + props.margin,
        ));
    }
    kek
}

async fn draw_layer(cords: Vec<(f32, f32)>, is_negate: bool) {
    for i in 0..(cords.len() - 1) {
        let mut color = GREEN;
        let (x1, y1) = cords[i];
        for j in cords.iter().take(cords.len() - 1) {
            let (x2, y2) = *j;
            if is_negate {
                color = RED;
            }
            draw_line(x1, y1, x2, y2, 3.0, color);
            draw_circle(x1, y1, 6.0, color);
            draw_circle(x2, y2, 6.0, color);
        }
    }
}

fn get_inv(perm: Vec<u32>) -> u32 {
    let mut invars = 0;
    for i in 0..(perm.len()) {
        let mut current_sum = 0;
        for j in (i + 1)..(perm.len()) {
            if perm[i] > perm[j] {
                current_sum += 1;
                println!("i: {:?}, j: {:?}", i, j);
            }
        }
        invars += current_sum;
    }
    invars
}
/*fn eg_el<A>(matrix: Vec<Vec<A>>, i: u32, j: u32) {
    matrix[i][j]
}

fn crazyMultiplifier(a: , b: B) {
    for i in
}
*/
