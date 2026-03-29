use std::fs;
use std::io::{copy, Cursor};
use std::path::{Path, PathBuf};
use crate::configs::php_connects_cfg::Config;
use std::process::Command;

pub fn install_all(config: &Config) {
    let home_dir = std::env::var("USERPROFILE").expect("Não foi possível encontrar a pasta do usuário.");
    let base_path = PathBuf::from(home_dir).join(".php-connects");

    let php_url = format!("https://windows.php.net/downloads/releases/archives/php-{}-Win32-vs16-x64.zip", &config.versions.php_version);
    let mysql_url = format!("https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-{}-winx64.zip", &config.versions.mysql_version);
    let bun_url = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-windows-x64.zip", &config.versions.bun_version);

    // Cria a pasta .pconnect/bin se não existir
    if !base_path.exists() {
        fs::create_dir_all(&base_path).unwrap();
    }

    // Baixar e instalar o PHP
    download_and_extract("php", &php_url, &config.versions.php_version, &base_path);

    // Baixar e instalar o MySQL
    download_and_extract("mysql", &mysql_url, &config.versions.mysql_version, &base_path);

    // Baixar e instalar o Bun
    download_and_extract("bun", &bun_url, &config.versions.bun_version, &base_path);
}

fn download_and_extract(program_name: &str, url: &str, version: &str, target_path: &Path) {
    let program_path = target_path.join(&format!("{}-{}", program_name, version));
    
    // Se a pasta já existir, assume que o programa já está instalado e pula o download
    if program_path.exists() {
        println!("🗂️ {}-{} já está instalado.", program_name, version);
        return;
    }
    
    // Baixar arquivo ZIP
    println!("📥 Baixando {}-{}...", program_name, version);
    let response = reqwest::blocking::get(url).expect(&format!("Falha ao baixar {}", program_name));
    let content = Cursor::new(response.bytes().expect(&format!("Falha ao ler bytes do {}", program_name)));

    // Extrair arquivo ZIP
    println!("📦 Extraindo arquivos...");
    let mut archive = zip::ZipArchive::new(content).expect("Falha ao abrir ZIP");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = program_path.join(file.mangled_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() { fs::create_dir_all(&p).unwrap(); }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
    }
    println!("✅ {}-{} instalado em: {}", program_name, version, program_path.display());

    if program_name == "mysql" {
        initialize_mysql_local_storage(&program_path);
    }
}

fn initialize_mysql_local_storage(mysql_path: &Path) {
    let mysqld_bin = mysql_path.join("bin").join("mysqld.exe");

    if !mysqld_bin.exists() {
        println!("❌ Erro: mysqld.exe não encontrado em {}", mysqld_bin.display());
        return;
    }

    println!("⚙️ Inicializando base de dados global (apenas arquivos de sistema)...");

    let status = Command::new(mysqld_bin)
        .arg("--initialize-insecure")
        .arg(format!("--basedir={}", mysql_path.display()))
        .arg(format!("--datadir={}", mysql_path.join("data").display()))
        .status();

    match status {
        Ok(s) if s.success() => println!("✅ MySQL preparado para uso."),
        _ => println!("⚠️ Aviso: O MySQL pode já ter sido inicializado ou houve um erro permissão."),
    }
}