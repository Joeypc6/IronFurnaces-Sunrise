---
navigation:
  title: Con AE2
---

# Con AE2

Un horno sirve de máquina de procesado para un proveedor de patrones.

- Pon el proveedor encima del horno (un lado de entrada): mete los objetos del patrón.
- Pon ese lado en Input/Output y activa Auto Output: los resultados vuelven al proveedor.
- Dale combustible: una tolva en un lado de combustible, o [calor inalámbrico](../energy/wireless.md).
- Una fábrica recibe los objetos en sus filas; activa la [división automática](../augments/auto_split.md) para que trabajen todas.
- El estado de fabricación muestra el [tiempo restante](ae2_eta.md) según la velocidad real del horno.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/ae2.snbt" />
</GameScene>
