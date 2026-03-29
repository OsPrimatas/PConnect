use std::fs;
use std::io::{copy, Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use reqwest::header::{USER_AGENT};
use crate::configs::pconnect_cfg::GlobalConfig;

pub fn install_all(global: &GlobalConfig) {
    let home_dir = std::env::var("USERPROFILE").expect("❌ Erro: USERPROFILE não encontrado");
    let base_path = PathBuf::from(home_dir).join(".pconnect");

    if !base_path.exists() {
        fs::create_dir_all(&base_path).ok();
    }

    // --- PHP + Composer ---
    if global.installations.php_install {
        let version = &global.versions.php_version;
        let url = format!(
            "https://downloads.php.net/~windows/releases/archives/php-{}-nts-Win32-vs17-x64.zip", 
            version
        );
        download_and_extract("php", &url, version, &base_path);
        let php_path = base_path.join(format!("php-{}", version));
        configure_php(&php_path);
        download_composer(&php_path);
    }

    // --- PostgreSQL ---
    if global.installations.postgresql_install {
        let version = &global.versions.postgresql_version;
        let url = format!(
            "https://get.enterprisedb.com/postgresql/postgresql-{}-1-windows-x64-binaries.zip",
            version
        );
        download_and_extract("postgres", &url, version, &base_path);
    }

    // --- Bun ---
    if global.installations.bun_install {
        let version = &global.versions.bun_version;
        // Usando a versão baseline conforme seu erro anterior
        let url = format!(
            "https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-windows-x64-baseline.zip", 
            version
        );
        
        // 1. Baixa e extrai normalmente
        download_and_extract("bun", &url, version, &base_path);
        
        // 2. Organiza a bagunça das subpastas do Bun
        fix_bun_structure("bun", version, &base_path);
    }
}

/// Remove a subpasta chata do Bun e deixa o bun.exe direto em .pconnect/bun-versao/
fn fix_bun_structure(program_name: &str, version: &str, base_path: &Path) {
    let program_path = base_path.join(format!("{}-{}", program_name, version));
    
    // Procura por qualquer subpasta dentro da pasta do bun (ex: bun-windows-x64-baseline)
    if let Ok(entries) = fs::read_dir(&program_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                println!("🧹 Organizando arquivos do Bun de: {:?}", path.file_name().unwrap());
                
                // Move tudo que está dentro da subpasta para a raiz (program_path)
                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let from = sub_entry.path();
                        let to = program_path.join(from.file_name().unwrap());
                        
                        if to.exists() { let _ = fs::remove_file(&to); }
                        let _ = fs::rename(&from, &to);
                    }
                }
                // Remove a subpasta que agora deve estar vazia
                let _ = fs::remove_dir_all(&path);
            }
        }
    }
}

// --- Resto das suas funções (Mantidas como originais, apenas ajustadas para compilar) ---

fn download_composer(php_path: &Path) {
    let composer_path = php_path.join("composer.phar");
    if composer_path.exists() { return; }

    println!("📥 Baixando Composer...");
    let client = reqwest::blocking::Client::new();
    if let Ok(res) = client.get("https://getcomposer.org/composer.phar").header(USER_AGENT, "Mozilla/5.0").send() {
        if res.status().is_success() {
            let mut file = fs::File::create(&composer_path).unwrap();
            let mut content = Cursor::new(res.bytes().unwrap());
            copy(&mut content, &mut file).unwrap();
            println!("✅ Composer instalado!");
        }
    }
}

fn download_and_extract(program_name: &str, url: &str, version: &str, target_path: &Path) {
    let program_path = target_path.join(&format!("{}-{}", program_name, version));
    if program_path.exists() {
        println!("🗂️  {}-{} já está instalado.", program_name, version);
        return;
    }
    
    println!("📥 Baixando {}-{}...", program_name, version);
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0")
        .build().unwrap();

    let response = client.get(url).send().expect("Erro ao conectar");
    let bytes = response.bytes().expect("Erro nos bytes");
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).expect("Erro no ZIP");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if file.name().contains("__MACOSX") { continue; }
        
        let outpath = match file.enclosed_name() {
            Some(path) => program_path.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() { fs::create_dir_all(&p).ok(); }
            let mut outfile = fs::File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
    }

    if program_name == "postgres" {
        initialize_postgres_local_storage(&program_path);
    }
}

fn configure_php(php_path: &Path) {
    let ini_dev = php_path.join("php.ini-development");
    let ini_real = php_path.join("php.ini");
    if !ini_dev.exists() || ini_real.exists() { return; }

    if let Ok(content) = fs::read_to_string(&ini_dev) {
        let new_content = content
            .replace(";extension_dir = \"ext\"", "extension_dir = \"ext\"")
            .replace(";extension=curl", "extension=curl")
            .replace(";extension=mbstring", "extension=mbstring")
            .replace(";extension=openssl", "extension=openssl")
            .replace(";extension=pdo_pgsql", "extension=pdo_pgsql")
            .replace(";extension=pgsql", "extension=pgsql")
            .replace(";extension=fileinfo", "extension=fileinfo")
            .replace(";extension=pdo_sqlite", "extension=pdo_sqlite");

        let mut file = fs::File::create(ini_real).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();
    }
}

fn initialize_postgres_local_storage(pg_path: &Path) {
    let mut bin_path = pg_path.join("bin");
    if !bin_path.exists() { bin_path = pg_path.join("pgsql").join("bin"); }
    let initdb_bin = bin_path.join("initdb.exe");
    let data_dir = pg_path.join("data");

    if !initdb_bin.exists() || data_dir.exists() { return; }

    println!("⚙️  Inicializando cluster PostgreSQL...");
    let _ = Command::new(initdb_bin)
        .arg("-D").arg(&data_dir)
        .arg("-U").arg("postgres")
        .arg("--auth=trust")
        .status();
}