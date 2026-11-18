---
navigation:
  title: Rainbow generator
  parent: energy/index.md
  position: 30
  icon: rainbow_core
---

# Rainbow generator

A [Rainbow Furnace](../furnaces/rainbow.md) with a Generator augment can make **50,000 RF/t** more. It needs, all placed by the same player, who is online:

- one generator of each: Copper, Iron, Silver, Gold, Diamond, Emerald, Crystal, Obsidian, Netherite;
- each burning fuel, with room in its buffer (send its power somewhere);
- no second Rainbow generator of theirs.

The 50,000 RF/t goes out of its Output sides that touch something taking power, split between them, not into its own buffer. It glows while it runs.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/rainbow_generator.snbt" />
</GameScene>
