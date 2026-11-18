---
navigation:
  title: Generator
---

# Generator

<ItemLink id="augment_generator" />: der Ofen verbrennt Brennstoff zu Energie und schmilzt nicht mehr.

- Brennstoff: alles, was im Ofen brennt, 20 RF pro Tick Brenndauer (Kohle: 32.000 RF).
- Eine rote Erweiterung ändert den [Brennstoff](../energy/fuels.md); die Stufe bestimmt die [Leistung](../energy/output.md).
- Gibt jeden Tick Energie über seine Ausgabe- und Ein/Ausgabe-Seiten ab; Kabel können sie auch abziehen.
- Brennt nicht, solange sein Speicher voll ist.
- Ein Trichter an einer Brennstoffseite versorgt ihn.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/generator_factory.snbt" />
</GameScene>

<Recipe id="ironfurnaces:augments/augment_generator" />
