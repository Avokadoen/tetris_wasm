mod utils; 
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileSet {
    center_index: usize,
    tiles: [bool; 9],
}

impl TileSet {
    pub fn new() -> TileSet {
        
        // TODO: random post here
        let center_index = 16;

        // TODO: apply prototype pattern here
        let tiles = [false, false, true,
                     false, false, true,
                     true,  true,  true ];
        
        TileSet {
            center_index,
            tiles,
        }
    }

    // TODO: refactor this beast function
    pub fn get_lowest_indexes(&self) -> [i32; 3] {
        let mut rtr: [i32; 3] = [-1; 3];
        let mut index = 0;

        'check: for i in 1..3 {
            for j in 1..3 {
                if self.tiles[(i * j) - 1] == true {
                    rtr[index] = i as i32;
                    index += 1;
                }

                if index > 0 && j == 3 {
                    break 'check;
                }
            }
        }

        rtr
    }

    pub fn tile_to_grid_pos(&self, width: usize) -> Vec<usize> {
        let mut rtr = Vec::new();
        let top_left_index = self.center_index - width - 1;
        for i in 1..3 {
            for j in 1..3 { 
                if self.tiles[(i * j) - 1] {
                    rtr.push(top_left_index + (i - 1) + (j * width));
                }
            }
        }

        rtr
    }
}

#[wasm_bindgen]
pub struct Board {
    size: usize, // TODO: remember to set this
    width: usize,
    height: usize,
    active_tile: TileSet,
    rested_tiles: Vec<usize>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        let width = 16;
        let height = 32;
        let size = width * height;

        Board {
            width,
            height,
            size,
            active_tile: TileSet::new(),
            rested_tiles: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn active_tile_ptr(&self) -> *const TileSet {
        &self.active_tile
    }

    pub fn rested_tile_ptr(&self) -> *const usize {
        self.rested_tiles.as_ptr()
    }

    pub fn rested_tile_len(&self) -> usize { 
        self.rested_tiles.len()
    }

    pub fn update(&mut self) {
        let to_check = self.active_tile.get_lowest_indexes();

        let mut can_fall = false;
        'space_check: for i in 0..3 {
            if to_check[i] < 0 {
                continue;
            }

            // todo: assert this makes sense to_check[i] as usize
            let tile_index = self.active_tile.center_index + to_check[i] as usize;
            let tile_under = tile_index + self.width;

            if tile_under > self.size {
                break 'space_check;
            }

            if self.rested_tiles.contains(&tile_under) {
                break 'space_check;
            }

            can_fall = true;
        }

        if can_fall {
            self.active_tile.center_index += self.width; 
        } else {
            let top_left_index = self.active_tile.center_index - self.width - 1;
            // TODO: some assert for negative top_left_index
            for i in 1..3 {
                for j in 1..3 {
                    if self.active_tile.tiles[i * j - 1] {
                        self.rested_tiles.push(top_left_index + (i - 1) + (j * self.width))
                    }
                }
            }

            // TODO: create a new active tile
            self.active_tile.center_index = 0 
        }
    }
}