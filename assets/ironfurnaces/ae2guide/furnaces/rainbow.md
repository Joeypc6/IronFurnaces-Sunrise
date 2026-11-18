---
navigation:
  title: Rainbow Furnace
  parent: furnaces/index.md
  position: 40
  icon: million_furnace
item_ids:
- ironfurnaces:million_furnace
- ironfurnaces:rainbow_core
- ironfurnaces:rainbow_plating
- ironfurnaces:rainbow_coal
---

# Rainbow Furnace

- Smelts up to **64** items at once, in **1 s** (20 ticks).
- [Factory](../augments/factory.md): **6** rows. [Generator](../augments/generator.md): **2,000 RF/t**, buffer **1,000,000 RF**.
- As a generator it can also run the [Rainbow generator](../energy/rainbow.md).
- An explosion destroys it and leaves a <ItemLink id="rainbow_coal" />.
- Rainbow Coal is fuel that stays: one item's smelt per use, 5,120 uses.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/million_furnace.snbt" />
</GameScene>

<Recipe id="ironfurnaces:rainbow_plating" />

<Recipe id="ironfurnaces:rainbow_core" />

<Recipe id="ironfurnaces:furnaces/million_furnace" />
