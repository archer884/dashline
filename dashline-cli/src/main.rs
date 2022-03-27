use std::env;

use clap::Parser;
use dashline_data::{model::NewPilot, service::PilotService};

#[derive(Clone, Debug, Parser)]
enum Args {
    AddPilot { name: String },
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
    }
}

fn add_pilot(connection: &str, name: &str) -> anyhow::Result<()> {
    let service = PilotService::new(connection)?;
    let new_pilot = NewPilot::new(name);
    service.insert(&new_pilot)?;
    Ok(())
}
