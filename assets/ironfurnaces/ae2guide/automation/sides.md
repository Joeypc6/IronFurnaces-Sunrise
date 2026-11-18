---
navigation:
  title: Sides
  parent: automation/index.md
  position: 20
  icon: minecraft:hopper
---

# Sides

Each face is set to one of:

- **Input**: items to smelt go in (fuel too).
- **Output**: results come out; a generator sends power here.
- **Input/Output**: both.
- **Fuel Input/Eject**: fuel in, empty buckets out.
- **NONE**: closed.

New furnaces: top Input, bottom Output, the rest Fuel, so hoppers work as on a vanilla furnace. In a factory Input and Output reach every row; a generator takes items only through Fuel sides.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/sides.snbt" />
</GameScene>
