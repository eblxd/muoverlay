const {
    app,
    BrowserWindow,
    powerSaveBlocker,
    Tray,
    Menu,
    ipcMain,
    nativeImage,
    Notification
} = require('electron');

const path = require('path');
const fs = require('fs');

const iconPath = path.join(__dirname, 'icon.png');

let win;
let tray;
let isQuitting = false;

function loadTrayImage() {
    if (fs.existsSync(iconPath)) {
        const img = nativeImage.createFromPath(iconPath);
        return img.isEmpty() ? nativeImage.createEmpty() : img.resize({ width: 16, height: 16 });
    }
    return nativeImage.createEmpty();
}

function createWindow() {
    win = new BrowserWindow({
        width: 460,
        height: 780,
        minWidth: 380,
        minHeight: 560,
        alwaysOnTop: true,
        frame: false,
        transparent: false,
        backgroundColor: '#04070d',
        autoHideMenuBar: true,
        resizable: true,
        skipTaskbar: false,
        icon: fs.existsSync(iconPath) ? iconPath : undefined,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false,
            backgroundThrottling: false
        }
    });

    win.loadFile('index.html');

    win.on('close', (event) => {
        if (!isQuitting) {
            event.preventDefault();
            win.hide();
        }
    });

    win.on('minimize', (event) => {
        event.preventDefault();
        win.hide();
    });
}

ipcMain.on('win-minimize', () => {
    if (win) win.hide();
});

ipcMain.on('win-close', () => {
    if (win) win.hide();
});

ipcMain.on('win-quit', () => {
    isQuitting = true;
    app.quit();
});

ipcMain.on('win-pin', (event, pinned) => {
    if (win) win.setAlwaysOnTop(!!pinned);
});

ipcMain.on('win-notify', (event, payload) => {
    try {
        const n = new Notification({
            title: payload.title || 'Norton Eventos',
            body: payload.body || '',
            icon: fs.existsSync(iconPath) ? iconPath : undefined,
            silent: !!payload.silent
        });
        n.on('click', () => {
            if (win) {
                win.show();
                win.focus();
            }
        });
        n.show();
    } catch (e) {}
});

app.whenReady().then(() => {
    powerSaveBlocker.start('prevent-app-suspension');

    createWindow();

    tray = new Tray(loadTrayImage());

    const contextMenu = Menu.buildFromTemplate([
        {
            label: 'Abrir Norton Eventos',
            click: () => {
                if (win) {
                    win.show();
                    win.focus();
                }
            }
        },
        {
            label: 'Manter no topo',
            type: 'checkbox',
            checked: true,
            click: (item) => {
                if (win) win.setAlwaysOnTop(item.checked);
                if (win) win.webContents.send('pin-changed', item.checked);
            }
        },
        { type: 'separator' },
        {
            label: 'Sair',
            click: () => {
                isQuitting = true;
                app.quit();
            }
        }
    ]);

    tray.setToolTip('Norton Eventos — MU Online');
    tray.setContextMenu(contextMenu);

    tray.on('click', () => {
        if (!win) return;
        if (win.isVisible() && win.isFocused()) {
            win.hide();
        } else {
            win.show();
            win.focus();
        }
    });

    tray.on('double-click', () => {
        if (!win) return;
        win.show();
        win.focus();
    });
});

app.on('window-all-closed', (event) => {
    event.preventDefault();
});

app.on('before-quit', () => {
    isQuitting = true;
});
