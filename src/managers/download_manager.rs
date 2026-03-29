use std::fs;
use std::io::{copy, Cursor};
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::configs::pconnect_cfg::GlobalConfig;

pub fn install_all(global: &GlobalConfig) {
    let home_dir = std::env::var("USERPROFILE").unwrap();
    let base_path = PathBuf::from(home_dir).join(".php-connects");

    // PHP
    if global.installations.php_install {
        let url = format!("https://downloads.php.net/~windows/releases/archives/php-{}-nts-Win32-vs17-x64.zip", global.default_versions.php_version);
        download_and_extract("php", &url, &global.default_versions.php_version, &base_path);
    }

    // MySQL
    if global.installations.mysql_install {
        let url = format!("https://downloads.mysql.com/archives/get/p/23/file/mysql-{}-winx64.zip", global.default_versions.mysql_version);
        download_and_extract("mysql", &url, &global.default_versions.mysql_version, &base_path);
    }

    // Bun
    if global.installations.bun_install {
        let url = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-windows-x64-baseline.zip", global.default_versions.bun_version);
        download_and_extract("bun", &url, &global.default_versions.bun_version, &base_path);
    }
}

fn download_and_extract(program_name: &str, url: &str, version: &str, target_path: &Path) {
    let program_path = target_path.join(&format!("{}-{}", program_name, version));
    
    if program_path.exists() {
        println!("🗂️ {}-{} já está instalado.", program_name, version);
        return;
    }
    
    println!("📥 Baixando {}-{}...", program_name, version);
    
    // Tratamento básico para caso o link falhe (ex: versão errada no URL)
    let client = reqwest::blocking::Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
    .build()
    .unwrap();

    let response = client.get(url).send().unwrap_or_else(|_| {
        panic!("❌ Erro fatal: Não foi possível conectar para baixar {}.", program_name)
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