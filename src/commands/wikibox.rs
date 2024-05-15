//! Generates a wiki box for a given item.

use std::fmt::Write as _;

use crate::stationpedia::{Page, Stationpedia};

#[derive(Debug, clap::Parser)]
pub struct Wikibox {
    item: String,
}

#[derive(Clone)]
pub struct Hit<'a> {
    pub page: &'a Page,
    pub constructs: &'a Vec<&'a Page>,
    pub constructed_by: Option<&'a Page>,
}

impl Page {
    fn structure(&self, pedia: &Stationpedia) -> color_eyre::Result<Option<String>> {
        let mut out = String::new();
        let Page {
            structure,
            prefab_hash,
            prefab_name,
            title,
            base_power_draw,
            ..
        } = &self;
        let Some(structure) = structure else {
            return Ok(None);
        };

        out.push_str(&textwrap::dedent(&format!(
            "
                {{{{Structurebox
                | name = {title}
                | image = [[File:{prefab_name}.png]]
                | prefab_hash = {prefab_hash}
                | prefab_name = {prefab_name}\n"
        )));

        if let Some(power) = base_power_draw {
            writeln!(out, "| power_usage = {power}")?;
        }

        let grid = if structure.small_grid {
            "Small"
        } else {
            "Large"
        };
        writeln!(out, "| placed_on_grid = {grid} Grid")?;
        let mut rec = None;
        for (count, state) in structure.build_states.0.iter().enumerate() {
            let rcount = count + 1;
            if let Some(exit) = &state.tool_exit {
                if exit.len() > 1 {
                    panic!()
                }
                let tool = &pedia
                    .lookup_prefab_name(&exit[0].prefab_name)
                    .unwrap()
                    .title;
                writeln!(out, "| decon_with_tool{rcount} = [[{tool}]]")?;
            }
            if let Some(tool) = &state.tool {
                // to always get the tool, we sort by is_tool
                let mut tools = tool.clone();
                tools.sort_by_key(|t| t.is_tool);
                let has_tool = tools.iter().any(|t| t.is_tool);
                if count == 0 {
                    if has_tool {
                        panic!()
                    }
                    let placed_with_item = &pedia
                        .lookup_prefab_name(&tool[0].prefab_name)
                        .unwrap()
                        .title;
                    writeln!(out, "| placed_with_item = [[{placed_with_item}]]")?;
                    writeln!(out, "| item_rec1 = [[{placed_with_item}]]")?;
                } else if has_tool {
                    if let Some(tool1) = tool.first() {
                        let name = &pedia.lookup_prefab_name(&tool1.prefab_name).unwrap().title;
                        if let Some(quantity) = tool1.quantity {
                            writeln!(out, "| const_with_tool{count} = {quantity} x [[{name}]]")?;
                        } else {
                            writeln!(out, "| const_with_tool{count} = [[{name}]]")?;
                        }
                        rec = None;
                    }
                    if let Some(tool2) = tool.get(1) {
                        let name = &pedia.lookup_prefab_name(&tool2.prefab_name).unwrap().title;
                        if let Some(quantity) = tool2.quantity {
                            writeln!(out, "| const_with_item{count} = {quantity} x [[{name}]]")?;
                            rec = Some(format!("{quantity} x [[{name}]]"));
                        } else {
                            writeln!(out, "| const_with_item{count} = [[{name}]]")?;
                            rec = Some(format!("[[{name}]]"));
                        }
                    }
                } else if let Some(tool1) = tool.first() {
                    let name = &pedia.lookup_prefab_name(&tool1.prefab_name).unwrap().title;
                    if let Some(quantity) = tool1.quantity {
                        writeln!(out, "| const_with_item{count} = {quantity} x [[{name}]]")?;
                        rec = Some(format!("{quantity} x [[{name}]]"));
                    } else {
                        writeln!(out, "| const_with_item{count} = [[{name}]]")?;
                        rec = Some(format!("[[{name}]]"));
                    }
                    if tool.len() > 1 {
                        panic!()
                    }
                }
            }
            if let Some(rec) = &rec {
                writeln!(out, "| item_rec{rcount} = {rec}")?;
            }
        }
        write!(out, "}}}}")?;

        Ok(Some(out))
    }

    pub fn item(&self, pedia: &Stationpedia) -> color_eyre::Result<Option<String>> {
        let mut out = String::new();
        let Page {
            constructs,
            item,
            prefab_hash,
            prefab_name,
            title,
            growth_time,
            ..
        } = &self;
        let Some(item) = item else {
            return Ok(None);
        };
        out.push_str(
            textwrap::dedent(&format!(
                "
                {{{{Itembox
                | name = {title}
                | image = [[File:{prefab_name}.png]]
                | prefabhash = {prefab_hash}
                | prefabname = {prefab_name}
            "
            ))
            .trim_start(),
        );
        writeln!(out, "| stacks = {}", item.max_quantity.unwrap_or(1.0))?;

        writeln!(out, "| slot_class = SlotClass.{}", item.slot_class)?;
        writeln!(out, "| sorting_class = SortingClass.{}", item.sorting_class)?;

        let mut count = 1;
        for recipe in &item.recipes {
            let mut ingredients = String::new();
            for (i, (ingredient, quantity)) in recipe
                .reagents
                .iter()
                .filter(|(_, q)| *q > &0.0)
                .enumerate()
            {
                let name = &pedia
                    .lookup_prefab_name(ingredient)
                    .map(|i| &i.title)
                    .unwrap_or(ingredient);
                if i > 0 {
                    ingredients.push_str(", ");
                }
                let (amount, ingredient) = recipe_amount(pedia, name, &recipe.creator_prefab_name);
                write!(ingredients, "{quantity}{amount} [[{ingredient}]]")?;
            }
            if recipe.creator_prefab_name == "StructureOrganicsPrinter" {
                continue;
            }
            let creator = &pedia
                .lookup_prefab_name(&recipe.creator_prefab_name)
                .unwrap()
                .title;
            let tier = if recipe.tier_name == "TierTwo" {
                " (Tier Two)"
            } else {
                ""
            };
            writeln!(out, "| recipe_machine{count} = {creator}{tier}",)?;
            writeln!(out, "| recipe_cost{count} = {ingredients}",)?;
            count += 1;
        }

        // constructs
        if !constructs.is_empty() {
            let contructs = constructs
                .iter()
                .map(|c| {
                    let name = &pedia.lookup_key(&c.page_link).unwrap().title;
                    format!("[[{name}]]")
                })
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(out, "| constructs = {contructs}")?;
        }

        if let Some(food) = &item.food {
            if food.nutrition_value.is_some_and(|v| v > 0.0) {
                writeln!(out, "| nutrition = {}", food.nutrition_value.unwrap())?;
                writeln!(out, "| quality = {}", food.nutrition_quality_readable)?;
                if let Some(bonus) = food.mood_bonus.filter(|v| v != &0.0) {
                    writeln!(out, "| moodbonus = {}%", bonus * 100.0)?;
                }
            }
        }
        if let Some(growth_time) = &growth_time {
            writeln!(out, "| growthtime = {}", growth_time)?;
        }
        write!(out, "}}}}")?;
        Ok(Some(out))
    }

    pub fn item_recipe(&self, pedia: &Stationpedia) -> color_eyre::Result<Option<String>> {
        let mut out = String::new();
        let Page { item, .. } = &self;
        let Some(item) = item else {
            return Ok(None);
        };
        // {{Recipe
        // |{{Recipe/row |machine = Autolathe  |mats = 10g [[Iron]], 2g [[Copper]] |time = 10 |energy = 500}}
        // |{{Recipe/row |machine = Fabricator |mats = 10g [[Iron]], 2g [[Copper]] |time = 1 |energy = 500}}
        // |{{Recipe/row |machine = Recycler   |mats = 5g [[Iron]], 1g [[Copper]]  |time = 10 |energy = 1000}}
        // }}
        if item.recipes.is_empty() {
            return Ok(None);
        }
        out.push_str(
            textwrap::dedent(
                "
                == Recipes ==
                {{Recipe
            ",
            )
            .trim_start(),
        );
        for recipe in &item.recipes {
            let mut ingredients = String::new();
            for (i, (ingredient, quantity)) in recipe
                .reagents
                .iter()
                .filter(|(_, q)| *q > &0.0)
                .enumerate()
            {
                let _name = &pedia
                    .lookup_prefab_name(ingredient)
                    .map(|i| &i.title)
                    .unwrap_or(ingredient);
                if i > 0 {
                    ingredients.push_str(", ");
                }
                let (amount, ingredient) =
                    recipe_amount(pedia, ingredient, &recipe.creator_prefab_name);
                write!(ingredients, "{quantity}{amount} [[{ingredient}]]")?;
            }
            if recipe.creator_prefab_name == "StructureOrganicsPrinter" {
                continue;
            }
            let creator = &pedia
                .lookup_prefab_name(&recipe.creator_prefab_name)
                .unwrap()
                .title;
            let tier = if recipe.tier_name == "TierTwo" {
                " (Tier Two)"
            } else {
                ""
            };
            let time = recipe.time;
            let energy = recipe.energy;
            writeln!(out, "|{{{{Recipe/row |machine = {creator}{tier} |mats = {ingredients} |time = {time} |energy = {energy}}}}}")?;
        }
        write!(out, "}}}}")?;
        Ok(Some(out))
    }

    pub fn description(
        &self,
        pedia: &Stationpedia,
        config: &toml_edit::DocumentMut,
    ) -> color_eyre::Result<Option<String>> {
        if self.description.is_empty() {
            return Ok(None);
        }
        let mut out = String::new();
        out.push_str(&textwrap::dedent(
            "
            == Stationpedia Description ==
            ",
        ));
        // description looks like html, example: The advanced <link=Xigo><color=#0080FFFF>Xigo</color></link> Padi 2 tablet is an improved version of the basic <link=ThingItemTablet><color=green>Handheld Tablet</color></link>, boasting two <link=CartridgePage><color=#0080FFFF>cartridge</color></link> slots. The Padi 2 accepts <link=ThingCartridgeAtmosAnalyser><color=green>Atmos Analyzer</color></link>, <link=ThingCartridgeTracker><color=green>Tracker</color></link>, <link=ThingCartridgeMedicalAnalyser><color=green>Medical Analyzer</color></link>, <link=ThingCartridgeOreScanner><color=green>Ore Scanner</color></link>, <link=ThingCartridgeElectronicReader><color=green>eReader</color></link>, and various other cartridges.\n\t  \n\t  With a <link=ThingItemIntegratedCircuit10><color=green>Integrated Circuit (IC10)</color></link> in the <link=SlotProgrammableChip><color=orange>Programmable Chip</color></link>, you can access variable slots on the carrying human using the device numbers (d0, d1, etc...), so long as the item can be access via logic, such as the <link=ThingItemHardSuit><color=green>Hardsuit</color></link>.Connects to <pos=300><link=ThingStructureLogicTransmitter><color=green>Logic Transmitter</color></link>
        // we need to replace <link>s with proper wiki links.
        // For example:
        // Xigo is a faction, which should link to `Xigo (faction)`
        // ThingItemTablet should link to that key in the stationpedia, e.g [[Handheld Tablet]] which is the displayname of ThingItemTablet
        // etc

        // Implementation for all descriptions
        let re = regex::Regex::new(r"<color=.*?>|</color>").unwrap();
        let description = re.replace_all(&self.description, "").to_string();

        // then we walk through each link and replace it with the proper wiki link, we do this by splitting the string on <link=
        // then we split on > to get the link and the text and finally insert the proper wiki link and rest of text
        let split = description.split("<link=");
        for link in split {
            tracing::debug!("split: {}", link);
            let Some((thing, rest)) = link.split_once('>') else {
                out.push_str(link);
                continue;
            };
            let (display, rest) = rest.split_once("</link>").unwrap();
            if let Some(link) = config
                .get("stationpedia")
                .and_then(|c| c.get("links"))
                .and_then(|c| c.get(thing))
                .and_then(|c| c.as_str())
            {
                if link == display {
                    out.push_str(&format!("[[{}]]", link));
                } else {
                    out.push_str(&format!("[[{}|{}]]", link, display));
                }
            } else if let Some(item) = pedia.lookup_key(thing) {
                if item.title == display {
                    out.push_str(&format!("[[{}]]", item.title));
                } else {
                    out.push_str(&format!("[[{}|{}]]", item.title, display));
                }
            } else if let Some(slot) = thing.strip_prefix("Slot") {
                out.push_str(&format!("{} slot", slot));
            } else {
                out.push_str(&format!("[[{}|{}]]", thing, display));
            }
            out.push_str(rest)
        }
        // now, replace all leftover tags
        let re = regex::Regex::new(r"<[^>]+>").unwrap();
        let out = re.replace_all(&out, "").to_string();
        Ok(Some(out))
    }
}

fn recipe_amount<'a>(
    pedia: &'a Stationpedia,
    ingredient: &'a str,
    creator_prefab_name: &str,
) -> (&'static str, &'a str) {
    // FIXME: Use correct ingredient name, Soy is for example really Soybean, Steel could be Can, etc etc

    let ingredient = match creator_prefab_name {
        "ApplianceMicrowave" | "StructureAutomatedOven" => {
            // takes uncooked items.
            pedia
                .pages
                .iter()
                .filter_map(|p| p.item.as_ref().map(|item| (p, item)))
                .filter(|(_, i)| i.reagents.is_some())
                .filter(|(_, i)| {
                    i.reagents
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|(r, a)| r == ingredient && a > &0.0)
                })
                .find(|(_, item)| item.food.as_ref().is_some_and(|f| f.nutrition_quality == 1))
                .map(|(i, _)| &*i.title)
                .unwrap_or(ingredient)
        }
        "AppliancePackagingMachine" | "StructureAdvancedPackagingMachine" => match ingredient {
            "Steel" => "Empty Can",
            _ => pedia
                .pages
                .iter()
                .filter_map(|p| p.item.as_ref().map(|item| (p, item)))
                .filter(|(_, i)| i.reagents.is_some())
                .filter(|(_, i)| {
                    i.reagents
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|(r, a)| r == ingredient && a > &0.0)
                })
                .find(|(_, item)| {
                    item.food
                        .as_ref()
                        .is_some_and(|f| f.nutrition_quality == 2 || ingredient == "Oil")
                })
                .map(|(i, _)| &*i.title)
                .unwrap_or(ingredient),
        },
        "ApplianceChemistryStation" => match ingredient {
            "Fenoxitone" => "Fern",
            _ => ingredient,
        },
        _ => ingredient,
    };
    let amount = match ingredient {
        "Iron" | "Gold" | "Carbon" | "Uranium" | "Copper" | "Steel" | "Hydrocarbon" | "Silver"
        | "Electrum" | "Invar" | "Constantan" | "Solder" | "Silicon" | "Waspaloy" | "Stellite"
        | "Inconel" | "Hastelloy" | "Astroloy" | "Cobalt" | "Flour" => "g",
        "Milk" | "Soy Oil" => "ml",
        _ => " x",
    };
    (amount, ingredient)
}

impl Wikibox {
    pub(crate) fn run(
        &self,
        stationpedia: &crate::stationpedia::Stationpedia,
        config: &toml_edit::DocumentMut,
        verbose: bool,
    ) -> color_eyre::Result<()> {
        // Pair up items that construct things with that thing.
        let mut hits = vec![];
        let page = if let Ok(hash) = self.item.parse::<i64>() {
            stationpedia
                .pages
                .iter()
                .find(|p| p.prefab_hash == hash)
                .unwrap()
        } else {
            // find matches, either by key, title or prefab_name
            let mut matcher = nucleo_matcher::Matcher::default();

            let mut pat = None::<nucleo_matcher::pattern::Pattern>;
            for page in &stationpedia.pages {
                if text_match(&mut matcher, &mut pat, &page.key, &self.item) > 100
                    || text_match(&mut matcher, &mut pat, &page.title, &self.item) > 100
                    || text_match(&mut matcher, &mut pat, &page.prefab_name, &self.item) > 100
                {
                    hits.push(page);
                }
            }
            if hits.is_empty() {
                eprintln!("No matches found for {}", self.item);
                return Ok(());
            }
            if let Some(exact) = hits
                .iter()
                .find(|p| p.key == self.item || p.title == self.item || p.prefab_name == self.item)
            {
                exact
            } else {
                if hits.len() > 1 {
                    eprintln!("Multiple matches found for {}", self.item);
                    for m in hits {
                        eprintln!("  {} - {}", m.key, m.title);
                    }
                    return Ok(());
                }
                &hits[0]
            }
        };
        if verbose {
            eprintln!("got match: \n---page:\n{:#?}", page,);
        }

        if let Some(item) = page.item(stationpedia)? {
            println!("\n{}", item);
        }

        if let Some(structure) = page.structure(stationpedia)? {
            println!("\n{}", structure);
        }

        if let Some(description) = page.description(stationpedia, config)? {
            println!("\n{}", description);
        }

        if let Some(item) = page.item_recipe(stationpedia)? {
            println!("\n{}", item);
        }

        Ok(())
    }
}

fn text_match(
    matcher: &mut nucleo_matcher::Matcher,
    pattern_scratch: &mut Option<nucleo_matcher::pattern::Pattern>,
    text: &str,
    search: &str,
) -> u32 {
    use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
    let pat = if let Some(pat) = pattern_scratch {
        //pat.reparse(search, CaseMatching::Smart, Normalization::Smart);
        pat
    } else {
        let _ = pattern_scratch.insert(Pattern::new(
            search,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        ));
        pattern_scratch.as_mut().unwrap()
    };
    let Some(matches) = pat.score(nucleo_matcher::Utf32Str::Ascii(text.as_bytes()), matcher) else {
        return 0;
    };
    matches
}
