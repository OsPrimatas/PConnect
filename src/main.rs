pub mod configs;
pub mod managers;

use clap::Parser;
use crate::configs::php_connects_cmd::Cli;
use crate::configs::php_connects_cmd::Commands;

fn main() {
    let cli = Cli::parse();
    let config = configs::php_connects_cfg::load_config();

    match &cli.command {
        Commands::Install => {
            managers::download_manager::install_all(&config);
        }
        Commands::Create(args) => {
            managers::create_project_manager::create_project(&args.name, &config);
        }
        Commands::Run => {
            println!("🚀 Iniciando ecossistema de desenvolvimento...");

            // Inicia o php
            managers::php_manager::run_php(&config);
            // Inicia o MySQL
            managers::mysql_manager::run_mysql(&config);
            // Inicia o Bun (Vite/Vue)
            managers::bun_manager::run_vue(&config);

            println!("\n✨ Stack iniciada com sucesso!");
            println!("🌐 Backend: http://localhost:{}", config.ports.php_port);
            println!("🎨 Frontend: http://localhost:{}", config.ports.vue_port);
            println!("\nPresione Ctrl+C para encerrar ou use 'pconnect stop' em outro terminal.");

            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        Commands::Stop => {
            managers::php_manager::stop_php();
            managers::mysql_manager::stop_mysql();
            managers::bun_manager::stop_vue();
        }
    }
}
