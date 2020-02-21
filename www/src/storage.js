let myStorage;

export const rotateKey = 'rotateKey';
export const leftKey = 'leftKey';
export const downKey = 'downKey';
export const rightKey = 'rightKey';

if (storageAvailable('localStorage')) {
    myStorage = window.localStorage;

    const setDefault = (keyName, keyValue) => {
        if (!myStorage.getItem(keyName)) {
            myStorage.setItem(keyName, keyValue);
        }
    } 

    setDefault(rotateKey, 'w');
    setDefault(leftKey, 'a');
    setDefault(downKey, 's');
    setDefault(rightKey, 'd');
} else {
    myStorage = {
        getItem: (keyName) => {
            switch(keyName) {
                case rotateKey:
                    return 'w';
                case leftKey:
                    return 'a';
                case downKey:
                    return 's';
                case rightKey:
                    return 'd';
            } 
        },
        setItem: (keyName, keyValue) => {},
    }
}

export function getLocalStorage() {
    return myStorage;
}


// source: https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API/Using_the_Web_Storage_API
function storageAvailable(type) {
    var storage;
    try {
        storage = window[type];
        var x = '__storage_test__';
        storage.setItem(x, x);
        storage.removeItem(x);
        return true;
    }
    catch(e) {
        return e instanceof DOMException && (
            // everything except Firefox
            e.code === 22 ||
            // Firefox
            e.code === 1014 ||
            // test name field too, because code might not be present
            // everything except Firefox
            e.name === 'QuotaExceededError' ||
            // Firefox
            e.name === 'NS_ERROR_DOM_QUOTA_REACHED') &&
            // acknowledge QuotaExceededError only if there's something already stored
            (storage && storage.length !== 0);
    }
}