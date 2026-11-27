#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-libreoffice-impress", about = "LibreOffice Impress presentation integration for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-libreoffice-impress: no entries configured."),
        Commands::Status => println!("sigmaos-libreoffice-impress: ready."),
    }
}
