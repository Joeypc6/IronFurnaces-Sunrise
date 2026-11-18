---
navigation:
  title: Generator
  parent: augments/index.md
  position: 50
  icon: augment_generator
item_ids:
- ironfurnaces:augment_generator
---

# Generator

<ItemLink id="augment_generator" />: the furnace burns fuel into power and no longer smelts.

- Fuel: anything a furnace burns, 20 RF per tick of burn time (coal: 32,000 RF).
- A red augment changes the [fuel](../energy/fuels.md); the tier sets the [rate](../energy/output.md).
- Sends power out of its Output and Input/Output sides every tick; cables can also pull it.
- Stops burning while its buffer is full.
- A hopper on a Fuel side feeds it.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/generator_factory.snbt" />
</GameScene>

<Recipe id="ironfurnaces:augments/augment_generator" />
