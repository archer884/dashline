use std::env;

use clap::Parser;
use dashline_data::{model::NewPilot, service::PilotService};

#[derive(Clone, Debug, Parser)]
enum Args {
    AddPilot { name: String },
    ListPilots,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: &Args) -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    match args {
        Args::AddPilot { name } => add_pilot(&database_url, &name),
        Args::ListPilots => list_pilots(&database_url),
    }
}

fn add_pilot(database_url: &str, name: &str) -> anyhow::Result<()> {
    let service = PilotService::new(database_url)?;
    let new_pilot = NewPilot::new(name);
    service.insert(&new_pilot)?;
    Ok(())
}

fn list_pilots(database_url: &str) -> anyhow::Result<()> {
    let service = PilotService::new(database_url)?;
    let pilots = service.list()?;

    for pilot in pilots {
        println!("{pilot:?}");
    }

    Ok(())
}