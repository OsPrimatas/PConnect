use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use crate::configs::php_connects_cfg::Config;

const PID_FILE: &str = ".bun.pid";

fn get_bun_executable_path(version: &str) -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("❌ Erro: USERPROFILE não encontrado");
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("bun-{}", version))
        .join("bun-windows-x64") 
        .join("bun.exe")
}

pub fn run_vue(config: &Config) {
    if Path::new(PID_FILE).exists() {
        println!("⚠️ O servidor Frontend (Bun/Vite) já parece estar rodando.");
        return;
    }

    let bun_bin = get_bun_executable_path(&config.versions.bun_version);
    let frontend_path = Path::new(&config.paths.frontend_dir);

    if !bun_bin.exists() {
        println!("❌ Erro: Bun não encontrado. Rode 'pconnect install' primeiro.");
        return;
    }

    // 1. Auto-Install: Se não houver node_modules, o CLI instala sozinho
    let node_modules = frontend_path.join("node_modules");
    if !node_modules.exists() {
        println!("📦 node_modules não encontrada em {}. Instalando dependências...", config.paths.frontend_dir);
        
        let install_status = Command::new(&bun_bin)
            .arg("install")
            .current_dir(frontend_path) // Roda dentro da pasta do Vue
            .status()
            .expect("Falha ao executar bun install");

        if !install_status.success() {
            println!("❌ Erro ao instalar dependências do Frontend.");
            return;
        }
    }

    println!("⚡ Iniciando Vue + Vite com Bun na porta {}...", config.ports.vue_port);

    let child = Command::new(bun_bin)
        .args(["run", "dev", "--port", &config.ports.vue_port.to_string()])
        .current_dir(frontend_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(process) => {
            let pid = process.id();
            fs::write(PID_FILE, pid.to_string()).expect("Não foi possível salvar o PID do Bun");
            println!("✅ Frontend pronto! (PID: {})", pid);
        }
        Err(e) => println!("❌ Erro ao iniciar o servidor de desenvolvimento: {}", e),
    }
}

pub fn stop_vue() {
    if !Path::new(PID_FILE).exists() {
        return;
    }

    let pid = fs::read_to_string(PID_FILE).unwrap_or_default();
    
    let _ = Command::new("taskkill")
        .args(["/F", "/PID", pid.trim(), "/T"])
        .output();

    let _ = fs::remove_file(PID_FILE);
    println!("✅ Servidor Frontend encerrado.");
}