"""Writes Iron Furnaces' assets/ and data/ for the fork build (ironfurnaces-fork/), from the NeoForge jar.

It reuses ironfurnaces-horizon/tools/make_content.py (the upstream content mod) and changes what the fork reads
differently:

- the block declarations: a plain iron block template (the block entity type is declared in code, so no beacon is
  needed), and a `state_light` table so a lit furnace gives off 14 (BlockIronFurnaceBase.getLightEmission; upstream
  needed a mixin on the state table for this);
- no `horizon-mod.toml` inside: the fork mod's own manifest is the only one;
- no `assets/ironfurnaces/atlases/gui.json`: its `neoforge:namespaced_directory` source isn't a type Horizon reads
  (the GUI textures are drawn by name, `ironfurnaces:gui/...`);
- the guide's Rainbow page says what BlockMillionFurnace.onExplosionHit does: a blast leaves a Rainbow Coal.

- the W guide's pages go to assets/ironfurnaces/ae2guide, where GuideME (../guide-fork) reads them (its <IfMods>
  shows the part about AE2 only with AE2 installed, as the Java guide).

Usage: python make_fork_content.py [jar]
"""

import json
import os
import shutil
import sys
import tempfile
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
UPSTREAM_TOOLS = os.path.join(ROOT, "..", "ironfurnaces-horizon", "tools")
sys.path.insert(0, UPSTREAM_TOOLS)
import make_content  # noqa: E402

JAR = os.path.join(ROOT, "..", "Ported", "ironfurnaces-neoforge-26.1.2-4.4.2.jar")
ID = make_content.ID

# Registration's Properties.ofFullCopy(<block>): the template each furnace copies its look-up rules from.
TEMPLATES = {
    "obsidian_furnace": "minecraft:obsidian",
    "crystal_furnace": "minecraft:glass",
    "netherite_furnace": "minecraft:netherite_block",
    "allthemodium_furnace": "minecraft:netherite_block",
    "vibranium_furnace": "minecraft:netherite_block",
    "unobtainium_furnace": "minecraft:netherite_block",
    "gold_furnace": "minecraft:gold_block",
    "diamond_furnace": "minecraft:diamond_block",
    "million_furnace": "minecraft:diamond_block",
    "emerald_furnace": "minecraft:emerald_block",
    "copper_furnace": "minecraft:copper_block",
}


def furnace_block(name):
    decl = make_content.furnace_block(name)
    decl["template"] = TEMPLATES.get(name, "minecraft:iron_block")
    if name == "crystal_furnace":
        # Glass: drawn see-through, so it doesn't hide its neighbours' faces.
        decl["properties"]["occludes"] = False
    decl["state_light"] = {"properties": ["lit"], "values": {"true": 14, "false": 0}, "default": 0}
    return decl


def heater_block():
    decl = make_content.heater_block()
    decl["template"] = "minecraft:iron_block"
    return decl


GUIDE_FIXES = {
    "rainbow.md": [(
        "A blast takes it apart and leaves its core behind.",
        "A blast takes it apart and leaves a Rainbow Coal behind.",
    )],
}


def main(jar):
    with tempfile.TemporaryDirectory() as tmp:
        out = os.path.join(tmp, "content.hmod")
        make_content.main(jar, out)
        files = {}
        with zipfile.ZipFile(out) as z:
            for name in z.namelist():
                files[name] = z.read(name)
    files.pop("horizon-mod.toml", None)
    for name in list(files):
        if name.startswith(f"assets/{ID}/atlases/"):
            files.pop(name)
    for name in make_content.FURNACES:
        files[f"data/{ID}/horizon/block/{name}.json"] = json.dumps(furnace_block(name), indent=2).encode()
    files[f"data/{ID}/horizon/block/heater.json"] = json.dumps(heater_block(), indent=2).encode()
    # The W guide is GuideME's (../guide-fork): its pages are client assets, assets/<ns>/ae2guide.
    for name in [n for n in files if n.startswith(f"data/{ID}/ae2guide/")]:
        files["assets/" + name[len("data/"):]] = files.pop(name)
    files[f"assets/{ID}/ae2guide/guide.json"] = json.dumps({"title": f"{ID}.guide.title", "start": "furnaces.md"}).encode()
    # Live recipe boxes on the item pages (../../tools/guide_recipes.py) and how guide-fork draws the mod's own types.
    sys.path.insert(0, os.path.join(ROOT, "..", "tools"))
    import guide_recipes
    g = f"assets/{ID}/ae2guide/guide.json"
    files[g] = guide_recipes.merge_types(files[g], guide_recipes.add(files, ID))
    # A stand-in for a shift-clicked stack that moved whole (src/quickmove.rs): never kept, in no tab, no recipe.
    files[f"data/{ID}/horizon/item/moved.json"] = json.dumps({"template": "minecraft:paper", "max_stack_size": 1}, indent=2).encode()
    for lang, title in (("en_us", "Iron Furnaces"),):
        path = f"assets/{ID}/lang/{lang}.json"
        doc = json.loads(files[path])
        doc[f"{ID}.guide.title"] = title
        doc[f"item.{ID}.moved"] = "Moved"
        files[path] = json.dumps(doc, indent=2, ensure_ascii=False).encode()
    for page, fixes in GUIDE_FIXES.items():
        path = f"assets/{ID}/ae2guide/{page}"
        text = files[path].decode()
        for old, new in fixes:
            assert old in text, (page, old)
            text = text.replace(old, new)
        files[path] = text.encode()
    for tree in ("assets", "data"):
        shutil.rmtree(os.path.join(ROOT, tree), ignore_errors=True)
    for name, data in files.items():
        path = os.path.join(ROOT, *name.split("/"))
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as f:
            f.write(data)
    print(len(files), "files written under", ROOT)


def commands_menu():
    """The `/ironfurnaces` commands menu's pages (`../../tools/commands-menu/ironfurnaces.json`), written into assets/ after the wipe."""
    import importlib.util
    path = os.path.join(ROOT, "..", "tools", "commands-menu", "pages.py")
    spec = importlib.util.spec_from_file_location("commands_menu_pages", path)
    pages = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(pages)
    pages.install(ROOT, "ironfurnaces")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else JAR)
    commands_menu()
