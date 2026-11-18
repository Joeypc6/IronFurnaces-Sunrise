---
navigation:
  title: Crafting ETA
  parent: index.md
  position: 91
  icon: ae2:crafting_unit
requires:
- ae2:pattern_provider
---

# Crafting ETA

AE2's crafting status shows a job's time left. For a pattern pushed into one of these furnaces it uses the furnace's own speed, right from the first craft:

- its tier's smelt time;
- the Speed and Fuel Efficiency augments;
- the Blasting or Smoking augment;
- how many it smelts at once (Rainbow, Allthemodium, Vibranium, Unobtainium);
- a factory's rows, with Auto Split on.

A generator, or a furnace stopped by redstone, gives no time. With [Machine Work](machine_work.md) AE2 reads it through that library; the times are the same.
