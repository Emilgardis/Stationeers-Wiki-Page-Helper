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
                    }
                    if let Some(tool2) = tool.get(1) {
                        let name = &pedia.lookup_prefab_name(&tool2.prefab_name).unwrap().title;
                        if let Some(quantity) = tool2.quantity {
                            writeln!(out, "| const_with_item{count} = {quantity} x [[{name}]]")?;
                        } else {
                            writeln!(out, "| const_with_item{count} = [[{name}]]")?;
                        }
                    }
                } else if let Some(tool1) = tool.first() {
                    let name = &pedia.lookup_prefab_name(&tool1.prefab_name).unwrap().title;
                    if let Some(quantity) = tool1.quantity {
                        writeln!(out, "| const_with_item{count} = {quantity} x [[{name}]]")?;
                    } else {
                        writeln!(out, "| const_with_item{count} = [[{name}]]")?;
                    }
                    if tool.len() > 1 {
                        panic!()
                    }
                }
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
            ..
        } = &self;
        let Some(item) = item else {
            return Ok(None);
        };
        out.push_str(&textwrap::dedent(&format!(
            "
                {{{{Itembox
                | name = {title}
                | image = [[File:{prefab_name}.png]]
                | prefabhash = {prefab_hash}
                | prefabname = {prefab_name}\n"
        )));

        if let Some(q) = item.max_quantity {
            writeln!(out, "| stacks = {q}")?;
        }
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
                let amount = match name.as_str() {
                    "Iron" | "Gold" | "Carbon" | "Uranium" | "Copper" | "Steel" | "Hydrocarbon"
                    | "Silver" | "Electrum" | "Invar" | "Constantan" | "Solder" | "Silicon"
                    | "Waspaloy" | "Stellite" | "Inconel" | "Hastelloy" | "Astroloy" | "Cobalt" => {
                        "g"
                    }
                    _ => " x",
                };
                write!(ingredients, "{quantity}{amount} [[{name}]]")?;
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
        write!(out, "}}}}")?;
        Ok(Some(out))
    }
}

impl Wikibox {
    pub(crate) fn run(
        &self,
        stationpedia: &crate::stationpedia::Stationpedia,
        _config: &toml_edit::DocumentMut,
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
