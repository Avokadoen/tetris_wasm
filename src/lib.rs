mod utils; 
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

// TODO @bug: floating tile collision on strife
// TODO @bug: No collision on rotate leads to bugs
// TODO @bug: Sometimes completing a line leads to error in console and freeze of game execution

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

#[derive(PartialEq, Eq, Debug)]
pub enum TileVelocity {
    Nop,
    Strife(i16),
}

#[derive(Debug)]
pub struct FallingTile {
    indexes: Vec<usize>,
    center: usize,
    tile_type: TileType,
    velocity: TileVelocity,
}

impl FallingTile {
    pub fn new(board_width: usize) -> FallingTile {
        let mut rng = thread_rng();
        let selected: u32 = rng.gen_range(1, 8);

        match selected {
            1 => return FallingTile::variation_1(board_width),
            2 => return FallingTile::variation_2(board_width),
            3 => return FallingTile::variation_3(board_width),
            4 => return FallingTile::variation_4(board_width),
            5 => return FallingTile::variation_5(board_width),
            6 => return FallingTile::variation_6(board_width),
            7 => return FallingTile::variation_7(board_width),
            _ => panic!("unexpected random value")
        }
        
    }

    fn rotate(&mut self, degree: f64, board_width: usize) {
        let cos = degree.to_radians().cos();
        let sin = degree.to_radians().sin();
        for i in 0..self.indexes.len() {
            let x = (self.indexes[i] % board_width) as f64 - (self.center % board_width) as f64;
            let y = (self.indexes[i] / board_width) as f64 - (self.center / board_width) as f64;

            let rot_x = (x * cos) - (y * sin);
            let rot_y = (x * sin) + (y * cos);

            let y_change = rot_y.round() * board_width as f64; 
            self.indexes[i] = (self.center as f64 + y_change + rot_x.round()) as usize; 
        }
    }

    #[inline]
    pub fn variation_1(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![8, board_width + 8, board_width * 2 + 8, board_width * 3 + 8],
            center: board_width + 8,
            tile_type: TileType::Turquoise,
            velocity: TileVelocity::Nop
        }
    }

    #[inline]
    pub fn variation_2(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![8, board_width + 8, board_width * 2 + 7, board_width * 2 + 8],
            center: board_width + 8,
            tile_type: TileType::Blue,
            velocity: TileVelocity::Nop
        }
    }

    #[inline]
    pub fn variation_3(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![8, board_width + 8, board_width * 2 + 8, board_width * 2 + 9],
            center: board_width + 8,
            tile_type: TileType::Orange,
            velocity: TileVelocity::Nop
        }
    }

    #[inline]
    pub fn variation_4(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![8, 9, board_width + 8, board_width + 9],
            center: board_width + 8,
            tile_type: TileType::Yellow,
            velocity: TileVelocity::Nop
        }
    }

    #[inline]
    pub fn variation_5(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![8, 9, board_width + 8, board_width + 7],
            center: board_width + 8,
            tile_type: TileType::Green,
            velocity: TileVelocity::Nop
        }
    }

    #[inline]
    pub fn variation_6(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![8, board_width + 7, board_width + 8, board_width + 9],
            center: board_width + 8,
            tile_type: TileType::Purple,
            velocity: TileVelocity::Nop
        }
    }

    #[inline]
    pub fn variation_7(board_width: usize) -> FallingTile {
        FallingTile {
            indexes: vec![7, 8, board_width + 8, board_width + 9],
            center: 8,
            tile_type: TileType::Purple,
            velocity: TileVelocity::Nop
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
    // indexes that are in the falling tile
    falling: FallingTile, 
    tiles: Vec<TileType>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        utils::set_panic_hook();

        let width: usize = 16;
        let height: usize = 32;
        let size = width * height;

        Board {
            width,
            height,
            size,
            falling: FallingTile::new(width),
            tiles: vec![TileType::Empty; size],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tiles_ptr(&self) -> *const TileType {
        self.tiles.as_ptr()
    }

    pub fn move_left(&mut self) {
        self.falling.velocity = TileVelocity::Strife(-1);
    }

    pub fn move_rigth(&mut self) {
        self.falling.velocity = TileVelocity::Strife(1);
    }

    pub fn rotate_left(&mut self) {
        for i in &self.falling.indexes {
            self.tiles[*i] = TileType::Empty;
        }

        self.falling.rotate(-90.0, self.width);
    }

    pub fn rotate_right(&mut self) {
        for i in &self.falling.indexes {
            self.tiles[*i] = TileType::Empty;
        }

        self.falling.rotate(90.0, self.width);
    }

    pub fn tiles_len(&self) -> usize {
        self.tiles.len()
    }

    pub fn update(&mut self) {
        match self.collide_test() {
            CollisionEvent::Bottom => self.on_new_tile(),
            CollisionEvent::Side => self.move_falling_tiles(CollisionEvent::Side),
            CollisionEvent::Nop => self.move_falling_tiles(CollisionEvent::Nop),
        }
        
        self.falling.velocity = TileVelocity::Nop;
    }

    fn on_new_tile(&mut self) {
        // Scan board for complete lines
        for i in (0..self.height).rev() {
            let mut row_count = 0;
            for j in 0..self.width {
                if self.tiles[(i * self.width) + j] != TileType::Empty {
                    row_count += 1;
                }
            }

            if row_count >= self.width {
                for j in 0..self.width {
                    self.tiles[i * self.width + j] = TileType::Empty;
                }

                'fall_board: for j in (0..i + 1).rev() {
                    for k in 0..self.width {
                        let tile_under_index = ((j + 1) * self.width) + k; 

                        let tile_over_index = j * self.width + k;
                        if self.tiles[tile_over_index] != TileType::Empty && self.tiles[tile_under_index] == TileType::Empty {
                            self.tiles[tile_under_index] = self.tiles[j * self.width + k];
                            self.tiles[tile_over_index] = TileType::Empty;
                        }
                    }
                }
            }
        }

        // spawn a new falling tile
        self.falling = FallingTile::new(self.width);
    }

    fn move_falling_tiles(&mut self, collision_state: CollisionEvent) {
        'move_tile: for i in 0..self.falling.indexes.len() as usize {
            let same_value_count = self.falling.indexes.iter()
                                                        .filter(|&index| *index == self.falling.indexes[i])
                                                        .count();

            if same_value_count <= 1 {
                self.tiles[self.falling.indexes[i]] = TileType::Empty;
            }

            // // Handle strifing
            if collision_state != CollisionEvent::Side {
                match self.falling.velocity {
                    TileVelocity::Strife(vel) => {
                        // TODO @bug: dangerous casting
                        self.falling.indexes[i] = (self.falling.indexes[i] as i16 + vel) as usize; 
                    },
                    _ => (),
                }
                
            }

            self.falling.indexes[i] += self.width;
             self.tiles[self.falling.indexes[i]] = self.falling.tile_type;
        }

        // TODO: move move logic into function to handle any tile. also logic in FallingTile
        if collision_state != CollisionEvent::Side {
            match self.falling.velocity {
                TileVelocity::Strife(vel) => {
                    self.falling.center = (self.falling.center as i16 + vel) as usize; 
                },
                _ => (),
            }
            
        }

        self.falling.center += self.width;
    }

    fn collide_test(&self) -> CollisionEvent {
        'collide_test: for i in 0..self.falling.indexes.len() as usize {
            if self.falling.indexes[i] + self.width > self.size {
                return CollisionEvent::Bottom;
            }
    
            let mut moved_index = self.falling.indexes[i];
            match self.falling.velocity {
                TileVelocity::Strife(vel) => {
                    let moved_index_signed = moved_index as i16 + vel;
                    if moved_index_signed < 0 {
                        return CollisionEvent::Side;
                    }
    
                    // TODO @bug: dangerous casting?
                        // maybe assert within usize range
                    moved_index = moved_index_signed as usize;
                    
                    
                    // TODO: @bug: rounding errors on divide?
                    if moved_index > self.size
                    || (moved_index / self.width) != (self.falling.indexes[i] / self.width) {
                        return CollisionEvent::Side;
                    }

                    let same_value_count = self.falling.indexes.iter()
                                        .filter(|&index| *index == moved_index)
                                        .count();


                    if same_value_count <= 0 && self.tiles[moved_index] != TileType::Empty {
                        return CollisionEvent::Side;
                    }
    
                    CollisionEvent::Nop
                }
                TileVelocity::Nop => CollisionEvent::Nop
            };
    
            moved_index += self.width;
            
            if moved_index > self.size - 1 
            || self.tiles[moved_index] != TileType::Empty && !self.falling.indexes.contains(&moved_index) {
                return CollisionEvent::Bottom;
            }
        }
        CollisionEvent::Nop
    }
}