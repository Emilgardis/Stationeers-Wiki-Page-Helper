use crate::{enums::Enums, stationpedia::Stationpedia};

use clap::Subcommand;

pub mod instructions;
#[cfg(feature = "mw")]
pub mod query;
pub mod wikibox;

#[derive(Debug, Subcommand)]
pub enum Sub {
    Instructions(instructions::Instructions),
    Wikibox(wikibox::Wikibox),
    #[cfg(feature = "mw")]
    Query(query::Query),
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
            Sub::Wikibox(c) => c.run(stationpedia, enums, config, self.verbose)?,
            #[cfg(feature = "mw")]
            Sub::Query(c) => c.run(stationpedia, config, self.verbose)?,
        }
        Ok(())
    }
}
