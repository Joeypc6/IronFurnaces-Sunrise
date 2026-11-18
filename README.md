# Iron Furnaces for Sunrise

Iron Furnaces 4.4.2 (Qelifern, XenoMustache, Astryxion; Apache-2.0) as a native Sunrise mod: Hooks and Parts only, no
mixins and no `[[inject]]`. It was first written for the arismoko/horizon fork and moved to Sunrise on 2026-11-17; the
older Horizon mixin port is [IronFurnaces-Horizon](https://github.com/Joeypc6/IronFurnaces-Horizon).
Its logic was checked against the decompiled 4.4.2 jar. The textures, models,
recipes and language files are the original mod's.

Pinned to Sunrise Nightly 2026-11-17, commit `2d8126336e579777eabf9b2fd8b3d476236a04f2` (`sunrise-mod.toml`).
Optional dependencies: JEI (`^0.5`, the generator categories), GuideME (`^0.1`, the W guide), AE2 (`^15.4.13`,
`ae2::api::register_machine` for its crafting ETA) and Machine Work (`^0.1`, the shared `machine_work::MACHINE_WORK`
capability); the launcher turns on the Cargo feature named after each one when it is installed.

## Layout

- `sunrise-mod.toml`, `Cargo.toml`, `src/`: the mod.
  - `lib.rs`: hooks, Parts (furnaces, transmitter, upgrade and linker items, the transmitter item, menus), shared state.
  - `furnace.rs`: BlockIronFurnaceTileBase: furnace, factory and generator ticks, augments, auto input/output, redstone,
    experience, the Rainbow Furnace's generation, the transmitter's binding.
  - `caps.rs`: the furnaces' `ITEMS` and `ENERGY` capabilities (side settings, slot validity, energy by mode).
  - `interact.rs`: right clicks, augments, the looks, the Settings Linker, upgrades, placement facing.
  - `screen.rs`: the furnace, factory, generator and augment pages, the config tab, the transmitter's screen.
  - `recipes.rs`: cooking recipes from the world (`recipe_for`), the generator recipes, the slot-rule item sets.
  - `owners.rs`: PlayerFurnacesList as a player attachment. `tooltip.rs`: the items' tooltips. `client.rs`: animateTick
    (flames, smoke, sounds) and the comparator. `config.rs`: `config/ironfurnaces.toml` (the Java mod's keys).
    `jei.rs`: the JEI plugin. `quickmove.rs`: shift-clicks.
  - `work.rs`: the furnaces' `machine_work::MACHINE_WORK` answer (Machine Work library): `progress` is the cook under
    way (in factory mode the row closest to done, the cooking rows as batch), `pace` the furnace's own smelt time for an
    item (tier speed, speed and fuel augments, the red augment's recipe type) with the smelts at once as batch (the
    mult, or the factory rows with Auto Split). AE2's crafting ETA reads it first.
  - `ae2_eta.rs`: the same times through `ae2::api::register_machine`, for an AE2 without the library (kept for now).
- `assets/`, `data/` (content under `data/ironfurnaces/sunrise/`): generated from the jar by
  `tools/make_fork_content.py`. Run it after changing it.
- The in-game test scripts drive a local test rig and aren't published; what they check is listed below.

## Build and run

Copy the mod into the launcher's project folder and build it with the other ports:
`sunrise-launcher native-build --instance development --version nightly-2026-11-17-2d81263 --trust-native-code`,
then play the instance. `cargo test` runs the unit tests against the pinned SDK (`.sunrise/sdk`, fetched by
`cargo sunrise-mod`).

## On the Sunrise APIs

- Energy is Sunrise's `ENERGY` capability: the furnaces answer it themselves (before the game's own) so a factory
  takes energy and gives none, a generator gives and takes none, a furnace neither, as the Java tick sets its storage;
  generators push through their neighbours' `ENERGY` handlers in transactions. The transmitter is a `ModEnergyBuffer`.
- Items are Sunrise's `ITEMS` capability (WorldlyContainer rules: the slots each side's setting reaches, what fits
  there, what may come out); auto input and output move through the neighbours' `ITEMS` handlers in transactions.
- The player's furnace list is an attachment saved with the player; the menus are content menu types with a
  `MenuPart`'s slot rules; the screen is the block entity Part's screen hooks.
- The light: the blocks declare `dynamic_light`, and `light_emission` gives 0 while `misc.lightupdates` is on, as the
  Java mod; otherwise a lit furnace gives the data's `state_light` (14).
- Flames, smoke and their sounds come from the client block entity tick (`block_entity_client_tick`), at vanilla's
  random display tick rate: the chance a block within 16 and 32 blocks of the player is picked, as `animateTick`.

- The W guide is GuideME's (GuideME): the pages are in `assets/ironfurnaces/ae2guide` with their 3D scenes.
- The recipe viewer is JEI's ([JEI for Sunrise](https://github.com/Joeypc6/JEI-Sunrise), `jei::api`, `src/jei.rs`): the three generator categories (fuels, foods,
  `generator_blasting` recipes, each with its energy) and the catalysts (the furnaces and the factory augment for
  smelting, the blasting and smoking augments, the generator augment), as IronFurnacesJEIPlugin registers them.

Workarounds kept:
- **The screen's layout per page.** The screen hook is told only the block, not its block entity, and the layout is
  read once when the screen opens: the furnace clicked is remembered, and switching to and from the augment page opens
  the screen again (two ticks on, after the block entity's update) laid out for that page.
- **Shift-clicks (`src/quickmove.rs`).** quickMoveStack is made in `menu_quick_move` (the furnace's slots in the lent
  entity, the player's other slots through `ItemPlace`, all or nothing). The hook's `stack` is an `ItemStack`, which
  can't be empty, so when a whole stack moved the clicked slot gets a stand-in item (`ironfurnaces:moved`, in no tab
  and with no recipe), taken out at that player's inventory check of the same tick (`player_inventory_changed`, which
  also runs at a player's first check after joining). It can't reach the world or a container either: the first
  tick it spends in an inventory removes it (`inventory_tick`), a dropped one never joins the level
  (`entity_join_level`), it can't be tossed (`item_toss`) or picked up by a hopper (`hopper_pickup_item`), and no
  container menu is open at the same time as the furnace's (an in-game test checks quitting right after a whole-stack
  shift-click, a given, tossed and dropper-fired stand-in over a hopper and chest, and a LAN guest's shift-clicks and
  disconnect).
  Moves from the furnace's own slots are the game's routing (into the inventory, results from its end), as Java's.
- **The transmitter's energy on the furnace's side.** A furnace takes 2000 from its bound transmitter by writing the
  transmitter's saved `energy` (the Java mod's `removeEnergy`); its `ENERGY` handler gives none out, as in Java.
- **Slot rules from item sets.** `menu_slot_rules` answers from item sets worked out (on each side) when the data
  packs load: each cooking type's inputs, the fuels, the foods, the generator recipes. Since Sunrise's slot context
  (hook v4: the menu's own slots), the input slots check the furnace's own recipe type and the generator slot its
  fuel kind by the red augment in the menu's slot 3, as the Java slots read the block entity.
- **Taking a result** (SlotIronFurnace.onTake) is seen at the furnace's next tick: results that left other than
  through its item handler were taken by the player who has it open. The experience and recipes are given at the
  next server tick, because orbs made during a block entity's tick don't reach the world.
- **The comparator** reads a value kept from the last tick (`analog_output` has no access to the slots).
- **Experience from a broken furnace** is taken in `pre_remove_side_effects` and added in `block_drops` (orbs at the
  block, as Java's grantStoredRecipeExperience).
- **Writes to the held stack** (the Settings Linker's copy, an augment or upgrade used) go through the hand's inventory
  slot: through the hand, a creative player's stack would stay and a changed copy be added.

## What works

On Sunrise (2026-11-17, Nightly 2d81263): all 13 furnaces register and place; a Diamond Furnace smelts raw iron with
coal (7 ingots after a few seconds), shows flames and smoke at its front, plays the crackle sound and lights its
surroundings (13 next to it); the screens open. Tested on the fork before the move (in-game test scripts):

`ingame.sh`: all 13 furnaces and the transmitter place with their states; an Iron Furnace smelts raw iron with coal at
its tier's 160 ticks (lit while burning); the furnace screen's slots sit at the Java container's positions (56,17 /
56,53 / 116,35), the config tab opens, its buttons set the side and auto input settings, and the augment button
switches to the augment page (slots 26,35 / 80,35 / 134,35); a generator augment by right click turns a Gold Furnace
into a generator that burns coal into energy (its slot at 56,40), dropping its contents as Java; a factory augment
gives a Diamond (tier 1) factory its four rows at the Java positions; hoppers follow the sides; an upgrade turns a
vanilla furnace into an Iron Furnace facing the player, its contents dropped; the transmitter's screen (slot 80,37)
binds a receiver.

`ingame2.sh`: shift-clicks as Java's quickMoveStack (raw iron to the input, coal to the fuel, an augment from the
furnace page into its hidden slot, dirt from the hotbar to the main inventory; nothing left behind or doubled, the
client's screen matching); redstone Low and High; the comparator lighting a wire; 7 experience from taking ten iron
ingots, and 7 from breaking a furnace with them kept; the Settings Linker copying sides, auto IO and redstone onto
another furnace (also in creative); the Rainbow Furnace generating with one generator of each tier placed by its
owner, its energy charging a factory under it; the W guide (GuideME) over a furnace item, with its scenes; JEI's
generator categories (screenshots in `fork-game-ironfurnaces/shots2`).

`standin.sh`: the shift-click stand-in never stays in an inventory, the world, a hopper, a chest or a drop (host
quit-and-reload, a given one, a tossed one, a dropper firing one over a hopper, a LAN guest's shift-clicks and
disconnect).

## Not done yet

- **JEI cards' energy** is text under the card (`jei::api` has no tooltip over a drawn bar, which is where the Java
  card shows it), and the flame and energy bar are not animated.
- **Mod item components**: the Settings Linker's settings and the transmitter item's energy live in
  `minecraft:custom_data` (the Java mod registers its own component types).
- **Jade** needs nothing: the Jade port reads `ITEMS` and `ENERGY` itself.

## What differs from the Java mod on purpose

- None of the upstream port's differences remain: experience drops as orbs where the furnace stood or where the taker
  stands, and taking a result unlocks its recipes.
- As Java: upgrading an Iron Furnace doesn't carry its augment record, so a furnace with a blue augment drops its
  contents on the first tick after the upgrade; a blast leaves a Rainbow Furnace's Rainbow Coal, not its core.
