import { Board } from "tetris-wasm";
import { memory } from "tetris-wasm/tetris_wasm_bg";

const TILE_SIZE     = 15; // px
const GRID_COLOR    = "#CCCCCC";
const TILE_COLOR    = "#FFFFFF";

const board     = Board.new();
const width     = board.width();
const height    = board.height();




const canvas = document.getElementById("tetris-canvas");
canvas.height = (TILE_SIZE + 1) * height + 1;
canvas.width = (TILE_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (TILE_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (TILE_SIZE + 1) + 1, (TILE_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (TILE_SIZE + 1) + 1);
      ctx.lineTo((TILE_SIZE + 1) * width + 1, j * (TILE_SIZE + 1) + 1);
    }
  
    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
  };

const drawTiles = () => {
    const tilePtr = board.active_tile_ptr();
    
    const restedPtr = board.rested_tile_ptr();

    const rested = new Uint8Array(memory.buffer, restedPtr, board.rested_tile_len);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);
  
        // TODO: includes is too slow
        ctx.fillStyle = rested.includes(idx)
          ? GRID_COLOR
          : TILE_COLOR;
  
        ctx.fillRect(
          col * (TILE_SIZE + 1) + 1,
          row * (TILE_SIZE + 1) + 1,
          TILE_SIZE,
          TILE_SIZE
        );
      }
    }
  
    ctx.stroke();
};

const renderLoop = () => {
  board.update();

  drawGrid();
  drawTiles();

  requestAnimationFrame(renderLoop);
};

drawGrid();
drawTiles();
renderLoop();

