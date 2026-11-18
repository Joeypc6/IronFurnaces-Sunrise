---
navigation:
  title: Seiten
---

# Seiten

Jede Seite hat eine dieser Einstellungen:

- **Input** (Eingabe): zu schmelzende Gegenstände hinein (auch Brennstoff).
- **Output** (Ausgabe): Ergebnisse heraus; ein Generator gibt hier Energie ab.
- **Input/Output**: beides.
- **Fuel Input/Eject**: Brennstoff hinein, leere Eimer heraus.
- **NONE**: geschlossen.

Neue Öfen: oben Eingabe, unten Ausgabe, der Rest Brennstoff, so arbeiten Trichter wie beim normalen Ofen. In einer Fabrik erreichen Ein- und Ausgabe alle Reihen; ein Generator nimmt Gegenstände nur über Brennstoffseiten.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/sides.snbt" />
</GameScene>
