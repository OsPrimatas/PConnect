use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config{ 
    pub project: Project,
    pub stack: Stack,
    pub ports: Ports,
    pub paths: Paths,
    pub versions: Versions,
    pub mysql: Mysql,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Project{ 
    pub name: String,
    pub edition: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stack{ 
    pub frontend: String,
    pub backend: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ports{ 
    pub php_port: u16,
    pub laravel_port: u16,
    pub mysql_port: u16,
    pub vue_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Paths{
    pub backend_dir: String,
    pub frontend_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Versions{
    pub bun_version: String,
    pub vue_version: String,
    pub laravel_version: String,
    pub php_version: String,
    pub mysql_version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mysql{
    pub db: String,
    pub host: String,
    pub user: String,
    pub pass: String,
}

pub fn load_config() -> Config {
    let content = fs::read_to_string("php_connects.cfg.toml").expect("❌ Erro! \n não foi possível encontrar o arquivo de configurações 'php_connects.cfg.toml'.");

    let config: Config = toml::from_str(&content)
        .expect("❌ Erro: Falha ao processar o TOML. Verifique a sintaxe.");

    validate_versions(&config);
    config
}

fn validate_versions(config: &Config) {
    if config.versions.php_version.as_str() < "8.5.4" {
        panic!("🛑 Erro: Este CLI exige PHP 8.5.4 ou superior. \n Versão detectada: {}", config.versions.php_version);
    }
    
    if config.versions.laravel_version.as_str() < "13" {
        panic!("🛑 Erro: Apenas Laravel 13 ou superior é suportado no pconnect.");
    }

    println!("✅ Configurações e Versões validadas para o projeto: {}", config.project.name);
}