use std::fs;
use std::io::{copy, Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use reqwest::header::{HeaderMap, USER_AGENT};
use crate::configs::pconnect_cfg::GlobalConfig;

pub fn install_all(global: &GlobalConfig) {
    let home_dir = std::env::var("USERPROFILE").expect("❌ Erro: USERPROFILE não encontrado");
    let base_path = PathBuf::from(home_dir).join(".pconnect");

    if !base_path.exists() {
        fs::create_dir_all(&base_path).ok();
    }

    // Instalação do PHP + Composer
    if global.installations.php_install {
        let version = &global.versions.php_version;
        let url = format!(
            "https://downloads.php.net/~windows/releases/archives/php-{}-nts-Win32-vs17-x64.zip", 
            version
        );
        
        // 1. Instala o PHP
        download_and_extract("php", &url, version, &base_path);
        
        let php_path = base_path.join(format!("php-{}", version));
        
        // 2. Configura o php.ini
        configure_php(&php_path);

        // 3. Baixa o Composer.phar para dentro da pasta do PHP
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
        let url = format!(
            "https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-windows-x64-baseline.zip", 
            version
        );
        download_and_extract("bun", &url, version, &base_path);
    }
}

// Função para baixar o Composer especificamente
fn download_composer(php_path: &Path) {
    let composer_path = php_path.join("composer.phar");
    
    if composer_path.exists() {
        println!("🗂️  Composer já está presente na pasta do PHP.");
        return;
    }

    println!("📥 Baixando Composer...");
    
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://getcomposer.org/composer.phar")
        .header(USER_AGENT, "Mozilla/5.0")
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let mut file = fs::File::create(&composer_path).expect("Falha ao criar arquivo composer.phar");
                let mut content = Cursor::new(res.bytes().expect("Falha ao ler bytes do Composer"));
                copy(&mut content, &mut file).expect("Falha ao salvar o Composer");
                println!("✅ Composer instalado com sucesso em {}!", php_path.display());
            } else {
                println!("❌ Erro ao baixar Composer: Status {}", res.status());
            }
        }
        Err(e) => println!("❌ Erro de conexão ao baixar Composer: {}", e),
    }
}

fn download_and_extract(program_name: &str, url: &str, version: &str, target_path: &Path) {
    let program_path = target_path.join(&format!("{}-{}", program_name, version));
    
    if program_path.exists() {
        println!("🗂️  {}-{} já está instalado.", program_name, version);
        return;
    }
    
    println!("📥 Baixando {}-{}...", program_name, version);
    
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client.get(url).send().unwrap_or_else(|_| {
        panic!("❌ Erro fatal: Não foi possível conectar para baixar {}.", program_name)
    });

    if !response.status().is_success() {
        println!("❌ Erro ao baixar {}: Status {} na URL: {}", program_name, response.status(), url);
        return;
    }

    let bytes = response.bytes().expect("Falha ao ler bytes do servidor");
    let content = Cursor::new(bytes);

    println!("📦 Extraindo arquivos de {}...", program_name);
    let mut archive = zip::ZipArchive::new(content).expect("Falha ao abrir arquivo ZIP");

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
            if let Some(p) = outpath.parent() {
                if !p.exists() { fs::create_dir_all(&p).unwrap(); }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }
    }

    println!("✅ {}-{} instalado com sucesso!", program_name, version);

    if program_name == "postgres" {
        initialize_postgres_local_storage(&program_path);
    }
}

fn configure_php(php_path: &Path) {
    let ini_dev = php_path.join("php.ini-development");
    let ini_real = php_path.join("php.ini");

    if !ini_dev.exists() || ini_real.exists() { return; }

    println!("⚙️  Configurando php.ini para PostgreSQL...");

    if let Ok(content) = fs::read_to_string(&ini_dev) {
        let mut new_content = content
            .replace(";extension_dir = \"ext\"", "extension_dir = \"ext\"")
            .replace(";extension=curl", "extension=curl")
            .replace(";extension=mbstring", "extension=mbstring")
            .replace(";extension=openssl", "extension=openssl")
            .replace(";extension=pdo_pgsql", "extension=pdo_pgsql")
            .replace(";extension=pgsql", "extension=pgsql")
            .replace(";extension=fileinfo", "extension=fileinfo"); // Importante para o Laravel

        new_content = new_content.replace(";extension=pdo_sqlite", "extension=pdo_sqlite");

        let mut file = fs::File::create(ini_real).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();
        println!("✅ php.ini configurado com extensões necessárias.");
    }
}

fn initialize_postgres_local_storage(pg_path: &Path) {
    let mut bin_path = pg_path.join("bin");
    if !bin_path.exists() {
        bin_path = pg_path.join("pgsql").join("bin");
    }

    let initdb_bin = bin_path.join("initdb.exe");
    let data_dir = pg_path.join("data");

    if !initdb_bin.exists() { 
        println!("⚠️  initdb.exe não encontrado em {}. Verifique a estrutura.", bin_path.display());
        return; 
    }

    if data_dir.exists() { return; }

    println!("⚙️  Inicializando cluster de dados do PostgreSQL...");

    let _ = Command::new(initdb_bin)
        .arg("-D")
        .arg(&data_dir)
        .arg("-U")
        .arg("postgres")
        .arg("--auth=trust")
        .status();

    println!("✅ PostgreSQL pronto para uso local.");
}