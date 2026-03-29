use std::fs;
use std::io::{copy, Cursor};
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_PHP: &str = "8.5.4";
const DEFAULT_MYSQL: &str = "8.0.46";
const DEFAULT_BUN: &str = "1.3.6";

pub fn install_all() {
    let home_dir = std::env::var("USERPROFILE").expect("Não foi possível encontrar a pasta do usuário.");
    let base_path = PathBuf::from(home_dir).join(".php-connects");

    // URLs usando as constantes padrão
    let php_url = format!("https://windows.php.net/downloads/releases/archives/php-{}-Win32-vs16-x64.zip", DEFAULT_PHP);
    let mysql_url = format!("https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-{}-winx64.zip", DEFAULT_MYSQL);
    let bun_url = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-windows-x64.zip", DEFAULT_BUN);

    if !base_path.exists() {
        fs::create_dir_all(&base_path).unwrap();
    }

    println!("🛠️ Iniciando instalação global do ecossistema pconnect...");

    download_and_extract("php", &php_url, DEFAULT_PHP, &base_path);
    download_and_extract("mysql", &mysql_url, DEFAULT_MYSQL, &base_path);
    download_and_extract("bun", &bun_url, DEFAULT_BUN, &base_path);
    
    println!("\n✨ Instalação concluída! Agora você pode usar 'pconnect create <nome>'.");
}

fn download_and_extract(program_name: &str, url: &str, version: &str, target_path: &Path) {
    let program_path = target_path.join(&format!("{}-{}", program_name, version));
    
    if program_path.exists() {
        println!("🗂️ {}-{} já está instalado.", program_name, version);
        return;
    }
    
    println!("📥 Baixando {}-{}...", program_name, version);
    
    // Tratamento básico para caso o link falhe (ex: versão errada no URL)
    let response = reqwest::blocking::get(url).unwrap_or_else(|_| {
        panic!("❌ Erro fatal: Não foi possível conectar ao servidor para baixar {}. Verifique sua internet.", program_name)
    });

    if !response.status().is_success() {
        println!("❌ Erro ao baixar {}: Status {}", program_name, response.status());
        return;
    }

    let content = Cursor::new(response.bytes().expect("Falha ao ler bytes"));

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

    println!("✅ {}-{} instalado.", program_name, version);

    if program_name == "mysql" {
        initialize_mysql_local_storage(&program_path);
    }
}

fn initialize_mysql_local_storage(mysql_path: &Path) {
    let mysqld_bin = mysql_path.join("bin").join("mysqld.exe");

    if !mysqld_bin.exists() { return; }

    println!("⚙️ Inicializando base de dados global...");

    let _ = Command::new(mysqld_bin)
        .arg("--initialize-insecure")
        .arg(format!("--basedir={}", mysql_path.display()))
        .arg(format!("--datadir={}", mysql_path.join("data").display()))
        .status();
}