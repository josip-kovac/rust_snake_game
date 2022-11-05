use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;
use std::fmt;

// this is how we will build our (green) snake.
use crate::draw::draw_block;

/// Color of a snake. Currently, it is green.
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

/// TODO: this could be in init signature.
const SNAKE_DEFAULT_LEN: i32 = 3;



#[derive(Clone, Copy, PartialEq, Debug)]
/// This handles the direction of the Snake.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// TODO: I will try to explain this later.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
/// Coordinates of a Block. This does not need to be public because we are not exporting it anywhere.
struct Block {
    x: i32,
    y: i32,
}

/// Struct of a Snake (no shit, Sherlock, LOL).
pub struct Snake {
    /// Currently active direction of snake.
    direction: Direction,
    /// Body of a snake (LinkedList of Blocks).
    body: LinkedList<Block>,
    /// TODO DOC. [Video link](https://youtu.be/DnT_7M7L7vo?t=709).
    tail: Option<Block>,
}

impl Snake {
    /// Initialization of a snake (redundant doc, LOL).
    pub fn new(x: i32, y: i32) -> Snake {

        // Init of body.
        let mut body: LinkedList<Block> = LinkedList::new();

        // This is how we will set our default snake.
        // Snake will have length of 3 (SNAKE_DEFAULT_LEN).

        for offset in (0..SNAKE_DEFAULT_LEN).rev() {
            body.push_back(Block {x: x + offset, y: y});
        }

        Snake {
            // By default, our snake will move to the right.
            direction: Direction::Right,
            // The body is already defined.
            body: body,
            // Our snake will not have any tail.
            tail: None,
        }
    }

    /// This function will draw our snake onto the context.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        // With .front(), I am getting the last element of LinkedList.
        // with .unwrap(), I am getting rid of an Option Enum, without having to write explicit Error handling. This practice is discouraged, but it is used here because I know this cannot fail.
        let head_block = self.body.front().unwrap();

        // we can return this tuple because Block struct has x and y defined.
        (head_block.x, head_block.y)
    }

    /// Definition of snake move.
    pub fn move_forward(&mut self, dir: Option<Direction>) {

        // This will change direction of snake if there is a key for direction.
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                // This seems inverted, but actually it is not. The reference point is the origin of snake.
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        
        // Now, the final block will move to that new position.
        self.body.push_front(new_block);

        // But, we need to remove the last block, in order to keep length consistent. Comment out last following two lines for some LOLs.
        let removed_block = self.body.pop_back().unwrap();
        // The removed block is now our tail. But, TBH, I don't understand why we do this self.tail operation. @TODO
        self.tail = Some(removed_block);
    }

    /// We are just returning the direction snake is heading to.
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// Function for manipulation of head position based on Direction.
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let moving_dir: Direction;

        match dir {
            Some(d) => moving_dir = d,
            None => moving_dir = self.direction
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    /// When Snake eats an apple, the tail will grow by one Block.
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        // Original code had `let mut ch = 0;` as a way to index error, but for block takes care of that for us!
        // let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            // This code is reduntant, for block takes care of this by design.
            // ch += 1;
            // if ch == self.body.len() - 1 {
            //     break;
            // }
        }
        return false;
    }

    // pub fn len(&self) -> usize {
    //     self.body.len()
    // }

}

impl fmt::Debug for Snake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Snake")
        .field("direction", &self.direction)
        .field("body (len)", &self.body.len())
        .finish()
    }
}
