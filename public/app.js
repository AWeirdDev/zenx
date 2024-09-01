'use strict';
const __vault = {};
const socket = new WebSocket(`ws://${window.location.host}/ws`);

let heartbeat;

socket.addEventListener('open', () => {
    console.log('Connected to the WebSocket server');
    socket.send(JSON.stringify({ d: 'ping' }));
    document.getElementById('__venx_loader').setAttribute('data-ok', 'true');

    heartbeat = setInterval(() => {
        socket.send(JSON.stringify({ d: 'ping' }));
    }, 5000)
});

socket.addEventListener('message', async (event) => {
    // event.data: Blob
    const data_blob = await event.data.text();
    const { 'd': payload } = JSON.parse(data_blob);

    console.log(payload);
    if (payload === 'pong') {
        console.log('[zenx] heartbeat ok');
    }
    else if ("render" in payload) {
        console.log('[zenx] render');
        document.getElementById('app').style.opacity = '1';
        socket.send(JSON.stringify({
            d: {
                rendered: render(payload.render)
            }
        }));
    }
});

socket.addEventListener('close', () => {
    console.log('[zenx] disconnected.');
});

socket.addEventListener('error', (error) => {
    console.error('[zenx] ws error:', error);
});


// utils

function render({ components }) {
    let vaults = [];

    components.forEach(item => {
        const { id_selector, text_content, tag } = item;

        if (id_selector.startsWith("def:")) {
            let ele = edit({ parent: document.getElementById(id_selector.split(':', 2)[1]), textContent: text_content, tag });
            let vaultIdentifier = addToVault(ele);
            vaults.push(vaultIdentifier);
        }
    })

    return vaults;
}

/*
 * @param {HTMLElement} parent - parent element
*/
function edit({ parent, textContent, tag }) {
    let ele = document.createElement(tag);
    ele.textContent = textContent;
    return parent.appendChild(ele);
}

function uuid() {
    var d = Date.now();
    if (typeof performance !== 'undefined' && typeof performance.now === 'function') {
        d += performance.now();
    }
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
        var r = (d + Math.random() * 16) % 16 | 0;
        d = Math.floor(d / 16);
        return (c === 'x' ? r : (r & 0x3 | 0x8)).toString(16);
    });
}

/*
 * @param {HTMLElement} element - element
 * @returns {string} - uuid
*/
function addToVault(element) {
    let me = uuid();
    __vault[me] = element;
    return me;
}
