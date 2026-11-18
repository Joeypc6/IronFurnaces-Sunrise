---
navigation:
  title: Fabrik
---

# Fabrik

<ItemLink id="augment_factory" />: der Ofen schmilzt in Reihen, mit Energie statt Brennstoff.

- Reihen: **2** (Kupfer, Eisen), **4** (Silber bis Smaragd), **6** (ab Kristall).
- Jede Reihe schmilzt im Tempo des Ofens.
- Energie: 20 RF pro Tick der Rezeptzeit, 4.000 RF für einen normalen Vorgang; eine Reihe startet nur mit so viel Vorrat.
- Speicher: 80.000 / 200.000 / 1.000.000 RF, nach denselben Stufen.
- Nimmt Energie von jeder Seite, gibt keine ab.
- [Automatisches Aufteilen](auto_split.md) verteilt die Eingabe auf die Reihen.

Die Unterseite eines Generators ist eine Ausgabeseite: stell ihn obendrauf, und er versorgt die Fabrik.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/generator_factory.snbt" />
</GameScene>

<Recipe id="ironfurnaces:augments/augment_factory" />
