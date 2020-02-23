mod utils; 
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub struct TileChange {
    x: i32,
    y: i32,
    rot_degree: f64 
}

impl TileChange {
    pub fn new() -> TileChange {
        TileChange {
            x: 0,
            y: 0,
            rot_degree: 0.0
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.rot_degree = 0.0;
    }
}

#[derive(Debug)]
pub struct FallingTile {
    indexes: [usize; 4],
    uncommited_change: TileChange,
    center: usize,
    tile_type: TileType,
    rotate_this_frame: bool,
}

// TODO: board_width is required on all functions, should be a member? (yes)
impl FallingTile {
    pub fn new(board_width: usize) -> FallingTile {
        let mut rng = thread_rng();
        let selected: u32 = rng.gen_range(1, 8);

        let mut rtr_tile = match selected {
            1 => FallingTile::variation_1(board_width),
            2 => FallingTile::variation_2(board_width),
            3 => FallingTile::variation_3(board_width),
            4 => FallingTile::variation_4(board_width),
            5 => FallingTile::variation_5(board_width),
            6 => FallingTile::variation_6(board_width),
            7 => FallingTile::variation_7(board_width),
            _ => {
                log!("unexpected random value: {}", selected);
                panic!("unexpected random value")
            }
        };
        
        // place tile set in the middle
        for i in 0..rtr_tile.indexes.len() {
            rtr_tile.indexes[i] += ((board_width - 2) as f64 * 0.5) as usize;
        }

        rtr_tile.center += ((board_width - 2) as f64 * 0.5) as usize;

        return rtr_tile;
    }

    pub fn commit_changes(&mut self, board_width: usize) {
        self.commit_rotate(self.uncommited_change.rot_degree, board_width);

        let velocity =  self.uncommited_change.x + self.uncommited_change.y * board_width as i32;

        self.center = (self.center as i32 + velocity) as usize;
        
        for i in 0..self.indexes.len() {
            self.indexes[i] = (self.indexes[i] as i32 + velocity) as usize;
        }

        self.uncommited_change.reset();
    }

    pub fn as_virtual(&self, index: usize, board_width: usize) -> usize {
        let virtual_rot_index = self.rotate_specific(index, self.uncommited_change.rot_degree.to_radians(), board_width);
        
        (virtual_rot_index as i32 + self.uncommited_change.x + self.uncommited_change.y * board_width as i32) as usize
    }

    pub fn rotate_specific(&self, index: usize, radians: f64, board_width: usize) -> usize {
        let cos = radians.cos();
        let sin = radians.sin();

        let x = (self.indexes[index] % board_width) as f64 - (self.center % board_width) as f64;
        let y = (self.indexes[index] / board_width) as f64 - (self.center / board_width) as f64;

        let rot_x = (x * cos) - (y * sin);
        let rot_y = (x * sin) + (y * cos);

        let y_change = rot_y.round() * board_width as f64; 

        (self.center as f64 + y_change + rot_x.round()) as usize
    }

    pub fn rotate(&mut self, degree: f64) {
        self.uncommited_change.rot_degree += degree;
    }

    fn commit_rotate(&mut self, degree: f64, board_width: usize) {
        let radians = degree.to_radians();
        for i in 0..self.indexes.len() {
            self.indexes[i] = self.rotate_specific(i, radians, board_width);
        }
    }

    #[inline]
    fn variation_1(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [1, board_width + 1, board_width * 2 + 1, board_width * 3 + 1],
            uncommited_change: TileChange::new(),
            center: board_width + 1,
            tile_type: TileType::Turquoise,
            rotate_this_frame: false
        }
    }

    #[inline]
    fn variation_2(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [1, board_width + 1, board_width * 2 + 0, board_width * 2 + 1],
            uncommited_change: TileChange::new(),
            center: board_width + 1,
            tile_type: TileType::Blue,
            rotate_this_frame: false
        }
    }

    #[inline]
    fn variation_3(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [1, board_width + 1, board_width * 2 + 1, board_width * 2 + 2],
            uncommited_change: TileChange::new(),
            center: board_width + 1,
            tile_type: TileType::Orange,
            rotate_this_frame: false
        }
    }

    #[inline]
    fn variation_4(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [1, 2, board_width + 1, board_width + 2],
            uncommited_change: TileChange::new(),
            center: board_width + 1,
            tile_type: TileType::Yellow,
            rotate_this_frame: false
        }
    }

    #[inline]
    fn variation_5(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [1, 2, board_width + 1, board_width + 0],
            uncommited_change: TileChange::new(),
            center: board_width + 1,
            tile_type: TileType::Green,
            rotate_this_frame: false
        }
    }

    #[inline]
    fn variation_6(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [1, board_width + 0, board_width + 1, board_width + 2],
            uncommited_change: TileChange::new(),
            center: board_width + 1,
            tile_type: TileType::Purple,
            rotate_this_frame: false
        }
    }

    #[inline]
    fn variation_7(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: [0, 1, board_width + 1, board_width + 2],
            uncommited_change: TileChange::new(),
            center: 1,
            tile_type: TileType::Red,
            rotate_this_frame: false
        }
    }

}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Empty = 1,
    Turquoise = 2,
    Blue = 3,
    Orange = 4,
    Yellow = 5,
    Green = 6,
    Purple = 7,
    Red = 8,
}

#[derive(PartialEq, Eq)]
pub enum CollisionEvent {
    Nop,
    Bottom,
    Side
}


#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    size: usize,
    score: i32,
    // indexes that are in the falling tile
    falling: FallingTile, 
    tiles: [TileType; 512],
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        utils::set_panic_hook();

        let width: usize = 10;
        let height: usize = 20;
        let size = width * height;
        let score: i32 = 0;
        let tiles = [TileType::Empty; 512];

        Board {
            width,
            height,
            size,
            falling: FallingTile::new(width),
            score,
            tiles,
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.tiles.len() {
            self.tiles[i] = TileType::Empty;
        }

        self.score = 0;
        self.falling = FallingTile::new(self.width);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn tiles_ptr(&self) -> *const TileType {
        self.tiles.as_ptr()
    }

    pub fn move_left(&mut self) {
        self.falling.uncommited_change.x = -1;
    }

    pub fn move_rigth(&mut self) {
        self.falling.uncommited_change.x = 1;
    }
    
    pub fn rotate(&mut self) {
        self.falling.rotate_this_frame = true;
    }
    
    pub fn tiles_len(&self) -> usize {
        self.tiles.len()
    }
    
    pub fn update_rotate_stride(&mut self) {
        for i in &self.falling.indexes {
            self.tiles[*i] = TileType::Empty;
        }

        // handle stride
        'stride: for i in 0..self.falling.indexes.len() {
            if self.is_colliding(i) == true {
                self.falling.uncommited_change.x = 0;
                break 'stride;
            }
        }

        // handle rotation
        if self.falling.rotate_this_frame {
            self.falling.rotate_this_frame = false;
            self.rotate_right();

            'rot_collide: for i in 0..self.falling.indexes.len() {
                if self.is_colliding(i) == true {
                    self.undo_rotation();
                    break 'rot_collide;
                }
            }
        }

        self.falling.commit_changes(self.width);

        for i in &self.falling.indexes {
            self.tiles[*i] = self.falling.tile_type;
        }
    }

    pub fn update_fall(&mut self) { 
        for i in &self.falling.indexes {
            self.tiles[*i] = TileType::Empty;
        }

        // handle falling
        self.falling.uncommited_change.y = 1;
        let mut bottom_reached = false;
        'falling: for i in 0..self.falling.indexes.len() {
            bottom_reached = self.is_colliding(i); 
            if bottom_reached {
                self.falling.uncommited_change.y = 0;
                break 'falling;
            }
        }

        for i in &self.falling.indexes {
            self.tiles[*i] = self.falling.tile_type;
        }

        if bottom_reached {
            self.on_new_tile();
        }
    }
    
    fn undo_rotation(&mut self) {
        self.falling.rotate(-90.0);
    }

    fn rotate_right(&mut self) {
        for i in &self.falling.indexes {
            self.tiles[*i] = TileType::Empty;
        }

        self.falling.rotate(90.0);
    }

    fn on_new_tile(&mut self) {
        // Scan board for complete lines
        for i in 0..self.height {
            let mut row_count = 0;
            for j in 0..self.width {
                if self.tiles[(i * self.width) + j] != TileType::Empty {
                    row_count += 1;
                }
            }
            if row_count >= self.width {
                self.score += 10;

                for j in 0..self.width {
                    self.tiles[i * self.width + j] = TileType::Empty;
                }

                // make all tiles over fall
                // TODO @refactor: if no tiles had color on line, break loop
                'fall_loop: for j in (1..(i + 1)).rev() {
                    for k in 0..self.width {
                        let tile_over_index = (j - 1) * self.width + k;

                        let tile_under_index = j * self.width + k;

                        if self.tiles[tile_over_index] != TileType::Empty && self.tiles[tile_under_index] == TileType::Empty {
                            self.tiles[tile_under_index] = self.tiles[tile_over_index];
                            self.tiles[tile_over_index] = TileType::Empty;
                        }
                    }
                }
            }
        }

        // spawn a new falling tile
        self.falling = FallingTile::new(self.width);

        // if we are colliding before any update has occurured, we have lost
        for i in 0..self.falling.indexes.len() {
            if self.is_colliding(i) {
                self.reset();
            }
        }
    }

    fn is_colliding(&self, falling_index: usize) -> bool {
        let virtual_tile_index = self.falling.as_virtual(falling_index, self.width);
        
        if virtual_tile_index >= self.size {
            return true;
        }

        // arbitrary check to see if tile jumps to other side of board
        let virtual_col = (virtual_tile_index % self.width) as i32;
        let current_col = (self.falling.indexes[falling_index] % self.width) as i32;   
        if (virtual_col - current_col).wrapping_abs() > (self.width as i32).wrapping_div(2) {
            return true;
        } 

        if self.tiles[virtual_tile_index] != TileType::Empty {
            return true;
        }
        false
    }
}