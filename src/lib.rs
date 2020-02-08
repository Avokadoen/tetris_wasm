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
        match self.move_falling_tiles() {
            // spawn new tile
            CollisionEvent::Bottom => self.spawn_new_tile(),
            _ => (),
        }
    }

    fn spawn_new_tile(&mut self) {
        self.falling = FallingTile::new(self.width);
    }

    fn move_falling_tiles(&mut self) -> CollisionEvent {
        // TODO @refactor: this code is ugly
        let mut return_event = CollisionEvent::Nop;
        'move_tile: for i in 0..self.falling.indexes.len() as usize {
            return_event = self.collide_test(i);
            if return_event == CollisionEvent::Bottom {
                break 'move_tile;
            }

            let same_value_count = self.falling.indexes.iter()
                                                        .filter(|&index| *index == self.falling.indexes[i])
                                                        .count();

            if same_value_count <= 1 {
                self.tiles[self.falling.indexes[i]] = TileType::Empty;
            }

            // Handle strifing
            if return_event != CollisionEvent::Side {
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

        self.falling.velocity = TileVelocity::Nop;
        return return_event;
    }

    // TODO @refactor: I'm guessing this will be wonky
    fn collide_test(&self, falling_index: usize) -> CollisionEvent {
        if self.falling.indexes[falling_index] + self.width > self.size {
            return CollisionEvent::Bottom;
        }

        return match self.falling.velocity {
            TileVelocity::Strife(vel) => {
                let mut event = CollisionEvent::Nop;

                let moved_index_signed = self.falling.indexes[falling_index] as i16 + vel;
                if moved_index_signed < 0 {
                    event = CollisionEvent::Side;
                }

                // TODO @bug: dangerous casting?
                let moved_index = moved_index_signed as usize;
                
                
                // TODO: @bug: rounding errors on divide?
                if moved_index > self.size
                || (moved_index / self.width) != (self.falling.indexes[falling_index] / self.width) {
                    event = CollisionEvent::Side;
                }

                if self.tiles[moved_index] != TileType::Empty && !self.falling.indexes.contains(&moved_index) {
                    event = CollisionEvent::Bottom;
                }

                event
            }
            TileVelocity::Nop => CollisionEvent::Nop
        }
    }

    pub fn move_left(&mut self) {
        self.falling.velocity = TileVelocity::Strife(-1);
    }

    pub fn move_rigth(&mut self) {
        self.falling.velocity = TileVelocity::Strife(1);
    }

}