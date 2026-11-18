# The Rainbow Furnace and wireless heat

The Rainbow Furnace smelts a stack at a time. As a generator it needs one of each tier of furnace, placed by the same player, running
as generators with room in their buffers. A blast takes it apart and leaves a Rainbow Coal behind.

A **Wireless Heat Transmitter** takes power. Put a **Receiver** in it to bind them, then in a furnace's fuel slot, and the furnace
burns the transmitter's power.

<GameScene zoom="3" interactive={true}>
  <ImportStructure src="scenes/heater.snbt" />
</GameScene>

<RecipeFor id="ironfurnaces:million_furnace" />
<RecipeFor id="ironfurnaces:rainbow_core" />
<RecipeFor id="ironfurnaces:rainbow_plating" />
