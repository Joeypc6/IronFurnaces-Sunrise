---
navigation:
  title: Horno de vibranium
---

# Horno de vibranium

- Funde un objeto en **0,15 s** (3 ticks); un horno normal tarda 10 s.
- Funde hasta **32** objetos a la vez.
- [Fábrica](../augments/factory.md): **6** filas.
- [Generador](../augments/generator.md): **3.000 RF/t**, reserva de **1.000.000 RF**.
- Sus recetas piden metales del mod Allthemodium; se muestran si está instalado.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/vibranium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/vibranium_furnace" />

## Mejora

Agáchate y haz clic derecho en un <ItemLink id="allthemodium_furnace" /> con <ItemLink id="upgrade_vibranium" />.

<Recipe id="ironfurnaces:upgrades/upgrade_vibranium" />
</IfMods>
