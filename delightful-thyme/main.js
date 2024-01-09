import './style.css'

document.addEventListener('fx-warp', () => {
    // start displacement map animation (displacement wave)
    const newElement = document.createElementNS("http://www.w3.org/2000/svg", "animate");
    newElement.setAttribute("id", "treeportal-turbulence-anim");
    newElement.setAttribute("attributeName", "baseFrequency");
    newElement.setAttribute("from", "0.002 0.04");
    newElement.setAttribute("to", "0.002 0.05");
    newElement.setAttribute("dur", "1000ms");
    newElement.setAttribute("repeatCount", "indefinite");
    document.getElementById("treeportal-turbulence").appendChild(newElement);
    // start canvas filter animation (blur, hue shift)
    const canvas = document.getElementById("game-window");
    canvas.classList.add("portal");
    canvas.classList.add("warpoffset");
    window.setTimeout(() => {
        document.getElementById("game-window").classList.remove("portal");
    }, 800);
    window.setTimeout(() => {
        document.getElementById("treeportal-turbulence-anim").remove();
        document.getElementById("game-window").classList.remove("warpoffset");
    }, 1600);
    canvas.classList.add("portal");
    document.querySelector("body").classList.add("dim");
});
document.addEventListener('fx-nudge', () => {
    document.getElementById("game-window").classList.add("nudge");
    window.setTimeout(() => {
        document.getElementById("game-window").classList.remove("nudge");
    }, 400);
});
document.addEventListener('fx-update_stats', e => {
    const stats = e.detail;
    console.log(stats);
    if (stats.deepest_level % 10 > 0) {
        // normal level
        fetchNarration(stats, 'level');
    } else {
        // druid garden
        fetchNarration(stats, 'garden');
    }
});
document.addEventListener('fx-player_died', e => {
    const stats = e.detail;
    console.log(stats);
    fetchNarration(stats, 'dead');
});
document.addEventListener('fx-player_won', e => {
    const stats = e.detail;
    console.log(stats);
    fetchNarration(stats, 'baked');
});

function shiftbg() {
    const bodyElement = document.getElementById('input-window');
    if (bodyElement) {
        bodyElement.style.backgroundPositionX = -Math.random() * 8000 + "px";
        bodyElement.style.backgroundPositionY = -Math.random() * 8000 + "px";
        window.setTimeout(shiftbg, Math.random() * 5000);
    }
}
async function fetchNarration(stats, narrationType) {
    document.getElementById('top-text-element').innerText = '';
    try {
        const response = await fetch(`/api/wild-thyme/narration`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                narrationType,
                ...gameStats,
                ...stats
            }),
        });
        const result = await response.json();
        console.log("Success:", result);
        document.getElementById('top-text-element').innerText = result.narration;
    } catch (error) {
        console.error("Error:", error);
        document.getElementById('top-text-element').innerText = 'In a realm where the mists of time and magic intertwine, a veil of mystery descends upon the forest, obscuring the vision of even the most ancient observers. Beneath this enigmatic shroud, a seeker moves in silence, their path and challenges hidden from all eyes. The forest itself holds its breath, awaiting the revelations that will emerge when the fog lifts, revealing the unknown journey that unfolds within its heart.';
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
    gameStats.playerName = event.target.value;
    const inputWindow = document.getElementById('input-window');
    inputWindow.classList.add('out');
    inputWindow.querySelector('input').disabled = true;
    window.setTimeout(() => inputWindow.remove(), 2000);
});
