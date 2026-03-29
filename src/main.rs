pub mod configs;
pub mod managers;

use clap::Parser;
use crate::configs::pconnect_cmd::{Cli, Commands};
use crate::configs::pconnect_cfg;

fn main() {
    let cli = Cli::parse();

    // 1. Carregamos a Configuração Global (onde ficam as versões e caminhos da .pconnect)
    let global_config = pconnect_cfg::load_global_config();

    match &cli.command {
        // --- COMANDO INSTALL ---
        Commands::Install => {
            println!("🛠️  Iniciando instalação global do ecossistema pconnect...");
            managers::download_manager::install_all(&global_config); 
        }

        // --- COMANDO CREATE ---
        Commands::Create(args) => {
            managers::create_project_manager::create_project(&args.name, &global_config);
        }

        // --- COMANDOS QUE DEPENDEM DO PROJETO LOCAL ---
        Commands::Run | Commands::Stop => {
            // Carrega o pconnect.cfg.toml da pasta atual do projeto
            let project_config = pconnect_cfg::load_project_config();
            
            match &cli.command {
                Commands::Run => {
                    println!("🚀 Iniciando ecossistema de desenvolvimento...");

                    // 1. Inicia o PostgreSQL (Banco precisa subir antes do backend)
                    managers::postgresql_manager::run_postgresql(&project_config, &global_config);
                    
                    // 2. Inicia o PHP (Laravel Artisan)
                    managers::php_manager::run_php(&project_config, &global_config);
                    
                    // 3. Inicia o Bun (Vite/Vue)
                    managers::bun_manager::run_vue(&project_config, &global_config);

                    println!("\n✨ Stack iniciada com sucesso!");
                    println!("🌐 Laravel:  http://localhost:{}", project_config.ports.laravel_port);
                    println!("🎨 Frontend: http://localhost:{}", project_config.ports.vue_port);
                    println!("🐘 PostgreSQL: Porta {}", project_config.ports.postgresql_port); // Ajustado de MySQL para PostgreSQL
                    
                    println!("\nPressione Ctrl+C para encerrar ou use 'pconnect stop' em outro terminal.");

                    // Mantém o processo principal vivo
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
                Commands::Stop => { 
                    println!("🛑 Encerrando todos os serviços do projeto...");
                    
                    // Ordem de parada: App -> DB
                    managers::php_manager::stop_php();
                    managers::bun_manager::stop_vue();
                    managers::postgresql_manager::stop_postgresql();
                    
                    println!("✅ Todos os processos foram finalizados.");
                },
                _ => unreachable!(),
            }
        }
    
        Commands::Shell => {
            managers::shell_manager::spawn_shell(&global_config);
        }
    }
}