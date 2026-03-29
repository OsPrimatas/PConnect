use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use crate::configs::pconnect_cfg::{ProjectConfig, GlobalConfig};

const PID_FILE: &str = ".pconnect_php.pid";

fn get_php_executable(global: &GlobalConfig) -> PathBuf {
    // Se não for para usar o instalado pelo pconnect, assume que está no PATH
    if !global.installations.php_install {
        return PathBuf::from("php.exe");
    }

    let home = std::env::var("USERPROFILE").expect("❌ Erro: USERPROFILE não encontrado");
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("php-{}", global.default_versions.php_version))
        .join("php.exe")
}

pub fn run_php(project: &ProjectConfig, global: &GlobalConfig) {
    if Path::new(PID_FILE).exists() {
        println!("⚠️  O servidor PHP já parece estar rodando.");
        return;
    }

    let php_bin = get_php_executable(global);
    let backend_path = Path::new(&project.paths.backend_dir);

    if global.installations.php_install && !php_bin.exists() {
        println!("❌ Erro: PHP não encontrado em {}. Rode 'pconnect install'.", php_bin.display());
        return;
    }

    // Verifica se o artisan existe antes de tentar rodar
    let artisan_path = backend_path.join("artisan");
    if !artisan_path.exists() {
        println!("❌ Erro: Arquivo 'artisan' não encontrado em {}. Isso é um projeto Laravel?", backend_path.display());
        return;
    }

    println!("🐘 Iniciando Laravel (PHP) na porta {}...", project.ports.laravel_port);

    // Usamos o 'artisan serve' para garantir que o Laravel funcione perfeitamente
    let child = Command::new(php_bin)
        .args([
            "artisan", 
            "serve", 
            "--host=127.0.0.1", 
            &format!("--port={}", project.ports.laravel_port)
        ])
        .current_dir(backend_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(process) => {
            let pid = process.id();
            fs::write(PID_FILE, pid.to_string()).expect("Não foi possível salvar o PID do PHP");
            println!("✅ PHP/Laravel pronto! (PID: {})", pid);
        }
        Err(e) => println!("❌ Erro ao iniciar o PHP: {}", e),
    }
}

pub fn stop_php() {
    if !Path::new(PID_FILE).exists() {
        return;
    }

    let pid = fs::read_to_string(PID_FILE).unwrap_or_default();
    
    let _ = Command::new("taskkill")
        .args(["/F", "/PID", pid.trim(), "/T"])
        .output();

    let _ = fs::remove_file(PID_FILE);
    println!("✅ Servidor PHP encerrado.");
}