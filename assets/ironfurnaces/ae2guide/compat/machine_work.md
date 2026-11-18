---
navigation:
  title: Machine Work
  parent: index.md
  position: 92
  icon: ae2:crafting_unit
requires:
- ae2:pattern_provider
---

# Machine Work

Machine Work is a small library mod: a machine tells other mods how far its work is and how long one operation takes. These furnaces answer it:

- **progress**: the smelt under way; in a factory the row closest to done, with the rows cooking.
- **pace**: the time one smelt of an item takes now, with how many it smelts at once.
- A generator, or a furnace stopped by redstone, answers nothing.

AE2's [crafting ETA](ae2_eta.md) reads it first.
