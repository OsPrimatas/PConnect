pub mod managers;
pub mod configs;
pub mod general_commands;
pub mod hosts;

use general_commands::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let config = configs::config_loader::load();

    match &cli.command {
        general_commands::Commands::Start => {
            println!("Iniciando O PHP & MySQL...");
            
            // Iniciar o php + mysql
            managers::php_manager::run_php(&config.php);
            managers::mysql_manager::run_mysql();
        }
        general_commands::Commands::End => {
            println!("Encerrando O PHP & MySQL...");

            // Encerrar o php + mysql
            managers::php_manager::stop_php();
            managers::mysql_manager::stop_mysql();
        }
        general_commands::Commands::Php { action } => match action {
            general_commands::ServiceAction::Start => {
                println!("Iniciando O PHP...");

                // Iniciar o php
                managers::php_manager::run_php(&config.php);
            }
            general_commands::ServiceAction::End => {
                println!("Encerrando O PHP...");

                // Encerrar o php
                managers::php_manager::stop_php();
            }
        },
        general_commands::Commands::Mysql { action } => match action {
            general_commands::ServiceAction::Start => {
                println!("Iniciando O MySQL...");

                // Iniciar o mysql
                managers::mysql_manager::run_mysql();
            }
            general_commands::ServiceAction::End => {
                println!("Encerrando O MySQL...");

                // Encerrar o mysql
                managers::mysql_manager::stop_mysql();
            }
        }
    }
}
