---
navigation:
  title: Iron Furnace
  parent: furnaces/tiers.md
  position: 20
  icon: iron_furnace
item_ids:
- ironfurnaces:iron_furnace
- ironfurnaces:upgrade_iron
- ironfurnaces:upgrade_iron2
---

# Iron Furnace

- Smelts an item in **8 s** (160 ticks); a vanilla furnace takes 10 s.
- [Factory](../augments/factory.md): **2** rows.
- [Generator](../augments/generator.md): **40 RF/t**, buffer **80,000 RF**.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/iron_furnace.snbt" />
</GameScene>

<Recipe id="ironfurnaces:furnaces/iron_furnace" />

<Recipe id="ironfurnaces:furnaces/iron_furnace2" />

## Upgrade

Sneak and right-click a <ItemLink id="minecraft:furnace" /> with <ItemLink id="upgrade_iron" />. Its items drop.

<Recipe id="ironfurnaces:upgrades/upgrade_iron" />

Sneak and right-click a <ItemLink id="copper_furnace" /> with <ItemLink id="upgrade_iron2" />.

<Recipe id="ironfurnaces:upgrades/upgrade_iron2" />
