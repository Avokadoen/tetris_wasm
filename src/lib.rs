mod utils; 
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(PartialEq, Eq)]
pub enum TileVelocity {
    Left,
    Rigth,
    Nop,
}

pub struct FallingTiles {
    start_index: usize,
    end_index: usize,
    velocity: TileVelocity,
}

impl FallingTiles {
    pub fn new(start_index: usize, end_index: usize) -> FallingTiles {
        FallingTiles {
            start_index,
            end_index,
            velocity: TileVelocity::Nop
        }
    }
}

#[derive(PartialEq, Eq)]
enum FallStatus {
    Freeze(usize),
    Continue,
}

#[wasm_bindgen]
pub struct Board {
    width: u16,
    height: u16,
    size: u16,
    falling: FallingTiles,
    tiles: Vec<u16>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        utils::set_panic_hook();

        let width = 16;
        let height = 32;
        let size = width * height;

        let tiles: Vec<u16> = vec![8, width + 8, width * 2 + 7, width * 2 + 8];
        let falling = FallingTiles::new(0, tiles.len());

        Board {
            width,
            height,
            size,
            falling,
            tiles,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn tiles_ptr(&self) -> *const u16 {
        self.tiles.as_ptr()
    }

    pub fn tiles_len(&self) -> usize {
        self.tiles.len()
    }

    pub fn update(&mut self) {
        // TODO: 2 loops aren't optimal
        // TODO: collision to the sides
        'strife: for i in (self.falling.start_index..self.falling.end_index).rev() {
            if self.falling.velocity == TileVelocity::Left {
                self.tiles[i] -= 1;
            } else if self.falling.velocity == TileVelocity::Rigth {
                self.tiles[i] += 1;
            }
        }

        let mut fall_status = FallStatus::Continue;
        'fall: for i in (self.falling.start_index..self.falling.end_index).rev() {
            // if we reached bottom tiles
            if (self.tiles[i] + self.width) / self.width > (self.height - 1) {
                fall_status = FallStatus::Freeze(i + 1);
                break 'fall;
            }

            // if we hit another static tile
            for j in (0..self.falling.start_index).rev() {
                if self.tiles[j] == self.tiles[i] + self.width {
                    fall_status = FallStatus::Freeze(i + 1);
                    break 'fall;
                }
            } 
            
            self.tiles[i] += self.width; 
        }

        match fall_status {
            FallStatus::Freeze(index) => {
                self.revert_moves(index);
                self.new_falling();
            }
            FallStatus::Continue => (),
        }

        self.falling.velocity = TileVelocity::Nop;
    }

    pub fn move_left(&mut self) {
        self.falling.velocity = TileVelocity::Left;
    }

    pub fn move_rigth(&mut self) {
        self.falling.velocity = TileVelocity::Rigth;
    }

    fn revert_moves(&mut self, event_index: usize) {
        for i in event_index..self.falling.end_index {
            if self.falling.velocity == TileVelocity::Left {
                self.tiles[i] += 1;
            } else if self.falling.velocity == TileVelocity::Rigth {
                self.tiles[i] -= 1;
            }

            self.tiles[i] -= self.width;
        }
    }

    fn new_falling(&mut self) {
        let start_index = self.tiles.len();
        
        let mut new_tiles: Vec<u16> = vec![8, self.width + 8, self.width * 2 + 7, self.width * 2 + 8];
       
        self.falling = FallingTiles::new(start_index, start_index + new_tiles.len());
        
        self.tiles.append(&mut new_tiles); 
    }
}