---
navigation:
  title: Horno de unobtainium
---

# Horno de unobtainium

- Funde un objeto en **0,05 s** (1 ticks); un horno normal tarda 10 s.
- Funde hasta **64** objetos a la vez.
- [Fábrica](../augments/factory.md): **6** filas.
- [Generador](../augments/generator.md): **5.000 RF/t**, reserva de **1.000.000 RF**.
- Sus recetas piden metales del mod Allthemodium; se muestran si está instalado.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/unobtainium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/unobtainium_furnace" />

## Mejora

Agáchate y haz clic derecho en un <ItemLink id="vibranium_furnace" /> con <ItemLink id="upgrade_unobtainium" />.

<Recipe id="ironfurnaces:upgrades/upgrade_unobtainium" />
</IfMods>
