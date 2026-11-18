---
navigation:
  title: Drahtlose Hitze
---

# Drahtlose Hitze

Öfen mit Energie statt Brennstoff betreiben.

1. Setze einen <ItemLink id="heater" /> und gib ihm Energie (er fasst 1.000.000 RF).
2. Leg einen <ItemLink id="item_heater" /> in seinen Platz: der Empfänger ist jetzt verbunden.
3. Leg den Empfänger in den Brennstoffplatz eines Ofens.

- Jeder geschmolzene Gegenstand kostet 2.000 RF (Tempo-Erweiterung: 4.000; Effizienz: 1.000).
- Ein Sender versorgt beliebig viele Empfänger in derselben Dimension.
- Nicht für Generatoren.
- Ein abgebauter Sender behält seine Energie.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/wireless.snbt" />
</GameScene>

<Recipe id="ironfurnaces:heater" />

<Recipe id="ironfurnaces:item_heater" />
