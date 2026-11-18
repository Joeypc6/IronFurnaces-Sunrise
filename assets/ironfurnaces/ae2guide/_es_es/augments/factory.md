---
navigation:
  title: Fábrica
---

# Fábrica

<ItemLink id="augment_factory" />: el horno funde en filas, con energía en vez de combustible.

- Filas: **2** (cobre, hierro), **4** (plata a esmeralda), **6** (cristal en adelante).
- Cada fila funde a la velocidad del horno.
- Energía: 20 RF por tick del tiempo de la receta, 4.000 RF por fundición normal; una fila solo empieza con esa energía guardada.
- Reserva: 80.000 / 200.000 / 1.000.000 RF, por los mismos niveles.
- Recibe energía por cualquier lado y no la da.
- [División automática](auto_split.md) reparte la entrada entre las filas.

La cara inferior de un generador es de Salida: ponlo encima y alimenta la fábrica.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/generator_factory.snbt" />
</GameScene>

<Recipe id="ironfurnaces:augments/augment_factory" />
