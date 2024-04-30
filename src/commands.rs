use crate::{enums::Enums, stationpedia::Stationpedia};

use clap::Subcommand;

mod instructions;
pub mod wikibox;

#[derive(Debug, Subcommand)]
pub enum Sub {
    Instructions(instructions::Instructions),
    Wikibox(wikibox::Wikibox),
}

impl super::Cli {
    pub fn run_subcommand(
        &self,
        stationpedia: &Stationpedia,
        enums: &Enums,
        config: &toml_edit::DocumentMut,
    ) -> color_eyre::Result<()> {
        match &self.subcommand {
            Sub::Instructions(c) => c.run(stationpedia, enums, config, self.verbose)?,
            Sub::Wikibox(c) => c.run(stationpedia, config, self.verbose)?,
        }
        Ok(())
    }
}
