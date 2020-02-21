import { getLocalStorage, rotateKey, downKey, leftKey, rightKey } from "./storage";

// Main menu
const mainMenu = document.getElementById('mainMenu');
const settingsMenu = document.getElementById('settingsMenu');
export function toggleMainMenu(canvas) {
    settingsMenu.style.display = 'none';
    if (mainMenu.style.display !== 'none') {
        mainMenu.style.display = 'none';
        canvas.style.display = 'block'
        return false;
    } else {
        mainMenu.style.width    = canvas.width + 'px';
        mainMenu.style.height   = canvas.height + 'px';
        mainMenu.style.display  = 'block';
        canvas.style.display    = 'none';
        return true;
    }
}

export function toggleSettingsMenu(canvas) {
    if (settingsMenu.style.display !== 'none') {
        settingsMenu.style.display = 'none';
        mainMenu.style.display     = 'block';
    } else {
        mainMenu.style.display      = 'none';
        settingsMenu.style.width    = canvas.width + 'px';
        settingsMenu.style.height   = canvas.height + 'px';
        settingsMenu.style.display  = 'block';
        setFormValues();
    }
}

// exported as we need external data
export function addResumeClickCallback(callback) {
    const resumeButton = document.getElementById('resume');
    resumeButton.addEventListener('click', callback);
}

export function addResetClickCallback(callback) {
    const resetButton = document.getElementById('reset');
    resetButton.addEventListener('click', callback);
}

export function addSettingsClickCallback(callback) {
    const settingsButton = document.getElementById('settings');
    settingsButton.addEventListener('click', callback);
}

// Settings menu
const rotKeyElement = document.getElementById('rot_inp');
rotKeyElement.addEventListener('input', (event) => {
    if (event.inputType != 'insertText') {
        return;
    }
});

const downKeyElemnt = document.getElementById('down_inp');
const leftKeyElement = document.getElementById('left_stride_inp');
const rightKeyElement = document.getElementById('right_stride_inp');


function setFormValues() {
    const myStorage = getLocalStorage();

    rotKeyElement.value = myStorage.getItem(rotateKey);
    downKeyElemnt.value = myStorage.getItem(downKey);
    leftKeyElement.value = myStorage.getItem(leftKey);
    rightKeyElement.value = myStorage.getItem(rightKey);
}