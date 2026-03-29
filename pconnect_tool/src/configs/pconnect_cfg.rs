use serde::Deserialize;
use std::fs;

// --- CONFIGURAÇÃO GLOBAL (Fica junto ao .exe) ---
#[derive(Debug, Deserialize, Clone)]
pub struct GlobalConfig {
    pub versions: Versions,
    pub installations: Installations,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Versions {
    pub bun_version: String,
    pub php_version: String,
    pub postgresql_version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Installations {
    pub bun_install: bool,
    pub vue_install: bool,
    pub laravel_install: bool,
    pub php_install: bool,
    pub postgresql_install: bool,
}

// --- CONFIGURAÇÃO LOCAL (Fica na raiz do projeto criado) ---
#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub ports: Ports,
    pub paths: Paths,
    pub postgresql: PostgreSQL,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ports {
    pub php_port: u16,
    pub laravel_port: u16,
    pub postgresql_port: u16,
    pub vue_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Paths {
    pub backend_dir: String,
    pub frontend_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostgreSQL {
    pub db: String,
    pub host: String,
    pub user: String,
    pub pass: String,
}

// --- FUNÇÕES DE CARREGAMENTO ---

/// Carrega o arquivo global que está na mesma pasta do executável
pub fn load_global_config() -> GlobalConfig {
    let mut exe_path = std::env::current_exe().expect("Falha ao obter caminho do executável");
    exe_path.pop();
    let config_path = exe_path.join("pconnect_global.cfg.toml");

    let content = fs::read_to_string(&config_path)
        .expect("❌ Erro: Arquivo 'pconnect_global.cfg.toml' não encontrado na pasta do executável.");

    toml::from_str(&content).expect("❌ Erro ao processar pconnect_global.cfg.toml")
}

/// Carrega o arquivo local do projeto
pub fn load_project_config() -> ProjectConfig {
    let content = fs::read_to_string("pconnect.cfg.toml")
        .expect("❌ Erro: 'pconnect.cfg.toml' não encontrado. Você está na pasta do projeto?");

    toml::from_str(&content).expect("❌ Erro ao processar pconnect.cfg.toml")
}