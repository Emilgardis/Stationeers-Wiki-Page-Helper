#![cfg(feature = "mw")]
//! Query items on wiki

use color_eyre::eyre;

use crate::stationpedia::Stationpedia;

#[derive(Debug, clap::Parser)]
pub struct Query {
    #[command(subcommand)]
    sub: QuerySubcommand,
}
impl Query {
    pub(crate) fn run(
        &self,
        stationpedia: &Stationpedia,
        config: &toml_edit::DocumentMut,
        verbose: bool,
    ) -> color_eyre::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(self._run(stationpedia, config, verbose))
    }
    pub(crate) async fn _run(
        &self,
        _stationpedia: &Stationpedia,
        config: &toml_edit::DocumentMut,
        _verbose: bool,
    ) -> color_eyre::Result<()> {
        let client = mwapi::Client::builder(
            config["bot"]["api_url"]
                .as_str()
                .ok_or_else(|| eyre::eyre!("No api_url in config"))?,
        )
        .build()
        .await?;
        dbg!(
            client
                .get_value(&[("action", "query"), ("prop", "info"), ("titles", "MIPS")])
                .await?
        );
        Ok(())
    }
}

#[derive(Debug, clap::Subcommand)]
enum QuerySubcommand {
    All,
}
