use image::{Luma, ImageReader};
use std::io;

fn image_to_binary_grid(image_path: &str) -> Vec<Vec<bool>> {
    let threshold: u8 = 128;
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .into_luma8(); // Convert to grayscale

    let (width, height) = img.dimensions();
    let mut binary_grid: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let Luma([pixel]) = img.get_pixel(x, y);
            binary_grid[y as usize][x as usize] = if *pixel > threshold { false } else { true };
        }
    }
    
    binary_grid
}

const DX: [isize; 4] = [0, 0, 1, -1];
const DY: [isize; 4] = [1, -1, 0, 0];

fn flood_fill_hole(grid: &Vec<Vec<bool>>, visited_hole: &mut Vec<Vec<bool>>, i: isize, j: isize) {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        if x < 0 || y < 0 || x >= rows || y >= cols || visited_hole[x as usize][y as usize] || grid[x as usize][y as usize] {
            continue;
        }
        
        visited_hole[x as usize][y as usize] = true;

        for k in 0..4 {
            stack.push((x + DX[k], y + DY[k]));
        }
    }
}

fn flood_fill_shape(grid: &Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>, visited_hole: &mut Vec<Vec<bool>>, i: isize, j: isize, num_holes: &mut i32) {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        if x < 0 || y < 0 || x >= rows || y >= cols || visited[x as usize][y as usize] {
            continue;
        }

        if !grid[x as usize][y as usize] {
            if !visited_hole[x as usize][y as usize] {
                flood_fill_hole(grid, visited_hole, x, y);
                *num_holes += 1;
            }
            continue;
        }

        visited[x as usize][y as usize] = true;

        for k in 0..4 {
            stack.push((x + DX[k], y + DY[k]));
        }
    }
}


fn count_shapes(grid: &Vec<Vec<bool>>, visited: &mut Vec<Vec<bool>>, visited_hole: &mut Vec<Vec<bool>>, i: isize, j: isize, no_holes: &mut i32, one_hole: &mut i32, two_holes: &mut i32) {
    let mut num_holes = 0;
    flood_fill_shape(grid, visited, visited_hole, i, j, &mut num_holes);
    match num_holes {
        0 => *no_holes += 1,
        1 => *one_hole += 1,
        2 => *two_holes += 1,
        _ => (),
    }
}

fn main() {
    
    let mut input = String::new();
    println!("Key in the name of the image file (make sure it is in the \"source files\" folder):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let grid = image_to_binary_grid(&input);
    let cols = grid[0].len();
    let rows = grid.len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut visited_hole = vec![vec![false; cols]; rows];

    // mark everything outside as visited
    flood_fill_hole(&grid, &mut visited_hole, 0, 0);

    let (mut no_holes, mut one_hole, mut two_holes) = (0, 0, 0);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] && !visited[i][j] {
                count_shapes(&grid, &mut visited, &mut visited_hole, i as isize, j as isize, &mut no_holes, &mut one_hole, &mut two_holes);
            }
        }
    }

    println!("number of 早: {}, number of 上: {}, number of 好: {}", two_holes, no_holes - one_hole, one_hole);

}
