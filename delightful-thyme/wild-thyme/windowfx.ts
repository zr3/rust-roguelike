globalThis.windowfx = {
warp: function warp() {
  document.dispatchEvent(new CustomEvent("fx-warp"));
},
nudge: function nudge() {
  document.dispatchEvent(new CustomEvent("fx-nudge"));
},
update_stats: function update_stats(
  deepest_level,
  most_items_held,
  thyme_eaten,
  min_hp,
  mobs_killed,
  traps_triggered,
  portals_taken,
  steps_taken,
) {
  document.dispatchEvent(
    new CustomEvent("fx-update_stats", {
      detail: {
        deepest_level,
        most_items_held,
        thyme_eaten,
        min_hp,
        mobs_killed,
        traps_triggered,
        portals_taken,
        steps_taken,
      },
    }),
  );
},
player_died: function player_died(
  deepest_level,
  most_items_held,
  thyme_eaten,
  min_hp,
  mobs_killed,
  traps_triggered,
  portals_taken,
  steps_taken,
) {
  document.dispatchEvent(
    new CustomEvent("fx-player_died", {
      detail: {
        deepest_level,
        most_items_held,
        thyme_eaten,
        min_hp,
        mobs_killed,
        traps_triggered,
        portals_taken,
        steps_taken,
      },
    }),
  );
},
player_won: function player_won(
  deepest_level,
  most_items_held,
  thyme_eaten,
  min_hp,
  mobs_killed,
  traps_triggered,
  portals_taken,
  steps_taken,
  cake_description,
  cake_overall_points,
  cake_moist_points,
  cake_sweet_points,
  cake_style_points,
  cake_hot_points,
  cake_mold_points,
  cake_edible_points,
) {
  document.dispatchEvent(
    new CustomEvent("fx-player_won", {
      detail: {
        deepest_level,
        most_items_held,
        thyme_eaten,
        min_hp,
        mobs_killed,
        traps_triggered,
        portals_taken,
        steps_taken,
        cake_description,
        cake_overall_points,
        cake_moist_points,
        cake_sweet_points,
        cake_style_points,
        cake_hot_points,
        cake_mold_points,
        cake_edible_points,
      },
    }),
  );
}
}
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
