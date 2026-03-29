use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::configs::pconnect_cfg::GlobalConfig;

pub fn create_project(name: &str, global: &GlobalConfig) {
    let project_root = std::env::current_dir().unwrap().join(name);

    if project_root.exists() {
        println!("❌ Erro: A pasta '{}' já existe.", name);
        return;
    }

    println!("🏗️ Criando novo projeto Fullstack: {}...", name);
    fs::create_dir_all(&project_root).expect("Falha ao criar pasta do projeto");

    // 1. Gerar o pconnect.cfg.toml (Local)
    generate_local_toml(&project_root, name);

    // 2. Criar o Back-end (Laravel)
    let backend_path = project_root.join("backend");
    create_laravel_backend(&backend_path, global);

    // 3. Criar o Front-end (Vue + Vite via Bun)
    let frontend_path = project_root.join("frontend");
    create_vue_frontend(&frontend_path, global);

    // 4. Configurar o .env do Laravel para o seu MySQL local
    setup_env_file(&backend_path, name);

    println!("\n✨ Projeto '{}' criado com sucesso!", name);
    println!("👉 Digite 'cd {}' e depois 'pconnect run' para começar.", name);
}

fn create_laravel_backend(path: &Path, global: &GlobalConfig) {
    println!("🐘 Instalando Laravel Skeleton...");
    
    let php_bin = get_global_bin("php", &global.default_versions.php_version, "php.exe");
    // O composer.phar geralmente fica na raiz da pasta do PHP que você instalou
    let composer_path = get_global_bin("php", &global.default_versions.php_version, "composer.phar");

    if !php_bin.exists() {
        println!("❌ Erro: PHP não encontrado em {}. Rode 'pconnect install' primeiro.", php_bin.display());
        return;
    }

    let status = Command::new(&php_bin)
        .args([
            composer_path.to_str().unwrap(),
            "create-project",
            "laravel/laravel",
            path.to_str().unwrap(),
        ])
        .status();

    if let Err(e) = status {
        println!("⚠️ Erro ao executar o Composer: {}", e);
    }
}

fn create_vue_frontend(path: &Path, global: &GlobalConfig) {
    println!("⚡ Gerando Frontend com Bun (Vue + Vite)...");
    
    // Verificamos se usamos o Bun global do sistema ou o que o pconnect instalou
    let bun_bin = if global.installations.bun_install {
        get_global_bin("bun", &global.default_versions.bun_version, "bun-windows-x64/bun.exe")
    } else {
        PathBuf::from("bun") // Tenta usar o que já está no PATH do usuário
    };

    let status = Command::new(bun_bin)
        .args([
            "create",
            "vite",
            path.to_str().unwrap(),
            "--template", "vue"
        ])
        .status();

    if let Err(e) = status {
        println!("❌ Erro ao rodar Bun: {}. Verifique se ele está instalado corretamente.", e);
    }
}

fn setup_env_file(backend_path: &Path, project_name: &str) {
    let env_path = backend_path.join(".env");
    
    // O Laravel cria o .env automaticamente no create-project, mas vamos garantir
    if !env_path.exists() { return; }

    println!("🔧 Ajustando .env para conexão local...");

    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Ajustes para as portas e dados que definimos no TOML local
    let new_content = content
        .replace("DB_CONNECTION=sqlite", "DB_CONNECTION=mysql") // Laravel 11+ usa sqlite por padrão
        .replace("DB_HOST=127.0.0.1", "DB_HOST=localhost")
        .replace("DB_PORT=3306", "DB_PORT=8082")
        .replace("DB_DATABASE=laravel", &format!("DB_DATABASE={}_db", project_name))
        .replace("DB_USERNAME=root", "DB_USERNAME=root")
        .replace("DB_PASSWORD=", "DB_PASSWORD=admin");

    fs::write(env_path, new_content).expect("Falha ao salvar .env");
}

fn get_global_bin(name: &str, version: &str, subpath: &str) -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("USERPROFILE não encontrado");
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("{}-{}", name, version))
        .join(subpath)
}

fn generate_local_toml(project_path: &Path, project_name: &str) {
    let toml_path = project_path.join("pconnect.cfg.toml");
    
    let default_content = format!(
    r#"[ports]
    php_port = 8080
    laravel_port = 8081
    mysql_port = 8082
    vue_port = 8083

    [paths]
    backend_dir = "./backend"
    frontend_dir = "./frontend"

    [mysql]
    db = "{0}_db"
    host = "localhost"
    user = "root"
    pass = "admin"
    "#, project_name);

    fs::write(toml_path, default_content)
        .expect("❌ Erro ao criar o arquivo pconnect.cfg.toml");
    
    println!("📝 Configuração local gerada com sucesso.");
}