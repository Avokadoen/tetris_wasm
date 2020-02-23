
import { Board, TileType } from 'tetris-wasm';
import { memory } from 'tetris-wasm/tetris_wasm_bg';
import { getLocalStorage, rotateKey, leftKey, downKey, rightKey } from './storage';
import { MainMenu, SettingsMenu } from './menu';

export const TILE_SIZE  = 20; // px
export const GRID_COLOR = "#CCCCCC";
export const TILE_COLOR = "#000000";

// Should be equal or less to 1
export const FALL_TO_MOVE_UPDATE_RATIO = 0.5;


export const BOARD  = Board.new();
export const WIDTH  = BOARD.width();
export const HEIGHT = BOARD.height();

export const CANVAS = document.getElementById("tetris-canvas");
export const CTX = CANVAS.getContext('2d');
export const SCORE_DISPLAY = document.getElementById("score_display");

CANVAS.width  = (TILE_SIZE + 1) * WIDTH + 1;
CANVAS.height = (TILE_SIZE + 1) * HEIGHT + 1;

export const Core = {
    prevFrameTime: new Date().getTime(),
    updateTimeCounter: 0,
    updateRate: 300,
    paused: false,

    initialize: () => {
        const toggleMenu = () => {
            Core.paused = MainMenu.toggleSelf(CANVAS);
        };
          
        const myStorage = getLocalStorage();
        
        // source: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
        document.addEventListener('keydown', (event) => {
            if (event.defaultPrevented) {
                return; // Do nothing if the event was already processed
            }
        
            switch (event.key) {
                case myStorage.getItem(rotateKey):
                BOARD.rotate();
                break;
                case myStorage.getItem(leftKey):
                BOARD.move_left();
                break;
                case myStorage.getItem(downKey):
                Core.updateRate = 50;
                break;
                case myStorage.getItem(rightKey):
                BOARD.move_rigth();
                break;
                case 'Escape':
                toggleMenu();
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
                Core.updateRate = 300;
                break;
                default:
                return; // Quit when this doesn't handle the key event.
            }
            // Cancel the default action to avoid it being handled twice
            event.preventDefault();
        }, true);
        
        MainMenu.addResumeClickCallback(() => {
            toggleMenu();
        });
        
        MainMenu.addResetClickCallback(() => {
            BOARD.reset();
            toggleMenu();
        });
        
        MainMenu.addSettingsClickCallback(() => {
            SettingsMenu.toggleSelf(CANVAS);
        });
        
        
        Core.drawGrid();
        Core.drawTiles();
        Core.gameLoop(); 
    },

    drawGrid: () => {
        CTX.beginPath();
        CTX.strokeStyle = GRID_COLOR;
    
        // Vertical lines.
        for (let i = 0; i <= WIDTH; i++) {
            const xPos = i * (TILE_SIZE + 1) + 1;
            CTX.moveTo(xPos, 0);
            CTX.lineTo(xPos, CANVAS.height);
        }
    
        // Horizontal lines.
        for (let i = 0; i <= HEIGHT; i++) {
            const yPos = i * (TILE_SIZE + 1) + 1; 
            CTX.moveTo(0,            yPos);
            CTX.lineTo(CANVAS.width, yPos);
        }
    
        CTX.stroke();
    },

    mapTileToColor: (u8Tile) => {
        switch(u8Tile) {
          case TileType.Empty:
            return "#FFFFFF";
          
          case TileType.Turquoise: 
            return "#40e0d0";
      
          case TileType.Blue: 
            return "#405be0";
      
          case TileType.Orange: 
            return "#f69114";
      
          case TileType.Yellow: 
            return "#e8ea19";
      
          case TileType.Green: 
            return "#01db10";
      
          case TileType.Purple: 
            return "#ad01db";
      
          case TileType.Red: 
            return "#920000";
      
          default: 
            console.error("Invalid tile type: ", u8Tile);
            return "#000000"
        }
    },

    getIndex: (row, column) => {
        return row * WIDTH + column;
    },
      
    drawTiles: () => {
        const tilesPtr = BOARD.tiles_ptr();
        
        const tiles = new Uint8Array(memory.buffer, tilesPtr, BOARD.tiles_len());
        
        CTX.beginPath();
        
        CTX.fillStyle = TILE_COLOR;
        
        for (let row = 0; row < HEIGHT; row++) {
            for (let col = 0; col < WIDTH; col++) {
            const index = Core.getIndex(row, col);
            CTX.fillStyle = Core.mapTileToColor(tiles[index]);
        
            CTX.fillRect(
                col * (TILE_SIZE + 1) + 1,
                row * (TILE_SIZE + 1) + 1,
                TILE_SIZE,
                TILE_SIZE);
            }
        }
        
        CTX.stroke();
    },

    gameLoop: (_) => {
        if (Core.paused) {
            Core.prevFrameTime = new Date().getTime();
            requestAnimationFrame(Core.gameLoop);
            return;
        }
      
        let thisFrameTime = new Date().getTime();
        Core.updateTimeCounter += thisFrameTime - Core.prevFrameTime;
      
        if (Core.updateTimeCounter > Core.updateRate * FALL_TO_MOVE_UPDATE_RATIO) {
            BOARD.update_rotate_stride();
        }
      
        if (Core.updateTimeCounter > Core.updateRate) {
            BOARD.update_fall();
            Core.updateTimeCounter = 0;
        }
      
        // TODO @refactor: should only happend when score changes
        SCORE_DISPLAY.innerText = "Score: " + BOARD.score();
      
        CTX.clearRect(0, 0, CANVAS.width, CANVAS.height); 
        Core.drawGrid();
        Core.drawTiles();
      
        requestAnimationFrame(Core.gameLoop);
      
        Core.prevFrameTime = thisFrameTime;
    },
}