Generate pages for [stationeers wiki](https://stationeers-wiki.com)


# Pre-req

Generate the `Stationpedia.json` using [Ryex/StationeersStationpediaExtractor](https://github.com/Ryex/StationeersStationpediaExtractor), place the file in this repo.

# Commands

## Wikibox

Returns the code for a box on the wiki

```bash
$ cargo run -q wikibox "Composite Door"

{{Structurebox
| name = Composite Door
| image = [[File:StructureCompositeDoor.png]]
| prefab_hash = -793837322
| prefab_name = StructureCompositeDoor
| power_usage = 10W
| placed_on_grid = Small Grid
| decon_with_tool1 = [[Wrench]]
| placed_with_item = [[Kit (Door)]]
| item_rec1 = [[Kit (Door)]]
| decon_with_tool2 = [[Hand Drill]]
| const_with_tool1 = [[Welding Torch]]
| const_with_item1 = 1 x [[Plastic Sheets]]
| decon_with_tool3 = [[Angle Grinder]]
| const_with_tool2 = [[Crowbar]]
| const_with_item2 = 1 x [[Glass Sheets]]
}}
```

```bash
$ cargo run -q wikibox "Tomato soup"

{{Itembox
| name = Tomato Soup
| image = [[File:ItemTomatoSoup.png]]
| prefabhash = 688734890
| prefabname = ItemTomatoSoup
| stacks = 1
| recipe_machine1 = Advanced Packaging Machine
| recipe_cost1 = 1g [[Steel]], 1ml [[Oil]], 5 x [[Tomato]]
| recipe_machine2 = Basic Packaging Machine
| recipe_cost2 = 1g [[Steel]], 1ml [[Oil]], 5 x [[Tomato]]
}}
```

## Instructions

Returns the text for <https://stationeers-wiki.com/index.php?title=MIPS/instructions>

