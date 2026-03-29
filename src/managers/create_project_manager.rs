use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::configs::php_connects_cfg::Config;

pub fn create_project(name: &str, config: &Config) {
    let project_root = std::env::current_dir().unwrap().join(name);

    if project_root.exists() {
        println!("❌ Erro: A pasta '{}' já existe.", name);
        return;
    }

    println!("Criando novo projeto fullstack: {}...", name);
    fs::create_dir_all(&project_root).expect("Falha ao criar pasta do projeto");

    // 1. Criar o Back-end (Laravel)
    create_laravel_backend(&project_root.join(&config.paths.backend_dir), config);

    // 2. Criar o Front-end (Vue + Vite via Bun)
    create_vue_frontend(&project_root.join(&config.paths.frontend_dir), config);

    // 3. Configurar o .env do Laravel para o seu MySQL local
    setup_env_file(&project_root.join(&config.paths.backend_dir), config);

    println!("\n✨ Projeto '{}' criado com sucesso!", name);
    println!("👉 Digite 'cd {}' e depois 'pconnect run' para começar.", name);
}

fn create_laravel_backend(path: &Path, config: &Config) {
    println!("🐘 Instalando Laravel Skeleton...");
    
    let php_bin = get_global_bin("php", &config.versions.php_version, "php.exe");
    
    let status = Command::new(&php_bin)
        .args([
            "composer.phar",
            "create-project",
            "laravel/laravel",
            path.to_str().unwrap(),
        ])
        .status();

    if let Err(e) = status {
        println!("⚠️ Erro ao criar Laravel: {}. Verifique se o composer.phar está na pasta global.", e);
    }
}

fn create_vue_frontend(path: &Path, config: &Config) {
    println!("⚡ Gerando Frontend com Bun (Vue + Vite)...");
    
    let bun_bin = get_global_bin("bun", &config.versions.bun_version, "bun-windows-x64/bun.exe");

    let status = Command::new(bun_bin)
        .args([
            "create",
            "vite",
            path.to_str().unwrap(),
            "--template", "vue"
        ])
        .status();

    if let Err(e) = status {
        println!("❌ Erro ao rodar Bun: {}", e);
    }
}

fn setup_env_file(backend_path: &Path, config: &Config) {
    let env_path = backend_path.join(".env");
    if !env_path.exists() { return; }

    println!("🔧 Configurando .env para MySQL local...");

    let mut content = fs::read_to_string(&env_path).unwrap_or_default();

    // Substituições mágicas para conectar ao seu MySQL local (.cache/mysql)
    content = content.replace("DB_HOST=127.0.0.1", &format!("DB_HOST={}", config.mysql.host));
    content = content.replace("DB_PORT=3306", &format!("DB_PORT={}", config.ports.mysql_port));
    content = content.replace("DB_DATABASE=laravel", &format!("DB_DATABASE={}", config.mysql.db));
    content = content.replace("DB_USERNAME=root", &format!("DB_USERNAME={}", config.mysql.user));
    content = content.replace("DB_PASSWORD=", &format!("DB_PASSWORD={}", config.mysql.pass));

    fs::write(env_path, content).expect("Falha ao salvar .env configurado");
}

// Função auxiliar para pegar os binários globais
fn get_global_bin(name: &str, version: &str, subpath: &str) -> PathBuf {
    let home = std::env::var("USERPROFILE").unwrap();
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("{}-{}", name, version))
        .join(subpath)
}