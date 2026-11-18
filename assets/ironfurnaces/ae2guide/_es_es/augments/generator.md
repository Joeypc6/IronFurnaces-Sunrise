---
navigation:
  title: Generador
---

# Generador

<ItemLink id="augment_generator" />: el horno quema combustible para hacer energía y ya no funde.

- Combustible: todo lo que arde en un horno, 20 RF por tick de combustión (carbón: 32.000 RF).
- Un aumento rojo cambia el [combustible](../energy/fuels.md); el nivel marca el [ritmo](../energy/output.md).
- Envía energía por sus lados de Salida y de Entrada/Salida cada tick; los cables también pueden sacarla.
- Deja de quemar mientras su reserva está llena.
- Una tolva en un lado de Combustible lo alimenta.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/generator_factory.snbt" />
</GameScene>

<Recipe id="ironfurnaces:augments/augment_generator" />
