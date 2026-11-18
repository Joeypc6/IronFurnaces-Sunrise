---
navigation:
  title: Horno de allthemodium
---

# Horno de allthemodium

- Funde un objeto en **0,25 s** (5 ticks); un horno normal tarda 10 s.
- Funde hasta **16** objetos a la vez.
- [Fábrica](../augments/factory.md): **6** filas.
- [Generador](../augments/generator.md): **2.000 RF/t**, reserva de **1.000.000 RF**.
- Sus recetas piden metales del mod Allthemodium; se muestran si está instalado.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/allthemodium_furnace.snbt" />
</GameScene>

<IfMods items="allthemodium:allthemodium_ingot">
<Recipe id="ironfurnaces:furnaces/allthemodium_furnace" />

## Mejora

Agáchate y haz clic derecho en un <ItemLink id="netherite_furnace" /> con <ItemLink id="upgrade_allthemodium" />.

<Recipe id="ironfurnaces:upgrades/upgrade_allthemodium" />
</IfMods>
