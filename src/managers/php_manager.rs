use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;
use crate::configs::config_loader::PhpConfig;
use crate::configs::get_executables;

const PID_FILE: &str = "php.pid";

pub fn run_php(config: &PhpConfig) {
    // Verificar se o PHP já está rodando
    if Path::new(PID_FILE).exists() {
        println!("⚠️ O PHP já está rodando. Use 'php-connects php end' para encerrar antes de iniciar novamente.");
        return;
    }

    let php_bin = get_executables::get_php_path();
    println!("Iniciando PHP na porta {}...", config.port);

    let child = Command::new(php_bin)
    .args([
        "-S", &format!("localhost:{}", config.port),
        "-t", &config.root_dir
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
        println!("⚠️ Nenhum processo PHP ativo encontrado para encerrar.");
        return;
    }

    let pid_str = fs::read_to_string(PID_FILE).expect("Erro ao ler o arquivo de PID");
    let pid = pid_str.trim();

    println!("🛑 Encerrando processo PHP (PID: {})...", pid);

    let status = Command::new("taskkill")
        .args(["/F", "/PID", pid, "/T"])
        .output();

    match status {
        Ok(_) => {
            let _ = fs::remove_file(PID_FILE);
            println!("✅ PHP encerrado.");
        }
        Err(e) => println!("❌ Erro ao tentar encerrar o PHP: {}", e),
    }
}