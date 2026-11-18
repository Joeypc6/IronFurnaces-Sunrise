---
navigation:
  title: Vibranium Furnace
  parent: furnaces/tiers.md
  position: 110
  icon: vibranium_furnace
item_ids:
- ironfurnaces:vibranium_furnace
- ironfurnaces:upgrade_vibranium
---

# Vibranium Furnace

- Smelts an item in **0.15 s** (3 ticks); a vanilla furnace takes 10 s.
- Smelts up to **32** items at once.
- [Factory](../augments/factory.md): **6** rows.
- [Generator](../augments/generator.md): **3,000 RF/t**, buffer **1,000,000 RF**.
- Its recipes need the Allthemodium mod's metals; they show when it is installed.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/vibranium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/vibranium_furnace" />

## Upgrade

Sneak and right-click a <ItemLink id="allthemodium_furnace" /> with <ItemLink id="upgrade_vibranium" />.

<Recipe id="ironfurnaces:upgrades/upgrade_vibranium" />
</IfMods>
