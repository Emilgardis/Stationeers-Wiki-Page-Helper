use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

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
        let mut output: HashMap<Vec<_>, Vec<ConfigInstruction>> = HashMap::new();
        #[allow(clippy::never_loop)]
        for (instruction, info) in &stationpedia.script_commands {
            let i = config["instructions"]["category"].as_table().unwrap();
            let mut col = InstructionCollector {
                command: instruction.clone(),
                category: vec![],
                actual_instruction: None,
                current_item: "".to_string(),
                info: Some(info.clone()),
            };
            toml_edit::visit::visit_table(&mut col, i);

            let Some(ins) = col.actual_instruction.as_ref() else {
                continue;
            };

            output
                .entry(ins.category.clone())
                .or_default()
                .push(ins.clone());
        }
        println!("{output:#?}");
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ConfigInstruction {
    command: String,
    category: Vec<String>,
    example: Option<String>,
    note: Option<String>,
    info: crate::stationpedia::Command,
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
            if node
                .as_array()
                .unwrap()
                .iter()
                .any(|i| i.as_str().is_some_and(|op| op == self.command))
            {
                self.actual_instruction = Some(ConfigInstruction {
                    command: self.command.clone(),
                    category: self.category.clone(),
                    example: None,
                    note: None,
                    info: self.info.take().unwrap(),
                });
            } else if let Some(it) = node.as_array().unwrap().iter().find_map(|i| {
                let it = i.as_inline_table();
                it.and_then(|t| t.get("op").and_then(|op| op.as_str()))
                    .is_some_and(|op| op == self.command)
                    .then_some(it)
                    .flatten()
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
                    info: self.info.take().unwrap(),
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
