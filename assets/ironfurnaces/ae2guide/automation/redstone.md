---
navigation:
  title: Redstone
  parent: automation/index.md
  position: 40
  icon: minecraft:comparator
---

# Redstone

Set in the [config tab](config_tab.md):

- **Redstone Ignored**: always runs.
- **Redstone Low**: runs only without a signal.
- **Redstone High**: runs only with a signal.

A comparator reads how full the furnace's slots are.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/redstone.snbt" />
</GameScene>
