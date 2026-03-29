use crate::configs::pconnect_cfg::*;
use crate::managers::{postgresql_manager, php_manager, bun_manager};

pub fn start_orchestrator(project: &ProjectConfig, global: &GlobalConfig) {
    // 1. Inicia os serviços
    postgresql_manager::run_postgresql(project, global);
    php_manager::run_php(project, global);
    bun_manager::run_vue(project, global);

    println!("\n🚀 Controle de Estoque Online!");
    println!("💡 Pressione Ctrl+C para encerrar com segurança.\n");

    // 2. Configura o Capturador de Ctrl+C
    ctrlc::set_handler(move || {
        println!("\n🛑 Sinal de interrupção recebido! Limpando processos...");
        
        // Chama as funções de stop que criamos nos managers
        postgresql_manager::stop_postgresql();
        php_manager::stop_php();
        bun_manager::stop_vue();

        println!("✅ Ambiente encerrado com sucesso. Até logo!");
        std::process::exit(0);
    }).expect("Erro ao configurar o manipulador de Ctrl+C");

    // 3. Mantém o loop principal vivo
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}