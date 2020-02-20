
const menu = document.getElementById('menu');

const tetrisCanvas = document.getElementById('tetris-canvas');

export function toggleMenu(width, height) {
    if (menu.style.display !== "none") {
        menu.style.display = "none";
    } else {
        menu.style.width = width + 'px';
        menu.style.height = height + 'px';
        menu.style.display = "block";
    }
}
