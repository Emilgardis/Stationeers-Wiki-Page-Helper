mod commands;
pub mod enums;
pub mod stationpedia;

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use clap::{Parser, Subcommand};
use color_eyre::eyre;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: commands::Sub,
    #[arg(short)]
    verbose: bool,
    #[arg(global = true, long, default_value = get_cargo_workspace().join("Stationpedia.json").into_os_string())]
    stationpedia: PathBuf,
    #[arg(global = true, long, default_value = get_cargo_workspace().join("Enums.json").into_os_string())]
    enums: PathBuf,
    #[arg(global = true, long, default_value = get_cargo_workspace().join("config.toml").into_os_string())]
    config: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let pedia: stationpedia::Stationpedia =
        parse_json(&mut serde_json::Deserializer::from_reader(
            std::io::BufReader::new(std::fs::File::open(&cli.stationpedia)?),
        ))?;
    let enums: enums::Enums = parse_json(&mut serde_json::Deserializer::from_reader(
        std::io::BufReader::new(std::fs::File::open(&cli.enums)?),
    ))?;
    let config = std::fs::read_to_string(&cli.config)?.parse::<toml_edit::DocumentMut>()?;
    cli.run_subcommand(&pedia, &enums, &config)?;
    Ok(())
}

pub fn parse_json<'a, T: serde::Deserialize<'a>>(
    jd: impl serde::Deserializer<'a>,
) -> Result<T, color_eyre::Report> {
    let mut track = serde_path_to_error::Track::new();
    let pathd = serde_path_to_error::Deserializer::new(jd, &mut track);
    let mut fun = |path: serde_ignored::Path| {
        tracing::warn!(key=%path,"Found ignored key");
    };
    serde_ignored::deserialize(pathd, &mut fun).map_err(|e| {
        eyre::eyre!(
            "path: {track} | error = {e}",
            track = track.path().to_string(),
        )
    })
}

pub fn get_cargo_workspace() -> &'static Path {
    static WORKSPACE: OnceLock<PathBuf> = OnceLock::new();
    #[derive(Debug, serde_derive::Deserialize)]
    pub struct CargoMetadata {
        pub workspace_root: PathBuf,
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    WORKSPACE.get_or_init(|| {
        let sh = xshell::Shell::new().unwrap();
        sh.change_dir(manifest_dir);
        xshell::cmd!(sh, "cargo metadata --format-version 1 --no-deps")
            .read()
            .map_err(color_eyre::Report::from)
            .and_then(|s| serde_json::from_str::<CargoMetadata>(&s).map_err(Into::into))
            .unwrap()
            .workspace_root
    })
}
