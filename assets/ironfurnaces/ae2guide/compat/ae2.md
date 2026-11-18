---
navigation:
  title: With AE2
  parent: index.md
  position: 90
  icon: ae2:pattern_provider
requires:
- ae2:pattern_provider
---

# With AE2

A furnace is a processing machine for a pattern provider.

- Put the provider on the furnace's top (an Input side): it pushes the pattern's items in.
- Set that side to Input/Output and turn on Auto Output: results go back into the provider.
- Keep it fuelled: a hopper on a Fuel side, or [wireless heat](../energy/wireless.md).
- A factory takes the items in its rows; turn on [Auto Split](../augments/auto_split.md) so every row works.
- The crafting status shows the [time left](ae2_eta.md) from the furnace's real speed.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/ae2.snbt" />
</GameScene>
