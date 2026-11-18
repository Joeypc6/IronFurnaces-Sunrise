---
navigation:
  title: Calor inalámbrico
---

# Calor inalámbrico

Haz funcionar hornos con energía en vez de combustible.

1. Coloca un <ItemLink id="heater" /> y dale energía (guarda 1.000.000 RF).
2. Pon un <ItemLink id="item_heater" /> en su casilla: el receptor queda vinculado.
3. Pon el receptor en la casilla de combustible de un horno.

- Cada objeto fundido cuesta 2.000 RF (aumento de velocidad: 4.000; eficiencia: 1.000).
- Un transmisor alimenta cualquier número de receptores en la misma dimensión.
- No sirve para generadores.
- Un transmisor roto conserva su energía.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/wireless.snbt" />
</GameScene>

<Recipe id="ironfurnaces:heater" />

<Recipe id="ironfurnaces:item_heater" />
