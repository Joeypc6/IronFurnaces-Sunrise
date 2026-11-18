---
navigation:
  title: Unobtainiumofen
---

# Unobtainiumofen

- Schmilzt einen Gegenstand in **0,05 s** (1 Ticks); ein normaler Ofen braucht 10 s.
- Schmilzt bis zu **64** Gegenstände zugleich.
- [Fabrik](../augments/factory.md): **6** Reihen.
- [Generator](../augments/generator.md): **5.000 RF/t**, Speicher **1.000.000 RF**.
- Seine Rezepte brauchen Metalle der Mod Allthemodium; sie erscheinen, wenn sie installiert ist.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/unobtainium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/unobtainium_furnace" />

## Aufwertung

Schleiche und klicke mit <ItemLink id="upgrade_unobtainium" /> rechts auf einen <ItemLink id="vibranium_furnace" />.

<Recipe id="ironfurnaces:upgrades/upgrade_unobtainium" />
</IfMods>
