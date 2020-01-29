import { Board } from "tetris-wasm";
import { memory } from "tetris-wasm/tetris_wasm_bg";

const TILE_SIZE     = 15; // px
const GRID_COLOR    = "#CCCCCC";
const TILE_COLOR    = "#000000";

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

const drawTiles = () => {
    const tilesPtr = board.tiles_ptr();

    const tiles = new Uint16Array(memory.buffer, tilesPtr, board.tiles_len());
  
    ctx.beginPath();
    ctx.fillStyle = TILE_COLOR;

    const col = (t) => (t % width);
    const row = (t) => (t / width) | 0;

    tiles.forEach(t => 
        ctx.fillRect(
            col(t) * (TILE_SIZE + 1) + 1,
            row(t) * (TILE_SIZE + 1) + 1,
            TILE_SIZE,
            TILE_SIZE
        )
    );
   
    ctx.stroke();
};

let prevFrameTime = new Date().getTime();
let updateTimeCounter = 0;

const gameLoop = () => {

    let thisFrameTime = new Date().getTime();
    updateTimeCounter += thisFrameTime - prevFrameTime;

    if (updateTimeCounter > 100) {
        board.update();
        updateTimeCounter = 0;
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height); 
    drawGrid();
    drawTiles();

    requestAnimationFrame(gameLoop);

    prevFrameTime = thisFrameTime;
};

drawGrid();
drawTiles();
gameLoop();

