use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pconnect")]
#[command(about = "Gerenciador minimalista de PHP e MySQL", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Inicia PHP e MySQL simultaneamente
    Start,
    /// Encerra todos os processos ativos
    End,
    /// Comandos específicos para o PHP
   Php {
        #[command(subcommand)]
        action: ServiceAction,
    },
    Mysql {
        #[command(subcommand)]
        action: ServiceAction,
    },
}

#[derive(Subcommand)]
pub enum ServiceAction {
    Start,
    End,
}