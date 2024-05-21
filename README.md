Generate pages for [stationeers wiki](https://stationeers-wiki.com)

# Pre-req

`third_party/Stationpedia.json` and `third_party/Enums.json` are generated using a fork of [StationeersStationpediaExtractor](https://github.com/Emilgardis/StationeersStationpediaExtractor).

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
| item_rec2 = 1 x [[Plastic Sheets]]
| decon_with_tool3 = [[Angle Grinder]]
| const_with_tool2 = [[Crowbar]]
| const_with_item2 = 1 x [[Glass Sheets]]
| item_rec3 = 1 x [[Glass Sheets]]
}}

<blockquote><q>[[Recurso Espaciais (Faction)|Recurso's]] composite doors are rated to 300kPa, which is more than sufficient for most purposes they were designed for. However, steep pressure differentials are not your friend.</q><br>
'''- Stationpedia'''</blockquote>

{{Data Network Header}}
{{Data Parameters|
{{Data Parameters/row|Power|Boolean|w=0|Can be read to return if the Composite Door is correctly powered or not, set via the power system, return 1 if powered and 0 if not|multiple=2|0|Unpowered|1|Powered}}
{{Data Parameters/row|Open|Integer|1 if device is open, otherwise 0}}
{{Data Parameters/row|Mode|Integer|The mode of the Composite Door.|multiple=2|0|Operate|1|Logic}}
{{Data Parameters/row|Lock|Boolean|Disable manual operation of the Composite Door.|multiple=2|0|Unlocked|1|Locked}}
{{Data Parameters/row|Setting|Integer|A variable setting that can be read or written.}}
{{Data Parameters/row|On|Boolean|The current state of the Composite Door.|multiple=2|0|Off|1|On}}
{{Data Parameters/row|RequiredPower|Integer|w=0|Idle operating power quantity, does not necessarily include extra demand power}}
{{Data Parameters/row|Idle|Integer|w=0|Returns 1 if the Composite Door is currently idle, otherwise 0}}
{{Data Parameters/row|PrefabHash|Integer|w=0|The hash of the structure}}
{{Data Parameters/row|ReferenceId|Integer|w=0|Unique Reference Identifier for this object}}
{{Data Parameters/row|NameHash|Integer|w=0|Provides the hash value for the name of the object as a 32 bit integer.}}
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
| slot_class = SlotClass.None
| sorting_class = SortingClass.Food
| recipe_machine1 = Advanced Packaging Machine
| recipe_cost1 = 1 x [[Empty Can]], 1ml [[Soy Oil]], 5 x [[Cooked Tomato]]
| recipe_machine2 = Basic Packaging Machine
| recipe_cost2 = 1 x [[Empty Can]], 1ml [[Soy Oil]], 5 x [[Cooked Tomato]]
| nutrition = 290
| quality = Good (+25% hydration capacity
}}

<blockquote><q>Made using [[Cooked Tomato]]s and an [[Empty Can]] in a [[Basic Packaging Machine]] or [[Advanced Packaging Machine]].</q><br>
'''- Stationpedia'''</blockquote>

== Recipes ==
{{Recipe
|{{Recipe/row |machine = Advanced Packaging Machine |mats = 1 x [[Empty Can]], 1ml [[Soy Oil]], 5 x [[Cooked Tomato]] |time = 5 |energy = 0}}
|{{Recipe/row |machine = Basic Packaging Machine |mats = 1 x [[Empty Can]], 1ml [[Soy Oil]], 5 x [[Cooked Tomato]] |time = 5 |energy = 0}}
}}
```

## Instructions

Returns the text for <https://stationeers-wiki.com/index.php?title=MIPS/instructions>

### License

<sup>
This project is licensed under the MIT License. See the LICENSE file for more details.
</sup>

### Third-Party Content

This project includes files generated using content from RocketWerkz Ltd., those files are provided in `third_party`
