# Iron Furnaces

Furnaces of every metal, each faster than the last. A furnace's side buttons say what each face does (input, output, fuel), and
hoppers and pipes follow them. Sneak and right-click a furnace with an upgrade to change its tier and keep what's inside.

<GameScene zoom="3" interactive={true}>
  <ImportStructure src="scenes/tiers.snbt" />
</GameScene>

Hoppers follow the sides: one on top feeds the input, one under it takes the result.

<GameScene zoom="3" interactive={true}>
  <ImportStructure src="scenes/automation.snbt" />
</GameScene>
<IfMods items="ae2:pattern_provider">

With AE2, a pattern provider on top fills it, and a face set to Input/Output with Auto Output sends the result back:

<GameScene zoom="3" interactive={true}>
  <ImportStructure src="scenes/automation_ae2.snbt" />
</GameScene>
</IfMods>

<RecipeFor id="ironfurnaces:iron_furnace" />
<RecipeFor id="ironfurnaces:copper_furnace" />
<RecipeFor id="ironfurnaces:silver_furnace" />
