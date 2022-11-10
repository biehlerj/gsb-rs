use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Import GNOME settings
    Import,
    /// Export GNOME settings
    Export,
    // Help,
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Command::Import => import_settings(),
        Command::Export => export_settings(),
    }
}

fn import_settings() {
    println!("Hello from import!")
}

fn export_settings() {
    println!("Hello from export")
}
