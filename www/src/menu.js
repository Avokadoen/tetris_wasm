import { getLocalStorage, rotateKey, downKey, leftKey, rightKey } from "./storage";

// ---------------------- MAIN MENU ---------------------------------------------
// TODO: this is the pause menu. main menu should be something else, but similiar
export const MainMenu = {
    rootElement: document.getElementById('mainMenu'),

    toggleSelf: (canvas) => {
        SettingsMenu.rootElement.style.display = 'none';
        if (MainMenu.rootElement.style.display !== 'none') {
            MainMenu.rootElement.style.display = 'none';
            canvas.style.display = 'block'
            return false;
        } else {
            MainMenu.rootElement.style.width    = canvas.width + 'px';
            MainMenu.rootElement.style.height   = canvas.height + 'px';
            MainMenu.rootElement.style.display  = 'block';
            canvas.style.display    = 'none';
            return true;
        }
    },

    // exported as we need external data
    addResumeClickCallback: (callback) => {
        const resumeButton = document.getElementById('resume');
        resumeButton.addEventListener('click', callback);
    },

    addResetClickCallback: (callback) => {
        const resetButton = document.getElementById('reset');
        resetButton.addEventListener('click', callback);
    },

    addSettingsClickCallback: (callback) => {
        const settingsButton = document.getElementById('settings');
        settingsButton.addEventListener('click', callback);
    }
}


// -------------------- SETTINGS --------------------------------------------------
// TODO: back button, player count setting, reset keybindings
export const SettingsMenu = {
    rootElement: document.getElementById('settingsMenu'),

    rotKeyElement: document.getElementById('rot_inp'),
    downKeyElement: document.getElementById('down_inp'),
    leftKeyElement: document.getElementById('left_stride_inp'),
    rightKeyElement: document.getElementById('right_stride_inp'),

    allKeys: [
        'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 
        'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
        'z', 'x', 'c', 'v', 'b', 'n', 'm', 
        'ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown',
        '1', '2', '3', '4', '6', '7', '8', '9'
    ],

    /* 
    *   Should only be called from this file
    */
    initialize: () => {
        const onSelectChange = (keyType, event) => {
            const myStorage = getLocalStorage();
            myStorage.setItem(keyType, event.originalTarget.value);
        };    

        SettingsMenu.rotKeyElement.addEventListener('change', event => onSelectChange(rotateKey, event));
        SettingsMenu.downKeyElement.addEventListener('change', event => onSelectChange(downKey, event));
        SettingsMenu.leftKeyElement.addEventListener('change', event => onSelectChange(leftKey, event));
        SettingsMenu.rightKeyElement.addEventListener('change', event => onSelectChange(rightKey, event));

        SettingsMenu.allKeys.forEach(k => {
            for (let i = 0; i < 4; i++) {
                const option = document.createElement('OPTION');
                option.value = k;
                option.text = k;

                switch(i) {
                    case 0:
                        SettingsMenu.rotKeyElement.add(option);
                        break;
                    case 1:
                        SettingsMenu.downKeyElement.add(option);
                        break;
                    case 2:
                        SettingsMenu.leftKeyElement.add(option);
                        break;
                    case 3:
                        SettingsMenu.rightKeyElement.add(option);
                        break;
                }
            }
        });
    },

    toggleSelf: (canvas) => {
        if (SettingsMenu.rootElement.style.display !== 'none') {
            SettingsMenu.rootElement.style.display = 'none';
            MainMenu.rootElement.style.display     = 'block';
        } else {
            MainMenu.rootElement.style.display      = 'none';
            SettingsMenu.rootElement.style.width    = canvas.width + 'px';
            SettingsMenu.rootElement.style.height   = canvas.height + 'px';
            SettingsMenu.rootElement.style.display  = 'block';
            SettingsMenu.setFormValues();
        }
    },

    setFormValues: () => {
        const myStorage = getLocalStorage();
    
        SettingsMenu.rotKeyElement.value = myStorage.getItem(rotateKey);
        SettingsMenu.downKeyElement.value = myStorage.getItem(downKey);
        SettingsMenu.leftKeyElement.value = myStorage.getItem(leftKey);
        SettingsMenu.rightKeyElement.value = myStorage.getItem(rightKey);
    },
} 

SettingsMenu.initialize();