use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use crate::configs::pconnect_cfg::{ProjectConfig, GlobalConfig};

const PID_FILE: &str = ".pconnect_mysql.pid";

fn get_mysql_bin(global: &GlobalConfig) -> PathBuf {
    // Se não for para usar o instalado pelo pconnect, tenta o global do sistema
    if !global.installations.mysql_install {
        return PathBuf::from("mysqld.exe");
    }

    let home = std::env::var("USERPROFILE").expect("❌ Erro: USERPROFILE não encontrado");
    PathBuf::from(home)
        .join(".php-connects")
        .join(format!("mysql-{}", global.default_versions.mysql_version))
        .join("bin")
        .join("mysqld.exe")
}

pub fn run_mysql(project: &ProjectConfig, global: &GlobalConfig) {
    if Path::new(PID_FILE).exists() {
        println!("⚠️  O MySQL já parece estar rodando.");
        return;
    }

    let mysql_bin = get_mysql_bin(global);
    let mysql_client = mysql_bin.parent().unwrap_or(Path::new(".")).join("mysql.exe");
    
    // O banco de dados fica isolado na pasta .cache do projeto atual
    let local_data_dir = std::env::current_dir().unwrap().join(".cache").join("mysql");
    let is_first_run = !local_data_dir.exists();

    if !mysql_bin.exists() && global.installations.mysql_install {
        println!("❌ Erro: MySQL não encontrado em {}. Rode 'pconnect install'.", mysql_bin.display());
        return;
    }

    // 1. Inicialização (Apenas na primeira vez que o projeto roda)
    if is_first_run {
        println!("🎲 Inicializando novo banco de dados local para o projeto...");
        fs::create_dir_all(&local_data_dir).unwrap();

        let init_status = Command::new(&mysql_bin)
            .arg("--initialize-insecure") // Cria root sem senha inicialmente
            .arg(format!("--datadir={}", local_data_dir.display()))
            .output();

        if let Err(e) = init_status {
            println!("❌ Falha ao inicializar banco: {}", e);
            return;
        }
    }

    println!("🐬 Iniciando MySQL na porta {}...", project.ports.mysql_port);

    // 2. Disparar o Servidor
    let child = Command::new(&mysql_bin)
        .args([
            &format!("--datadir={}", local_data_dir.display()),
            &format!("--port={}", project.ports.mysql_port),
            "--console",
        ])
        .stdout(Stdio::null()) // Silenciamos o log bruto do MySQL para não poluir o terminal
        .spawn();

    match child {
        Ok(process) => {
            let pid = process.id();
            fs::write(PID_FILE, pid.to_string()).ok();
            println!("✅ MySQL pronto! (PID: {})", pid);

            // 3. Configuração de Credenciais (Apenas no First Run)
            if is_first_run {
                setup_mysql_credentials(&mysql_client, project);
            }
        }
        Err(e) => println!("❌ Erro ao disparar MySQL: {}", e),
    }
}

fn setup_mysql_credentials(mysql_client: &Path, project: &ProjectConfig) {
    println!("🔐 Configurando credenciais (root) e criando Database '{}'...", project.mysql.db);
    
    // Aguarda o motor do banco subir
    thread::sleep(Duration::from_secs(5));

    // Comando para setar senha e criar o banco definido no pconnect.cfg.toml
    let sql_commands = format!(
        "ALTER USER 'root'@'localhost' IDENTIFIED BY '{}'; CREATE DATABASE IF NOT EXISTS {}; FLUSH PRIVILEGES;",
        project.mysql.pass, project.mysql.db
    );

    let _ = Command::new(mysql_client)
        .args([
            "--port", &project.ports.mysql_port.to_string(),
            "-u", "root",
            "--skip-password",
            "-e", &sql_commands
        ])
        .status();

    println!("✅ Banco de dados e senha configurados!");
}

pub fn stop_mysql() {
    if !Path::new(PID_FILE).exists() { return; }

    let pid = fs::read_to_string(PID_FILE).unwrap_or_default();
    
    let _ = Command::new("taskkill")
        .args(["/F", "/PID", pid.trim(), "/T"])
        .output();

    let _ = fs::remove_file(PID_FILE);
    println!("✅ Servidor MySQL encerrado.");
}