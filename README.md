# And We Had a Wild Thyme

smashed together learning project for me, based on Herbert Wolverson's book ( https://bfnightly.bracketproductions.com/ )

Rust, bracket-lib/rltk, specs

https://github.com/amethyst/rustrogueliketutorial/

## vision

a short, accessible roguelike that has an AI narrator

it's a traditional style roguelike, toned down and playable by people unfamiliar with the genre. the adventurer gives their name, starts with nothing, finds themself in a small forest town. the game is turn based, and based on player actions. the adventurer can MOVE, REST, USE ITEM, DROP ITEM, WEAR, TAKE OFF, BUMP. the world is filled with wild creatures and windy paths to explore. the goal is to win the GREAT BAKE-OFF in the starting town, but it will take an adventure and meeting a legend in the forest to get the ingredients!

at each level, the game uses an LLM to summarize what happened, and also to come up with some gags (like the wizard getting your name slightly wrong)

## ideas

- enemy called a REY that shoots a ranged attack light beam that makes you drop an item
- rogue blade vendor named PEPPERMINT WHOPPER MCGILLICUDY III
- wizard baker that obtains mysterious unique and rare foraged oddities, and creates psychadelic pastries that are like potions. lives in the woods for sure, in a tree stump, maybe foggy. nobody knows who he is, but folk lore tells legends of 'the wizard', he might be one but doesn't know if he is or not. he thinks he knows people bc he sacrificed some brain cells on mushrooms that might not be great for you, but always gets their name slightly wrong. very friendly and uncomfortably weird person
- for sure, for sure
- more focus on running/escaping, puzzle solving and creative item use than fighting

- mysterious fog on each level that moves and evolves and transports to the next level when you touch it
- finding water to stay alive, locating rivulets or rainwater and maybe purifying
- weather preparation
- ingredient spoilage
- avoid predators, traps 
- navigation, looking for landmarks
- foraging tools (basket for berries, sharp stick for digging roots)
- clearing path
- nighttime and light 
- forest crafting
- navigation + animal guide
- predators + protective charms
- foraging tools + environmental puzzles

- making a cake -- need flour, fat, eggs, sweet, milk, yeast, and can add an extra. these are categories of items, and the specific items have different buffs and debuffs for each judge
- judge's preferences revealed through world and npc comments
- public voting
- entire final baking scene is put together by an LLM based on events, with a default scene in case of error

## plan

✔️ m1: generic forest adventure with basic mechanics
- L1-5: herbivores, ghosts, spiders
- L6-10: REY, ghosts, wolves
- L11: the wizard

✔️ m2: town with npcs

  m3: ui, story and outer loop
- pass to fix colors and glyphs

  m4: narrator

  m5: website

