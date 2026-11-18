---
navigation:
  title: Wireless heat
  parent: energy/index.md
  position: 40
  icon: heater
item_ids:
- ironfurnaces:heater
- ironfurnaces:item_heater
---

# Wireless heat

Run furnaces on power instead of fuel.

1. Place a <ItemLink id="heater" /> and give it power (it holds 1,000,000 RF).
2. Put a <ItemLink id="item_heater" /> in its slot: the receiver is now bound to it.
3. Put the receiver in a furnace's fuel slot.

- Each item smelted costs 2,000 RF (Speed augment: 4,000; Fuel Efficiency: 1,000).
- One transmitter feeds any number of receivers in the same dimension.
- Not for generators.
- A broken transmitter keeps its power.

<GameScene zoom="4" interactive={true}>
  <ImportStructure src="../scenes/wireless.snbt" />
</GameScene>

<Recipe id="ironfurnaces:heater" />

<Recipe id="ironfurnaces:item_heater" />
