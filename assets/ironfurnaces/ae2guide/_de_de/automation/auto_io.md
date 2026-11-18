---
navigation:
  title: Automatische Ein- und Ausgabe
---

# Automatische Ein- und Ausgabe

Im [Einstellungsreiter](config_tab.md) einschalten; keine Trichter nötig.

- **Auto Input** holt aus dem Inventar neben jeder Eingabeseite (Schmelzgut) und jeder Brennstoffseite (Brennstoff).
- **Auto Output** schiebt Ergebnisse in das Inventar neben jeder Ausgabeseite und leere Eimer aus Brennstoffseiten.
- Sie arbeiten nach jedem Vorgang und alle 24 Ticks im Leerlauf.
- Eine Fabrik füllt und leert alle Reihen; ein Generator holt Brennstoff über seine Brennstoffseiten.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/auto_io.snbt" />
</GameScene>
