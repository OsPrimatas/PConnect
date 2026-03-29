use std::process::{Command};
use std::fs;
use std::path::{PathBuf};
use std::thread;
use std::time::Duration;
use crate::configs::php_connects_cfg::Config;

const PID_FILE: &str = ".mysql.pid";

// Obter a pasta global de instalação do MySQL (a mesma usada pelo download_manager)
fn get_global_mysql_bin(version: &str) -> PathBuf {
    // 1. Pega a pasta do usuário (C:\Users\Nome)
    let home = std::env::var("USERPROFILE")
        .expect("❌ Erro: Não foi possível encontrar a variável USERPROFILE");

    // 2. Monta o caminho até o executável que o download_manager baixou
    PathBuf::from(home)
        .join(".php-connects")           // Sua pasta global
        .join(format!("mysql-{}", version)) // Pasta da versão específica
        .join("bin")                     // Pasta padrão de binários do MySQL
        .join("mysqld.exe")              // O "Daemon" (servidor) do MySQL
}

pub fn run_mysql(config: &Config) {
    let mysql_bin = get_global_mysql_bin(&config.versions.mysql_version);
    let mysql_client = mysql_bin.parent().unwrap().join("mysql.exe"); // O cliente para enviar comandos
    let local_data_dir = std::env::current_dir().unwrap().join(".cache").join("mysql");
    
    let is_first_run = !local_data_dir.exists();

    if is_first_run {
        println!("🎲 Inicializando novo banco de dados local...");
        fs::create_dir_all(&local_data_dir).unwrap();

        Command::new(&mysql_bin)
            .arg("--initialize-insecure")
            .arg(format!("--datadir={}", local_data_dir.display()))
            .output()
            .expect("Falha ao inicializar banco local");
    }

    println!("🐬 Iniciando MySQL Local (Porta: {})", config.ports.mysql_port);

    let child = Command::new(&mysql_bin)
        .args([
            &format!("--datadir={}", local_data_dir.display()),
            &format!("--port={}", config.ports.mysql_port),
            "--console",
        ])
        .spawn()
        .expect("Falha ao disparar MySQL");

    // Salva o PID para o stop_mysql
    fs::write(PID_FILE, child.id().to_string()).ok();

    // 🔑 Configuração de Usuário e Senha (apenas na primeira vez)
    if is_first_run {
        println!("🔐 Configurando credenciais do root...");
        
        // Aguarda o banco subir totalmente
        thread::sleep(Duration::from_secs(5));

        let sql_command = format!(
            "ALTER USER 'root'@'localhost' IDENTIFIED BY '{}'; FLUSH PRIVILEGES;",
            config.mysql.pass
        );

        let setup_status = Command::new(mysql_client)
            .args([
                "--port", &config.ports.mysql_port.to_string(),
                "-u", "root",
                "--skip-password",
                "-e", &sql_command
            ])
            .status();

        match setup_status {
            Ok(s) if s.success() => println!("✅ Root configurado com a senha do seu config.toml!"),
            _ => println!("⚠️ Aviso: Não foi possível setar a senha. Talvez o banco demorou a subir."),
        }
    }
}

pub fn stop_mysql() {
    if !std::path::Path::new(PID_FILE).exists() {
        println!("⚠️ Nenhum MySQL ativo encontrado.");
        return;
    }

    let pid = fs::read_to_string(PID_FILE).unwrap_or_default();
    
    println!("🛑 Encerrando MySQL (PID: {})...", pid);

    let status = Command::new("taskkill")
        .args(["/F", "/PID", pid.trim(), "/T"])
        .output();

    if status.is_ok() {
        let _ = fs::remove_file(PID_FILE);
        println!("✅ MySQL encerrado.");
    }
}