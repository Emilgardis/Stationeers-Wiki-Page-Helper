use std::{collections::HashMap, fmt::Write as _};

use regex::Regex;

#[derive(Debug, clap::Parser)]
pub struct Instructions {}

impl Instructions {
    pub(crate) fn run(
        &self,
        stationpedia: &crate::stationpedia::Stationpedia,
        _enums: &crate::enums::Enums,
        config: &toml_edit::DocumentMut,
        _verbose: bool,
    ) -> color_eyre::Result<()> {
        let mut instructions: HashMap<Vec<_>, Vec<ConfigInstruction>> = HashMap::new();
        #[allow(clippy::never_loop)]
        for (instruction, info) in &stationpedia.script_commands {
            let i = config["instructions"].as_table().unwrap();
            let mut col = InstructionCollector {
                command: instruction.clone(),
                category: vec![],
                actual_instruction: None,
                current_item: "".to_string(),
                info: Some(info.clone()),
            };
            toml_edit::visit::visit_table(&mut col, i);

            let Some(ins) = col.actual_instruction.as_ref() else {
                tracing::info!("skipping instruction {}", instruction);
                continue;
            };

            instructions
                .entry(ins.category.clone())
                .or_default()
                .push(ins.clone());
        }
        let mut output = String::new();
        output.push_str(
            "<noinclude>
See [[IC10]] for the primary page for the IC10 instruction set. This page lists all available instructions
</noinclude>

",
        );

        let mut colcat = CategoryCollector {
            category: vec![],
            categories: vec![],
        };
        toml_edit::visit::visit_table(&mut colcat, config["instructions"].as_table().unwrap());
        let categories = colcat.categories;
        let re = Regex::new(r"</?[^>]+>").unwrap();
        for category in &categories {
            if category.first().is_some_and(|s| s == "Deprecated") {
                continue;
            }
            let width = category.len() + 1;
            output.push_str(&format!(
                "{0:=<width$} {1} {0:=<width$}\n\n",
                "=",
                category.join(" / "),
            ));

            if let Some(inss) = instructions.get_mut(category) {
                inss.sort_by_key(|ins| ins.order);
                render(inss, &re, &mut output)?;
            }
            writeln!(output)?;
        }
        println!("{}", output);
        Ok(())
    }
}

fn render(
    inss: &[ConfigInstruction],
    re: &Regex,
    output: &mut String,
) -> Result<(), color_eyre::eyre::Error> {
    for ins in inss {
        let command = &ins.command;
        let real_desc = &ins.info.desc.replace('|', "{{!}}");
        let desc = if let Some(desc) = &ins.desc {
            desc
        } else {
            real_desc
        };
        let syntax = 'syntax: {
            if let Some(sy) = &ins.syntax {
                break 'syntax std::borrow::Cow::Borrowed(sy.as_str());
            };
            let syntax = ins.info.example.replace('|', "{{!}}");
            std::borrow::Cow::Owned(re.replace_all(&syntax, "").into_owned())
        };
        write!(
            output,
            "{{{{ICInstruction|instruction={command}|description={desc}|syntax={syntax}"
        )?;
        if let Some(example) = &ins.example {
            write!(output, "\n|example=\n{example}")?;
        }
        if let Some(note) = &ins.note {
            write!(output, "\n|note=\n{note}")?;
        }
        writeln!(output, "}}}}")?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct ConfigInstruction {
    command: String,
    category: Vec<String>,
    example: Option<String>,
    note: Option<String>,
    desc: Option<String>,
    syntax: Option<String>,
    info: crate::stationpedia::Command,
    order: usize,
}

#[derive(Debug)]
struct InstructionCollector {
    command: String,
    actual_instruction: Option<ConfigInstruction>,
    category: Vec<String>,
    current_item: String,
    info: Option<crate::stationpedia::Command>,
}

impl<'doc> toml_edit::visit::Visit<'doc> for InstructionCollector {
    fn visit_table_like_kv(&mut self, key: &'doc str, node: &'doc toml_edit::Item) {
        if self.actual_instruction.is_some() {
            return;
        }
        if key == "instructions" {
            if let Some(e) = node
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .find_map(|(e, l)| l.as_str().filter(|op| op == &self.command).map(|_| e))
            {
                self.actual_instruction = Some(ConfigInstruction {
                    command: self.command.clone(),
                    category: self.category.clone(),
                    example: None,
                    note: None,
                    desc: None,
                    syntax: None,
                    info: self.info.take().unwrap(),
                    order: e,
                });
            } else if let Some((e, it)) = node.as_array().unwrap().iter().enumerate().find_map(|(e, i)| {
                let it = i.as_inline_table();
                it.and_then(|t| t.get("op").and_then(|op| op.as_str()))
                    .is_some_and(|op| op == self.command)
                    .then_some(it)
                    .flatten()
                    .map(|it| (e, it))
            }) {
                self.actual_instruction = Some(ConfigInstruction {
                    command: self.command.clone(),
                    category: self.category.clone(),
                    example: it
                        .get("example")
                        .and_then(|e| e.as_str())
                        .map(|s| textwrap::dedent(s).trim().to_owned()),
                    note: it
                        .get("note")
                        .and_then(|e| e.as_str())
                        .map(|s| textwrap::dedent(s).trim().to_owned()),
                    desc: it
                        .get("desc")
                        .and_then(|e| e.as_str())
                        .map(|s| textwrap::dedent(s).trim().to_owned()),
                    syntax: it
                        .get("syntax")
                        .and_then(|e| e.as_str())
                        .map(|s| textwrap::dedent(s).trim().to_owned()),
                    info: self.info.take().unwrap(),
                    order: e
                });
            }
            return;
        }
        self.current_item = key.to_string();
        toml_edit::visit::visit_table_like_kv(self, key, node)
    }
    fn visit_table_like(&mut self, node: &'doc dyn toml_edit::TableLike) {
        if self.current_item.is_empty() {
            toml_edit::visit::visit_table_like(self, node);
            return;
        }
        self.category.push(self.current_item.clone());
        toml_edit::visit::visit_table_like(self, node);
        self.category.pop();
    }
}

struct CategoryCollector {
    category: Vec<String>,
    categories: Vec<Vec<String>>,
}

impl<'doc> toml_edit::visit::Visit<'doc> for CategoryCollector {
    fn visit_table_like_kv(&mut self, key: &'doc str, node: &'doc toml_edit::Item) {
        if key == "instructions" {
            return;
        }
        self.category.push(key.to_string());
        self.categories.push(self.category.clone());
        toml_edit::visit::visit_table_like_kv(self, key, node);
        self.category.pop();
    }
}
