---
navigation:
  title: Mit AE2
---

# Mit AE2

Ein Ofen ist eine Verarbeitungsmaschine für einen Musterlieferanten.

- Setz den Lieferanten auf den Ofen (eine Eingabeseite): er schiebt die Gegenstände des Musters hinein.
- Stell diese Seite auf Input/Output und schalte Auto Output ein: die Ergebnisse gehen in den Lieferanten zurück.
- Halte ihn befeuert: ein Trichter an einer Brennstoffseite oder [drahtlose Hitze](../energy/wireless.md).
- Eine Fabrik nimmt die Gegenstände in ihre Reihen; schalte [automatisches Aufteilen](../augments/auto_split.md) ein, damit alle arbeiten.
- Der Herstellungsstatus zeigt die [Restzeit](ae2_eta.md) nach dem echten Tempo des Ofens.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/ae2.snbt" />
</GameScene>
