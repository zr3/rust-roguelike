globalThis.windowfx = {
  warp: function warp() {
    // start displacement map animation (displacement wave)
    const newElement = document.createElementNS(
      "http://www.w3.org/2000/svg",
      "animate",
    );
    newElement.setAttribute("id", "treeportal-turbulence-anim");
    newElement.setAttribute("attributeName", "baseFrequency");
    newElement.setAttribute("from", "0.002 0.04");
    newElement.setAttribute("to", "0.002 0.05");
    newElement.setAttribute("dur", "1000ms");
    newElement.setAttribute("repeatCount", "indefinite");
    document.getElementById("treeportal-turbulence")?.appendChild(newElement);
    // start canvas filter animation (blur, hue shift)
    const canvas = document.getElementById("game-window");
    canvas?.classList.add("portal");
    canvas?.classList.add("warpoffset");
    window.setTimeout(() => {
      document.getElementById("game-window")?.classList.remove("portal");
    }, 800);
    window.setTimeout(() => {
      document.getElementById("treeportal-turbulence-anim")?.remove();
      document.getElementById("game-window")?.classList.remove("warpoffset");
    }, 1600);
    canvas?.classList.add("portal");
    document.querySelector("body")?.classList.add("dim");
  },
  nudge: function nudge() {
    document.getElementById("game-window")?.classList.add("nudge");
    window.setTimeout(() => {
      document.getElementById("game-window")?.classList.remove("nudge");
    }, 400);
  },
  update_stats: function update_stats(
    deepest_level: number,
    most_items_held: number,
    thyme_eaten: number,
    min_hp: number,
    mobs_killed: number,
    traps_triggered: number,
    portals_taken: number,
    steps_taken: number,
    level_stats: any,
  ) {
    const stats = {
      deepest_level,
      most_items_held,
      thyme_eaten,
      min_hp,
      mobs_killed,
      traps_triggered,
      portals_taken,
      steps_taken,
      level_stats,
    };
    console.log(stats);
    if (stats.deepest_level % 10 > 0) {
      // normal level
      fetchNarration(stats, "level");
    } else {
      // druid garden
      fetchNarration(stats, "garden");
    }
  },
  player_died: function player_died(
    deepest_level: number,
    most_items_held: number,
    thyme_eaten: number,
    min_hp: number,
    mobs_killed: number,
    traps_triggered: number,
    portals_taken: number,
    steps_taken: number,
  ) {
    const stats = {
      deepest_level,
      most_items_held,
      thyme_eaten,
      min_hp,
      mobs_killed,
      traps_triggered,
      portals_taken,
      steps_taken,
    };
    console.log(stats);
    fetchNarration(stats, "dead");
  },
  player_won: function player_won(
    deepest_level: number,
    most_items_held: number,
    thyme_eaten: number,
    min_hp: number,
    mobs_killed: number,
    traps_triggered: number,
    portals_taken: number,
    steps_taken: number,
    cake_description: string,
    cake_overall_points: number,
    cake_moist_points: number,
    cake_sweet_points: number,
    cake_style_points: number,
    cake_hot_points: number,
    cake_mold_points: number,
    cake_edible_points: number,
  ) {
    const stats = {
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
    };
    console.log(stats);
    fetchNarration(stats, "baked");
  },
};
type NarrationType = "garden" | "baked" | "dead" | "level";
async function fetchNarration(
  stats: any,
  narrationType: NarrationType,
): Promise<void> {
  try {
    const response = await fetch(`/api/wild-thyme/narration`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        narrationType,
        ...globalThis.gameStats,
        ...stats,
      }),
    });
    const result = await response.json();
    console.log("Success:", result);
    spellOutText(result.narration);
    // document.getElementById('top-text-element').innerText = result.narration;
  } catch (error) {
    console.error("Error:", error);
    spellOutText(
      "In a realm where the mists of time and magic intertwine, a veil of mystery descends upon the forest, obscuring the vision of even the most ancient observers. Beneath this enigmatic shroud, a seeker moves in silence, their path and challenges hidden from all eyes. The forest itself holds its breath, awaiting the revelations that will emerge when the fog lifts, revealing the unknown journey that unfolds within its heart.",
    );
  }
}

async function spellOutText(text: string) {
  // clear existing
  const topText = document.getElementById("top-text");
  if (!topText) {
    console.error("#top-text must exist to spell out text");
    return;
  }
  topText.innerHTML = "";
  // split into 30-char wide lines
  const lines = greedyLineBreak(30, text);
  for (let line of lines) {
    topText.innerHTML += `<p class="intro-text type-animation">${line}</p>`;
    await delay(1100);
    topText.lastElementChild?.classList.remove("type-animation");
  }
}
function greedyLineBreak(lineLength: number, text: string) {
  // greedy algorithm to find line break positions
  const words = text.split(" ");
  let currentLength = 0;
  let startLine = 0;
  const lines: string[] = [];
  for (let wx = 0; wx < words.length; wx++) {
    const wordLength = words[wx].length;
    if (wordLength + currentLength >= lineLength) {
      currentLength = 0;
      lines.push(words.slice(startLine, wx).join(" "));
      startLine = wx;
    }
    currentLength += wordLength + 1;
  }
  lines.push(words.slice(startLine).join(" "));
  return lines;
}
function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function intro() {
  await spellOutText(
    "Welcome to the forest, friend. I know you want to bake, but I do not know your name.",
  );
  const topText = document.getElementById("top-text");
  if (!topText) {
    return;
  }
  topText.innerHTML += `
      <div id="wild-name-box">
          <label for="wild-name">Who are you?</label>
          <input
            id="wild-name"
            type="text"
            autofocus
            pattern="[\\w\\s]+"
            name="wild-name"
            required=""
            minlength="1"
            maxlength="10"
          />
        </div>
`;
  const wildNameInput = document.getElementById("wild-name");
  wildNameInput?.addEventListener("change", (event) => {
    console.log(`name is ${event.target.value}`);
    globalThis.gameStats.playerName = event.target.value;
    startGame();
    tutorialText();
  });
}
function startGame() {
    const inputWindow = document.getElementById("input-window");
    inputWindow?.classList.add("out");
    document.getElementById("wild-name-box")?.remove();
    document.querySelectorAll(".intro-text").forEach((e) => e.remove());
    window.setTimeout(() => inputWindow?.remove(), 2000);
    wasm_bindgen("./wasm/and_we_had_a_wild_thyme_bg.wasm");
}
async function tutorialText() {
    await spellOutText("Wow, ok. Hopefully I can remember that! Anyway, you are the @, and time only moves when you move. ENTER to see what the symbols on-screen are. SPACE to interact with things, and more controls below. DYING IS NORMAL! This world is about exploration and experimentation, and there are plenty of items to find, so don't be afraid to use them!");
    document.getElementById("bottom-text")?.classList.remove('hidden');
}
if (window.location.search.includes('debug')) {
    startGame();
} else {
    intro();
}
