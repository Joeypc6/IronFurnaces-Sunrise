---
navigation:
  title: Lados
---

# Lados

Cada cara tiene uno de estos ajustes:

- **Input** (entrada): entran objetos a fundir (y combustible).
- **Output** (salida): salen los resultados; un generador envía energía por aquí.
- **Input/Output**: ambos.
- **Fuel Input/Eject**: entra combustible, salen cubos vacíos.
- **NONE**: cerrada.

Hornos nuevos: arriba entrada, abajo salida, el resto combustible, así las tolvas funcionan como en un horno normal. En una fábrica entrada y salida llegan a todas las filas; un generador solo acepta objetos por lados de combustible.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/sides.snbt" />
</GameScene>
