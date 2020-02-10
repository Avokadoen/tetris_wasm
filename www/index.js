import { Board, TileType } from "tetris-wasm";
import { memory } from "tetris-wasm/tetris_wasm_bg";

const TILE_SIZE     = 15; // px
const GRID_COLOR    = "#CCCCCC";
const TILE_COLOR    = "#000000";
const EMPTY_TILE_COLOR = "#FFFFFF";

const board     = Board.new();
const width     = board.width();
const height    = board.height();

const canvas = document.getElementById("tetris-canvas");

canvas.width = (TILE_SIZE + 1) * width + 1;
canvas.height = (TILE_SIZE + 1) * height + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
      const xPos = i * (TILE_SIZE + 1) + 1;
      ctx.moveTo(xPos, 0);
      ctx.lineTo(xPos, canvas.height);
  }

  // Horizontal lines.
  for (let i = 0; i <= height; i++) {
      const yPos = i * (TILE_SIZE + 1) + 1; 
      ctx.moveTo(0,            yPos);
      ctx.lineTo(canvas.width, yPos);
  }

  ctx.stroke();
};

const mapTileToColor = (u8Tile) => {
  switch(u8Tile) {
    // Empty
    case TileType.Empty:
      return "#FFFFFF";
    
    // Black
    case TileType.Black: 
      return "#000000";

    default: 
      console.error("Invalid tile type: ", u8Tile);
      return "#920000"
  }
}

const getIndex = (row, column) => {
  return row * width + column;
};

const drawTiles = () => {
  const tilesPtr = board.tiles_ptr();

  const tiles = new Uint8Array(memory.buffer, tilesPtr, board.tiles_len());

  ctx.beginPath();

  ctx.fillStyle = TILE_COLOR;
  
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col);
      ctx.fillStyle = mapTileToColor(tiles[index]);

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

let prevFrameTime = new Date().getTime();
let updateTimeCounter = 0;
let updateRate = 100;

const gameLoop = () => {
  let thisFrameTime = new Date().getTime();
  updateTimeCounter += thisFrameTime - prevFrameTime;

  if (updateTimeCounter > updateRate) {
      board.update();
      updateTimeCounter = 0;
  }

  ctx.clearRect(0, 0, canvas.width, canvas.height); 
  drawGrid();
  drawTiles();

  requestAnimationFrame(gameLoop);

  prevFrameTime = thisFrameTime;
};

// source: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
document.addEventListener('keydown', (event) => {
  if (event.defaultPrevented) {
    return; // Do nothing if the event was already processed
  }
  switch (event.key) {
    case 'a': 
      board.move_left();
      break;
    case 'd':
      board.move_rigth();
      break;
    case 's':
      updateRate = 50
      break;
    default:
      return; // Quit when this doesn't handle the key event.
  }

  // Cancel the default action to avoid it being handled twice
  event.preventDefault();
}, true);

document.addEventListener('keyup', (event) => {
  if (event.defaultPrevented) {
    return; // Do nothing if the event was already processed
  }
  switch (event.key) {
    case 's':
      updateRate = 100
      break;
    default:
      return; // Quit when this doesn't handle the key event.
  }
  // Cancel the default action to avoid it being handled twice
  event.preventDefault();
}, true);

drawGrid();
drawTiles();
gameLoop();

