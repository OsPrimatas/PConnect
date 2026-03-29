use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use crate::configs::pconnect_cfg::{ProjectConfig, GlobalConfig};

const PID_FILE: &str = ".pconnect_bun.pid";

fn get_bun_executable(global: &GlobalConfig) -> PathBuf {
    // Se no global.installations.bun_install for false, tentamos usar o do sistema
    if !global.installations.bun_install {
        return PathBuf::from("bun"); 
    }

    // Caso contrário, busca na pasta .pconnect usando a versão global
    let home = std::env::var("USERPROFILE").expect("❌ Erro: USERPROFILE não encontrado");
    PathBuf::from(home)
        .join(".pconnect")
        .join(format!("bun-{}", global.versions.bun_version))
        .join("bun-windows-x64") 
        .join("bun.exe")
}

pub fn run_vue(project: &ProjectConfig, global: &GlobalConfig) {
    if Path::new(PID_FILE).exists() {
        println!("⚠️  O servidor Frontend (Bun/Vite) já parece estar rodando.");
        return;
    }

    let bun_bin = get_bun_executable(global);
    let frontend_path = Path::new(&project.paths.frontend_dir);

    // Validação de existência do binário (apenas se for o interno)
    if global.installations.bun_install && !bun_bin.exists() {
        println!("❌ Erro: Bun não encontrado em {}. Rode 'pconnect install' primeiro.", bun_bin.display());
        return;
    }

    // 1. Auto-Install: Verifica se node_modules existe na pasta definida no TOML local
    let node_modules = frontend_path.join("node_modules");
    if !node_modules.exists() {
        println!("📦 node_modules não encontrada em {}. Instalando dependências...", project.paths.frontend_dir);
        
        let install_status = Command::new(&bun_bin)
            .arg("install")
            .current_dir(frontend_path)
            .status();

        match install_status {
            Ok(s) if s.success() => println!("✅ Dependências instaladas."),
            _ => {
                println!("❌ Erro ao instalar dependências do Frontend.");
                return;
            }
        }
    }

    println!("⚡ Iniciando Vue + Vite na porta {}...", project.ports.vue_port);

    // 2. Execução do Servidor Dev
    let child = Command::new(bun_bin)
        .args(["run", "dev", "--port", &project.ports.vue_port.to_string()])
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