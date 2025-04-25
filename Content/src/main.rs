// Image processing module imports
use image::{Luma, ImageReader};
use std::io;

/// Converts an image to a binary grid based on luminance threshold
/// 
/// # Arguments
/// * `image_path` - Path to the input image file
/// 
/// # Returns
/// 2D vector where true represents dark pixels (below threshold) 
/// and false represents light pixels
fn image_to_binary_grid(image_path: &str) -> Vec<Vec<bool>> {
    // Luminance threshold for binarization (0-255)
    let threshold: u8 = 250;
    
    // Load and convert image to grayscale (8-bit luminance)
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .into_luma8();

    let (width, height) = img.dimensions();
    let mut binary_grid: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];

    // Convert each pixel to binary based on threshold
    for y in 0..height {
        for x in 0..width {
            let Luma([pixel]) = img.get_pixel(x, y);
            // true = dark pixel (character), false = light pixel (background)
            binary_grid[y as usize][x as usize] = *pixel <= threshold;
        }
    }
    
    binary_grid
}

// 8-directional neighborhood offsets for flood fill (N, S, E, W, NE, NW, SE, SW)
const DX: [isize; 8] = [0, 0, 1, -1, 1, -1, 1, -1];
const DY: [isize; 8] = [1, -1, 0, 0, 1, -1, -1, 1];

/// Flood fills a contiguous "hole" region (background pixels enclosed by shapes)
/// 
/// # Arguments
/// * `grid` - Reference to the binary image grid
/// * `visited_hole` - Mutable reference to track visited hole pixels
/// * `i`, `j` - Starting coordinates for the fill
fn flood_fill_hole(grid: &Vec<Vec<bool>>, visited_hole: &mut Vec<Vec<bool>>, i: isize, j: isize) {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let mut stack = vec![(i, j)];  // Using stack for DFS implementation

    while let Some((x, y)) = stack.pop() {
        // Skip if out of bounds, already visited, or part of a shape
        if x < 0 || y < 0 || x >= rows || y >= cols || 
           visited_hole[x as usize][y as usize] || 
           grid[x as usize][y as usize] {
            continue;
        }
        
        visited_hole[x as usize][y as usize] = true;

        // Add all 8 neighbors to stack
        for k in 0..8 {
            stack.push((x + DX[k], y + DY[k]));
        }
    }
}

/// Flood fills a shape and counts its enclosed holes (negative space)
/// 
/// # Arguments
/// * `grid` - Binary image grid
/// * `visited` - Tracks visited shape pixels
/// * `visited_hole` - Tracks visited hole pixels
/// * `i`, `j` - Starting coordinates
/// * `num_holes` - Counter for holes found in this shape
fn flood_fill_shape(
    grid: &Vec<Vec<bool>>, 
    visited: &mut Vec<Vec<bool>>, 
    visited_hole: &mut Vec<Vec<bool>>, 
    i: isize, 
    j: isize, 
    num_holes: &mut i32
) {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        // Skip if out of bounds or already visited
        if x < 0 || y < 0 || x >= rows || y >= cols || visited[x as usize][y as usize] {
            continue;
        }

        // If we hit background (potential hole)
        if !grid[x as usize][y as usize] {
            if !visited_hole[x as usize][y as usize] {
                flood_fill_hole(grid, visited_hole, x, y);
                *num_holes += 1;  // Found a new enclosed hole
            }
            continue;
        }

        visited[x as usize][y as usize] = true;

        // Explore 8-connected neighbors
        for k in 0..8 {
            stack.push((x + DX[k], y + DY[k]));
        }
    }
}

/// Counts shapes categorized by their number of enclosed holes
/// 
/// # Arguments
/// * `grid` - Binary image grid
/// * `visited` - Tracks visited shape pixels  
/// * `visited_hole` - Tracks visited hole pixels
/// * `i`, `j` - Starting coordinates
/// * `no_holes`, `one_hole`, `two_holes` - Counters for each shape type
fn count_shapes(
    grid: &Vec<Vec<bool>>, 
    visited: &mut Vec<Vec<bool>>, 
    visited_hole: &mut Vec<Vec<bool>>, 
    i: isize, 
    j: isize, 
    no_holes: &mut i32, 
    one_hole: &mut i32, 
    two_holes: &mut i32
) {
    let mut num_holes = 0;
    flood_fill_shape(grid, visited, visited_hole, i, j, &mut num_holes);
    
    // Categorize shape by number of holes
    match num_holes {
        0 => *no_holes += 1,  // Solid shape
        1 => *one_hole += 1,   // Shape with one hole
        2 => *two_holes += 1,  // Shape with two holes
        _ => panic!("Input image is not valid: A shape has been found with more than 2 holes."),
    }
}

fn main() {
    // Get input image path from user
    let mut input = String::new();
    println!("Key in the name of the image file (make sure it is in the release folder):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    
    // Process image into binary grid
    let grid = image_to_binary_grid(&input);
    let cols = grid[0].len();
    let rows = grid.len();
    
    // Visited trackers for shapes and holes
    let mut visited = vec![vec![false; cols]; rows];
    let mut visited_hole = vec![vec![false; cols]; rows];

    // Mark all outer background as visited (not part of shape holes)
    flood_fill_hole(&grid, &mut visited_hole, 0, 0);

    // Counters for different shape types
    let (mut no_holes, mut one_hole, mut two_holes) = (0, 0, 0);
    
    // Process all unvisited pixels
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] && !visited[i][j] {
                count_shapes(
                    &grid, 
                    &mut visited, 
                    &mut visited_hole, 
                    i as isize, 
                    j as isize, 
                    &mut no_holes, 
                    &mut one_hole, 
                    &mut two_holes
                );
            }
        }
    }

    // Output results (assuming specific Chinese character shapes)
    println!("number of 早: {}, number of 上: {}, number of 好: {}", 
        two_holes,     // Shapes with 2 holes (早)
        no_holes - one_hole, // Adjusted count for 上 
        one_hole       // Shapes with 1 hole (好)
    );
}