---
navigation:
  title: Regenbogengenerator
---

# Regenbogengenerator

Ein [Regenbogenofen](../furnaces/rainbow.md) mit Generator-Erweiterung kann zusätzlich **50.000 RF/t** liefern. Dafür braucht er, alles vom selben Spieler gesetzt, der online ist:

- je einen Generator aus: Kupfer, Eisen, Silber, Gold, Diamant, Smaragd, Kristall, Obsidian, Netherite;
- jeder verbrennt Brennstoff und hat Platz im Speicher (leite seine Energie ab);
- keinen zweiten Regenbogengenerator dieses Spielers.

Die 50.000 RF/t gehen über seine Ausgabeseiten, an denen etwas Energie annimmt, aufgeteilt, nicht in den eigenen Speicher. Er leuchtet, solange er läuft.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/rainbow_generator.snbt" />
</GameScene>
