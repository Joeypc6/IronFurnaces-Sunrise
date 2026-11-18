---
navigation:
  title: Entrada y salida automáticas
---

# Entrada y salida automáticas

Se activan en la [pestaña de ajustes](config_tab.md); no hacen falta tolvas.

- **Auto Input** saca del inventario junto a cada lado de entrada (lo que se funde) y de combustible (combustible).
- **Auto Output** mete los resultados en el inventario junto a cada lado de salida, y saca cubos vacíos por los de combustible.
- Actúan tras cada fundición, y cada 24 ticks en reposo.
- Una fábrica llena y vacía todas sus filas; un generador toma combustible por sus lados de combustible.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/auto_io.snbt" />
</GameScene>
