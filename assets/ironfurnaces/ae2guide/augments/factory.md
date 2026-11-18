---
navigation:
  title: Factory
  parent: augments/index.md
  position: 30
  icon: augment_factory
item_ids:
- ironfurnaces:augment_factory
---

# Factory

<ItemLink id="augment_factory" />: the furnace smelts in rows, on power instead of fuel.

- Rows: **2** (Copper, Iron), **4** (Silver to Emerald), **6** (Crystal and up).
- Each row smelts at the furnace's own speed.
- Power: 20 RF per tick of the recipe's time, 4,000 RF for a normal smelt; a row starts only with that much stored.
- Buffer: 80,000 / 200,000 / 1,000,000 RF, by the same tiers.
- Takes power from any side, gives none.
- [Auto Split](auto_split.md) shares the input over the rows.

A generator's bottom is an Output side: set one on top and it powers the factory.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/generator_factory.snbt" />
</GameScene>

<Recipe id="ironfurnaces:augments/augment_factory" />
