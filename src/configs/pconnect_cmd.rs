use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pconnect")]
#[command(about = "Um gerenciador de projetos que integrará PHP, MySQL, Bun e Laravel", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install,
    Create(CreateArgs),
    Run,
    Stop,
    Shell,
}

#[derive(Parser)]
pub struct CreateArgs {
    pub name: String,
    
    #[arg(short, long, default_value = "fullstack")]
    pub template: String,
}