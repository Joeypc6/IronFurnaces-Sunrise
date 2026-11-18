---
navigation:
  title: Allthemodiumofen
---

# Allthemodiumofen

- Schmilzt einen Gegenstand in **0,25 s** (5 Ticks); ein normaler Ofen braucht 10 s.
- Schmilzt bis zu **16** Gegenstände zugleich.
- [Fabrik](../augments/factory.md): **6** Reihen.
- [Generator](../augments/generator.md): **2.000 RF/t**, Speicher **1.000.000 RF**.
- Seine Rezepte brauchen Metalle der Mod Allthemodium; sie erscheinen, wenn sie installiert ist.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/allthemodium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/allthemodium_furnace" />

## Aufwertung

Schleiche und klicke mit <ItemLink id="upgrade_allthemodium" /> rechts auf einen <ItemLink id="netherite_furnace" />.

<Recipe id="ironfurnaces:upgrades/upgrade_allthemodium" />
</IfMods>
