---
navigation:
  title: Auto Input and Output
  parent: automation/index.md
  position: 30
  icon: minecraft:chest
---

# Auto Input and Output

Turn them on in the [config tab](config_tab.md); no hoppers needed.

- **Auto Input** pulls from the inventory beside each Input side (things to smelt) and each Fuel side (fuel).
- **Auto Output** pushes results into the inventory beside each Output side, and empty buckets out of Fuel sides.
- They run after each smelt, and every 24 ticks while idle.
- A factory fills and empties all its rows; a generator pulls fuel through its Fuel sides.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/auto_io.snbt" />
</GameScene>
