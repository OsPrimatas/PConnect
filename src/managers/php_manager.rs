use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use crate::configs::php_connects_cfg::Config;

const PID_FILE: &str = ".php.pid";

fn get_php_executable_path(version: &str) -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("Não foi possível encontrar a pasta do usuário.");
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("php-{}", version))
        .join("php.exe")
}

pub fn run_php(config: &Config) {
    // Verificar se o PHP já está rodando
    if Path::new(PID_FILE).exists() {
        println!("⚠️ O PHP já está rodando. Use 'php-connects php end' para encerrar antes de iniciar novamente.");
        return;
    }

    let php_bin = get_php_executable_path(&config.versions.php_version);
    println!("Iniciando PHP na porta {}...", config.ports.php_port);

    if !php_bin.exists() {
        println!("❌ Erro: PHP {} não encontrado em: {}", config.versions.php_version, php_bin.display());
        println!("Execute 'pconnect setup' ou 'pconnect install' primeiro.");
        return;
    }

    println!("🐘 Iniciando PHP {} na porta {}...", config.versions.php_version, config.ports.php_port);

    let child = Command::new(php_bin)
        .args([
            "-S", &format!("localhost:{}", config.ports.php_port),
            "-t", &config.paths.backend_dir,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(process) => {
            let pid = process.id();
            fs::write(PID_FILE, pid.to_string()).expect("Não foi possível salvar o PID do PHP");
            println!("✅ PHP iniciado com sucesso (PID: {})", pid);
        }
        Err(e) => println!("❌ Erro ao iniciar o PHP: {}", e),
    }
}

pub fn stop_php() {
    if !Path::new(PID_FILE).exists() {
        println!("⚠️ Nenhum processo PHP ativo encontrado.");
        return;
    }

    let pid = fs::read_to_string(PID_FILE).expect("Erro ao ler PID");
    
    println!("🛑 Encerrando PHP (PID: {})...", pid.trim());

    let status = Command::new("taskkill")
        .args(["/F", "/PID", pid.trim(), "/T"])
        .output();

    match status {
        Ok(_) => {
            let _ = fs::remove_file(PID_FILE);
            println!("✅ PHP encerrado e porta liberada.");
        }
        Err(e) => println!("❌ Erro ao encerrar: {}", e),
    }
}