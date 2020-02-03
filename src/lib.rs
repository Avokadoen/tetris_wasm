mod utils; 
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

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
    board_lines: Vec<u16>
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
        let board_lines = Vec::with_capacity(height as usize);

        Board {
            width,
            height,
            size,
            falling,
            tiles,
            board_lines
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
        'strife: for i in (self.falling.start_index..self.falling.end_index).rev() {
            let mut collide = false;

            if self.falling.velocity == TileVelocity::Left {
                let new_pos = self.tiles[i] - 1;
                collide = self.tiles[0..self.falling.start_index].contains(&new_pos);
                self.tiles[i] = new_pos;
            } else if self.falling.velocity == TileVelocity::Rigth {
                let new_pos = self.tiles[i] + 1;
                collide = self.tiles[0..self.falling.start_index].contains(&new_pos);
                self.tiles[i] = new_pos;
            }

            if collide {
                self.revert_strife(i);
                break 'strife;
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
                self.revert_fall(index);
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

    fn revert_fall(&mut self, event_index: usize) {
        for i in event_index..self.falling.end_index {
            self.tiles[i] -= self.width;
        }
    }

    fn scan_for_line(&mut self, freezed_tiles: FallingTiles) {
        for i in freezed_tiles.start_index..freezed_tiles.end_index  {
            let height_index = i / self.height as usize;
            self.board_lines[height_index] += 1;
            self.board_lines[height_index]
        }
    }

    fn revert_strife(&mut self, event_index: usize) {
        for i in event_index..self.falling.end_index {
            if self.falling.velocity == TileVelocity::Left {
                self.tiles[i] += 1;
            } else if self.falling.velocity == TileVelocity::Rigth {
                self.tiles[i] -= 1;
            }
        }
    }

    fn new_falling(&mut self) {
        let start_index = self.tiles.len();
        
        let mut new_tiles: Vec<u16> = vec![8, self.width + 8, self.width * 2 + 7, self.width * 2 + 8];
       
        self.falling = FallingTiles::new(start_index, start_index + new_tiles.len());
        
        self.tiles.append(&mut new_tiles); 
    }
}