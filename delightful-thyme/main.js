import "./style.css";
import { VirtualJoystick } from "./virtualjoystick";

globalThis.gameStats = {
  playerName: "???",
};

function shiftbg() {
  const bodyElement = document.getElementById("input-window");
  if (bodyElement) {
    bodyElement.style.backgroundPositionX = -Math.random() * 8000 + "px";
    bodyElement.style.backgroundPositionY = -Math.random() * 8000 + "px";
    window.setTimeout(shiftbg, Math.random() * 5000);
  }
}
window.setTimeout(shiftbg, Math.random() * 5000);

document.getElementById('fullscreen-controls').addEventListener('click', () => {
    if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen();
    } else {
        document.exitFullscreen();
    }
});

function triggerKey(id, key) {
  document.getElementById(id).addEventListener('click', () => {
    window.dispatchEvent(new KeyboardEvent('keydown', { code: key }));
  });
}

// triggerKey('gc-left', 'KeyH');
// triggerKey('gc-right', 'KeyL');
// triggerKey('gc-up', 'KeyK');
// triggerKey('gc-down', 'KeyJ');
triggerKey('gc-interact', 'Space');
triggerKey('gc-back', 'Backspace');
triggerKey('gc-inventory', 'KeyI');
triggerKey('gc-equipment', 'KeyE');
triggerKey('gc-drop', 'KeyD');
triggerKey('gc-scan', 'Enter');

new VirtualJoystick({
    mouseSupport	: true,
    limitStickTravel: true,
    stickRadius	: 50,
    container: document.getElementById('joystick-container'),
    strokeStyle: '#aaaaaa',
});

