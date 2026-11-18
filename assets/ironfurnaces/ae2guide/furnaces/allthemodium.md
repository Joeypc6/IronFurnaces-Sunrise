---
navigation:
  title: Allthemodium Furnace
  parent: furnaces/tiers.md
  position: 100
  icon: allthemodium_furnace
item_ids:
- ironfurnaces:allthemodium_furnace
- ironfurnaces:upgrade_allthemodium
---

# Allthemodium Furnace

- Smelts an item in **0.25 s** (5 ticks); a vanilla furnace takes 10 s.
- Smelts up to **16** items at once.
- [Factory](../augments/factory.md): **6** rows.
- [Generator](../augments/generator.md): **2,000 RF/t**, buffer **1,000,000 RF**.
- Its recipes need the Allthemodium mod's metals; they show when it is installed.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/allthemodium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/allthemodium_furnace" />

## Upgrade

Sneak and right-click a <ItemLink id="netherite_furnace" /> with <ItemLink id="upgrade_allthemodium" />.

<Recipe id="ironfurnaces:upgrades/upgrade_allthemodium" />
</IfMods>
