use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};

const DEFAULT_PHP: &str = "8.5.4";
const DEFAULT_BUN: &str = "1.3.6";

pub fn create_project(name: &str) {
    let project_root = std::env::current_dir().unwrap().join(name);

    if project_root.exists() {
        println!("❌ Erro: A pasta '{}' já existe.", name);
        return;
    }

    println!("🏗️  Criando novo projeto fullstack: {}...", name);
    fs::create_dir_all(&project_root).expect("Falha ao criar pasta do projeto");

    // 1. Criar o arquivo de configuração padrão
    generate_default_toml(&project_root, name);

    // 2. Criar o Back-end (Laravel)
    create_laravel_backend(&project_root.join("backend"));

    // 3. Criar o Front-end (Vue + Vite via Bun)
    create_vue_frontend(&project_root.join("frontend"));

    // 4. Configurar o .env do Laravel para o seu MySQL local
    setup_env_file(&project_root.join("backend"), name);

    println!("\n✨ Projeto '{}' criado com sucesso!", name);
    println!("👉 Digite 'cd {}' e depois 'pconnect run' para começar.", name);
}

fn create_laravel_backend(path: &Path) {
    println!("🐘 Instalando Laravel Skeleton...");
    
    let php_bin = get_global_bin("php", DEFAULT_PHP, "php.exe");
    let composer_path = get_global_bin("php", DEFAULT_PHP, "composer.phar");

    if !php_bin.exists() {
        println!("❌ Erro: PHP não encontrado. Rode 'pconnect install' primeiro.");
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
        println!("⚠️ Erro ao criar Laravel: {}. Verifique se o composer.phar está em {}", e, composer_path.display());
    }
}

fn create_vue_frontend(path: &Path) {
    println!("⚡ Gerando Frontend com Bun (Vue + Vite)...");
    
    let bun_bin = get_global_bin("bun", DEFAULT_BUN, "bun-windows-x64/bun.exe");

    if !bun_bin.exists() {
        println!("❌ Erro: Bun não encontrado. Rode 'pconnect install' primeiro.");
        return;
    }

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

fn setup_env_file(backend_path: &Path, project_name: &str) {
    let env_path = backend_path.join(".env");
    if !env_path.exists() { return; }

    println!("🔧 Configurando .env para MySQL local...");

    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Substituições baseadas nos padrões do generate_default_toml
    let new_content = content
        .replace("DB_HOST=127.0.0.1", "DB_HOST=localhost")
        .replace("DB_PORT=3306", "DB_PORT=8082")
        .replace("DB_DATABASE=laravel", &format!("DB_DATABASE={}_db", project_name))
        .replace("DB_USERNAME=root", "DB_USERNAME=root")
        .replace("DB_PASSWORD=", "DB_PASSWORD=admin");

    fs::write(env_path, new_content).expect("Falha ao salvar .env configurado");
}

fn get_global_bin(name: &str, version: &str, subpath: &str) -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("USERPROFILE não definido");
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("{}-{}", name, version))
        .join(subpath)
}

fn generate_default_toml(project_path: &Path, project_name: &str) {
    let toml_path = project_path.join("php_connects.cfg.toml");
    
    let default_content = format!(
    r#"[project]
    name = "{0}"
    edition = "1.0"

    [stack]
    frontend = "vue"
    backend = "laravel"
    database = "mysql"

    [ports]
    php_port = 8080
    laravel_port = 8081
    mysql_port = 8082
    vue_port = 8083

    [paths]
    backend_dir = "./backend"
    frontend_dir = "./frontend"

    [versions]
    bun_version = "{1}"
    vue_version = "3.5.31"
    laravel_version = "13"
    php_version = "{2}"
    mysql_version = "8.0.46"

    [mysql]
    db = "{0}_db"
    host = "localhost"
    user = "root"
    pass = "admin"
    "#, project_name, DEFAULT_BUN, DEFAULT_PHP);

    fs::write(toml_path, default_content)
        .expect("❌ Erro ao criar o arquivo php_connects.cfg.toml inicial");
    
    println!("📝 Arquivo de configuração gerado: php_connects.cfg.toml");
}