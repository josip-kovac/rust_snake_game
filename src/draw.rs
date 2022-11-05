use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

/// Our block will scale by 25 pixels.
const BLOCK_SIZE: f64 = 25.0;

/// This function gets coordinate for the game.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/// Draw block for the game.
///
/// # Arguments
///
/// * `color` - color of a  block.
/// * `x` - coordinate of x axis.
/// * `y` - coordinate of y axis.
/// * `con` - context.
/// * `g` - graphics buffer.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        // Parameters for the rectangle block
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

/// Slight modification of the `draw_block()` function. The difference is, now we have width and height. This will allow us to draw rectangles.
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            // This is the main difference
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
