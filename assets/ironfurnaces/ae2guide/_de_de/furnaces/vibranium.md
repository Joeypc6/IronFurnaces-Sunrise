---
navigation:
  title: Vibraniumofen
---

# Vibraniumofen

- Schmilzt einen Gegenstand in **0,15 s** (3 Ticks); ein normaler Ofen braucht 10 s.
- Schmilzt bis zu **32** Gegenstände zugleich.
- [Fabrik](../augments/factory.md): **6** Reihen.
- [Generator](../augments/generator.md): **3.000 RF/t**, Speicher **1.000.000 RF**.
- Seine Rezepte brauchen Metalle der Mod Allthemodium; sie erscheinen, wenn sie installiert ist.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/vibranium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/vibranium_furnace" />

## Aufwertung

Schleiche und klicke mit <ItemLink id="upgrade_vibranium" /> rechts auf einen <ItemLink id="allthemodium_furnace" />.

<Recipe id="ironfurnaces:upgrades/upgrade_vibranium" />
</IfMods>
