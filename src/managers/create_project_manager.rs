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

    println!("🏗️  Criando novo projeto Fullstack: {}...", name);
    fs::create_dir_all(&project_root).expect("Falha ao criar pasta do projeto");

    // 1. Gerar o pconnect.cfg.toml (Local)
    generate_local_toml(&project_root, name);

    // 2. Criar o Back-end (Laravel) e limpar o desnecessário
    let backend_path = project_root.join("backend");
    create_laravel_backend(&backend_path, global);
    clean_laravel_boilerplate(&backend_path);

    // 3. Criar o Front-end
    let frontend_path = project_root.join("frontend");
    create_vue_frontend(&frontend_path, global);

    // 4. Configurar o .env do Laravel para o seu PostgreSQL local
    setup_env_file(&backend_path, name);

    println!("\n✨ Projeto '{}' criado com sucesso!", name);
    println!("👉 Digite 'cd {}' e depois 'pconnect run' para começar.", name);
}

fn create_laravel_backend(path: &Path, global: &GlobalConfig) {
    println!("🐘 Instalando Laravel Skeleton via Composer...");
    
    let php_bin = get_global_bin("php", &global.default_versions.php_version, "php.exe");
    let composer_path = get_global_bin("php", &global.default_versions.php_version, "composer.phar");

    if !php_bin.exists() {
        println!("❌ Erro: PHP não encontrado em {}. Rode 'pconnect install' primeiro.", php_bin.display());
        return;
    }

    if !composer_path.exists() {
        println!("❌ Erro: composer.phar não encontrado. Tente rodar 'pconnect install' novamente.");
        return;
    }

    let status = Command::new(&php_bin)
        .args([
            composer_path.to_str().unwrap(),
            "create-project",
            "laravel/laravel",
            path.to_str().unwrap(),
            "--quiet",
        ])
        .status();

    if let Err(e) = status {
        println!("⚠️ Erro ao executar o Composer: {}", e);
    }
}

fn create_vue_frontend(path: &Path, global: &GlobalConfig) {
    println!("⚡ Gerando Frontend com Bun (Vue + Vite)...");
    
    let bun_bin = get_global_bin("bun", &global.default_versions.bun_version, "bun.exe");
    let project_root = path.parent().unwrap();
    let folder_name = path.file_name().unwrap().to_str().unwrap();

    // Adicione o & aqui 
    let status = Command::new(&bun_bin) 
        .current_dir(project_root)
        .args([
            "create",
            "vite",
            folder_name,
            "--template", 
            "vue"
        ])
        .status();

    if let Err(e) = status {
        println!("❌ Erro ao rodar Bun: {}.", e);
        return;
    }

    println!("📦 Instalando dependências do Vue (node_modules)...");
    // E adicione o & aqui também
    let _ = Command::new(&bun_bin) 
        .current_dir(path) 
        .args(["install"])
        .status();
}

fn setup_env_file(backend_path: &Path, project_name: &str) {
    let env_path = backend_path.join(".env");
    
    if !env_path.exists() { 
        println!("⚠️ Aviso: Arquivo .env não encontrado no backend.");
        return; 
    }

    println!("🔧 Ajustando .env para PostgreSQL...");

    let content = fs::read_to_string(&env_path).unwrap_or_default();
    let db_name = format!("{}_db", project_name.to_lowercase());
    
    let new_content = content
        .replace("DB_CONNECTION=sqlite", "DB_CONNECTION=pgsql")
        .replace("# DB_HOST=127.0.0.1", "DB_HOST=127.0.0.1")
        .replace("DB_PORT=3306", "DB_PORT=5432")
        .replace("DB_DATABASE=laravel", &db_name)
        .replace("DB_USERNAME=root", "DB_USERNAME=postgres")
        .replace("DB_PASSWORD=", "DB_PASSWORD=admin");

    fs::write(env_path, new_content).expect("Falha ao salvar .env");
}

/// Função inteligente para localizar binários lidando com subpastas de extração
fn get_global_bin(name: &str, version: &str, executable: &str) -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("USERPROFILE não encontrado");
    let base_dir = PathBuf::from(home)
        .join(".pconnect")
        .join(format!("{}-{}", name, version));

    // 1. Tenta o caminho óbvio (raiz da pasta da versão)
    let direct_path = base_dir.join(executable);
    if direct_path.exists() {
        return direct_path;
    }

    // 2. Caso PostgreSQL (costuma estar em pgsql/bin/)
    if name == "postgres" {
        let pgsql_bin = base_dir.join("pgsql").join("bin").join(executable);
        if pgsql_bin.exists() { return pgsql_bin; }
    }

    // 3. Caso Bun ou outros com "matrioska" (procura em subpastas imediatas)
    if let Ok(entries) = fs::read_dir(&base_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                // Tenta achar o executável dentro da subpasta (ex: bun-windows-x64/bun.exe)
                let attempt = entry.path().join(executable);
                if attempt.exists() { return attempt; }
                
                // Caso específico do Bun que pode ter bin/bun.exe dentro da subpasta
                let attempt_bin = entry.path().join("bin").join(executable);
                if attempt_bin.exists() { return attempt_bin; }
            }
        }
    }

    // Se nada funcionar, retorna o caminho direto para que o erro de .exists() apareça no chamador
    direct_path
}

fn generate_local_toml(project_path: &Path, project_name: &str) {
    let toml_path = project_path.join("pconnect.cfg.toml");
    
    let default_content = format!(
    r#"[ports]
    php_port = 8000
    laravel_port = 8001
    postgresql_port = 5432
    vue_port = 5173

    [paths]
    backend_dir = "./backend"
    frontend_dir = "./frontend"

    [postgresql]
    db = "{0}_db"
    host = "localhost"
    user = "postgres"
    pass = "admin"
    "#, project_name.to_lowercase());

    fs::write(toml_path, default_content)
        .expect("❌ Erro ao criar o arquivo pconnect.cfg.toml");
}

fn clean_laravel_boilerplate(backend_path: &Path) {
    println!("🧹 Limpando boilerplate desnecessário do Laravel...");

    // 1. Remove arquivos de frontend do backend (já que temos a pasta /frontend)
    let to_remove = [
        "package.json",
        "vite.config.js",
        "resources/css",
        "resources/js",
        "resources/views/welcome.blade.php",
    ];

    for item in to_remove {
        let path = backend_path.join(item);
        if path.is_dir() {
            let _ = fs::remove_dir_all(path);
        } else {
            let _ = fs::remove_file(path);
        }
    }

    // 2. Cria uma rota de API limpa 
    let routes_path = backend_path.join("routes/web.php");
    let clean_route = "<?php\n\nuse Illuminate\\Support\\Facades\\Route;\n\nRoute::get('/', fn() => ['status' => 'PConnect API Online']);";
    let _ = fs::write(routes_path, clean_route);

    println!("✅ Backend Laravel simplificado.");
}
