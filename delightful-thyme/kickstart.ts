  var gameStats = {
      playerName: '???',
  };
  var windowfx = {
      warp: () => document.dispatchEvent(new CustomEvent("fx-warp")),
      nudge: () => document.dispatchEvent(new CustomEvent("fx-nudge")),
      update_stats: (
        deepest_level,
        most_items_held,
        thyme_eaten,
        min_hp,
        mobs_killed,
        traps_triggered,
        portals_taken,
        steps_taken,
      ) => document.dispatchEvent(new CustomEvent("fx-update_stats", {
              detail: {
                deepest_level,
                most_items_held,
                thyme_eaten,
                min_hp,
                mobs_killed,
                traps_triggered,
                portals_taken,
                steps_taken,
              }
          })),
      player_died: (
        deepest_level,
        most_items_held,
        thyme_eaten,
        min_hp,
        mobs_killed,
        traps_triggered,
        portals_taken,
        steps_taken,
      ) => document.dispatchEvent(new CustomEvent("fx-player_died", {
              detail: {
                deepest_level,
                most_items_held,
                thyme_eaten,
                min_hp,
                mobs_killed,
                traps_triggered,
                portals_taken,
                steps_taken,
              }
          })),
      player_won: (
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
      ) => document.dispatchEvent(new CustomEvent("fx-player_won", {
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
              }
          })),
  };
      window.addEventListener("load", async () => {
        await wasm_bindgen("./and_we_had_a_wild_thyme_bg.wasm");
      });
