import "./style.css";

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
