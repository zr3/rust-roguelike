import './style.css'

globalThis.gameStats = {
  playerName: '???',
};

function shiftbg() {
    const bodyElement = document.getElementById('input-window');
    if (bodyElement) {
        bodyElement.style.backgroundPositionX = -Math.random() * 8000 + "px";
        bodyElement.style.backgroundPositionY = -Math.random() * 8000 + "px";
        window.setTimeout(shiftbg, Math.random() * 5000);
    }
}
window.setTimeout(shiftbg, Math.random() * 5000);
const wildNameInput = document.getElementById('wild-name');
wildNameInput.addEventListener('input', (event) => {
    const wildName = document.getElementById('wild-name');
    document.getElementById('input-window-confirm').visibility = wildName.checkValidity() ? 'visible' : 'hidden';
});
wildNameInput.addEventListener('change', (event) => {
    console.log(`name is ${event.target.value}`);
    globalThis.gameStats.playerName = event.target.value;
    const inputWindow = document.getElementById('input-window');
    inputWindow.classList.add('out');
    inputWindow.querySelector('input').disabled = true;
    window.setTimeout(() => inputWindow.remove(), 2000);
});

wasm_bindgen("./wild-thyme/and_we_had_a_wild_thyme_bg.wasm");
