---
navigation:
  title: Unobtainium Furnace
  parent: furnaces/tiers.md
  position: 120
  icon: unobtainium_furnace
item_ids:
- ironfurnaces:unobtainium_furnace
- ironfurnaces:upgrade_unobtainium
---

# Unobtainium Furnace

- Smelts an item in **0.05 s** (1 ticks); a vanilla furnace takes 10 s.
- Smelts up to **64** items at once.
- [Factory](../augments/factory.md): **6** rows.
- [Generator](../augments/generator.md): **5,000 RF/t**, buffer **1,000,000 RF**.
- Its recipes need the Allthemodium mod's metals; they show when it is installed.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/unobtainium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/unobtainium_furnace" />

## Upgrade

Sneak and right-click a <ItemLink id="vibranium_furnace" /> with <ItemLink id="upgrade_unobtainium" />.

<Recipe id="ironfurnaces:upgrades/upgrade_unobtainium" />
</IfMods>
