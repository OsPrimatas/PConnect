pub mod configs;
pub mod managers;

use clap::Parser;
use crate::configs::pconnect_cmd::{Cli, Commands};
use crate::configs::pconnect_cfg;

fn main() {
    let cli = Cli::parse();

    // 1. Carregamos a Configuração Global
    let global_config = pconnect_cfg::load_global_config();

    match &cli.command {
        // --- COMANDO INSTALL ---
        Commands::Install => {
            println!("🛠️  Iniciando instalação global do ecossistema pconnect...");
            managers::download_manager::install_all(&global_config); 
        }

        // --- COMANDO CREATE ---
        Commands::Create(args) => {
            // Passamos o nome do projeto e a config global para o manager de criação
            managers::create_project_manager::create_project(&args.name, &global_config);
        }

        // --- COMANDO RUN ---
        Commands::Run => {
            // Carrega o pconnect.cfg.toml da pasta onde o usuário está
            let project_config = pconnect_cfg::load_project_config();
            
            // Delega para o run_manager orquestrar os processos e o Ctrl+C
            managers::run_manager::start_orchestrator(&project_config, &global_config);
        }

        // --- COMANDO STOP ---
        Commands::Stop => { 
            println!("🛑 Encerrando todos os processos do ecossistema...");
            
            // Chama os comandos de parada individualmente
            managers::php_manager::stop_php();
            managers::bun_manager::stop_vue();
            managers::postgresql_manager::stop_postgresql();
            
            println!("✅ Todos os serviços foram finalizados.");
        }
    
        // --- COMANDO SHELL ---
        Commands::Shell => {
            managers::shell_manager::spawn_shell(&global_config);
        }
    }
}