mod utils; 
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

#[derive(PartialEq, Eq, Debug)]
pub enum TileVelocity {
    Nop,
    Strife(i16),
}

#[derive(Debug)]
pub struct FallingTile { 
    indexes: Vec<usize>,
    tile_type: TileType,
    velocity: TileVelocity,
}

impl FallingTile {
    pub fn new(board_width: usize) -> FallingTile {
        let indexes: Vec<usize> = vec![8, board_width + 8, board_width * 2 + 7, board_width * 2 + 8];

        FallingTile {
            indexes,
            tile_type: TileType::Black,
            velocity: TileVelocity::Nop
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Empty = 1,
    Black = 2
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
        for i in 0..self.height {
            let mut row_count = 0;
            for j in 0..self.width {
                if self.tiles[i * self.width + j] != TileType::Empty {
                    row_count += 1;
                }
            }

            if row_count >= self.width {
                for j in 0..self.width {
                    self.tiles[i * self.width + j] = TileType::Empty;
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

            // Handle strifing
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

    // TODO @bug: "Error: recursive use of an object detected which would lead to unsafe aliasing in rust"
    //              happens when you move to the most left tile at bottom
    pub fn move_left(&mut self) {
        self.falling.velocity = TileVelocity::Strife(-1);
    }

    pub fn move_rigth(&mut self) {
        self.falling.velocity = TileVelocity::Strife(1);
    }

}