use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use crate::configs::pconnect_cfg::{ProjectConfig, GlobalConfig};

const PID_FILE: &str = ".pconnect_postgresql.pid";

fn get_postgresql_bin_dir(global: &GlobalConfig) -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("❌ USERPROFILE não encontrado");
    let base = PathBuf::from(home)
        .join(".pconnect")
        .join(format!("postgres-{}", global.versions.postgresql_version));

    let pgsql_folder = base.join("pgsql");
    if pgsql_folder.exists() {
        pgsql_folder.join("bin")
    } else {
        base.join("bin")
    }
}

pub fn run_postgresql(project: &ProjectConfig, global: &GlobalConfig) {
    if Path::new(PID_FILE).exists() {
        println!("⚠️  O PostgreSQL já parece estar rodando.");
        return;
    }

    let bin_dir = get_postgresql_bin_dir(global);
    let postgres_exe = bin_dir.join("postgres.exe");
    let initdb_exe = bin_dir.join("initdb.exe");
    let psql_exe = bin_dir.join("psql.exe");
    
    // O banco de dados fica isolado na pasta .cache do projeto atual
    let local_data_dir = std::env::current_dir().unwrap().join(".cache").join("postgresql");
    let is_first_run = !local_data_dir.exists();

    if !postgres_exe.exists() {
        println!("❌ Erro: PostgreSQL não encontrado em {}. Rode 'pconnect install'.", postgres_exe.display());
        return;
    }

    // 1. Inicialização do Cluster (Apenas na primeira vez)
    if is_first_run {
        println!("🐘 Inicializando novo cluster PostgreSQL local para o projeto...");
        fs::create_dir_all(&local_data_dir).unwrap();

        // initdb -D <dir> -U postgres --auth=trust
        let init_status = Command::new(&initdb_exe)
            .arg("-D")
            .arg(&local_data_dir)
            .arg("-U")
            .arg("postgres")
            .arg("--auth=trust") 
            .output();

        if let Err(e) = init_status {
            println!("❌ Falha ao inicializar cluster: {}", e);
            return;
        }
    }

    println!("🐘 Iniciando PostgreSQL na porta {}...", project.ports.postgresql_port);

    // 2. Disparar o Servidor
    // No Postgres usamos -p para porta e -D para o diretório de dados
    let child = Command::new(&postgres_exe)
        .arg("-D")
        .arg(&local_data_dir)
        .arg("-p")
        .arg(project.ports.postgresql_port.to_string())
        .stdout(Stdio::null()) 
        .stderr(Stdio::null())
        .spawn();

    match child {
        Ok(process) => {
            let pid = process.id();
            fs::write(PID_FILE, pid.to_string()).ok();
            println!("✅ PostgreSQL pronto! (PID: {})", pid);

            // 3. Criar o Banco de Dados do Projeto
            // O Postgres precisa de um tempo para aceitar conexões após o spawn
            if is_first_run {
                setup_postgresql_database(&psql_exe, project);
            }
        }
        Err(e) => println!("❌ Erro ao disparar PostgreSQL: {}", e),
    }
}

fn setup_postgresql_database(psql_exe: &Path, project: &ProjectConfig) {
    println!("🗄️  Criando Database '{}' e configurando usuário...", project.postgresql.db);
    
    // Aguarda o motor subir
    thread::sleep(Duration::from_secs(4));

    // Comandos SQL para Postgres: criar banco e mudar senha do usuário padrão
    let sql_commands = format!(
        "ALTER USER postgres WITH PASSWORD '{}'; CREATE DATABASE {};",
        project.postgresql.pass, project.postgresql.db
    );

    let _ = Command::new(psql_exe)
        .args([
            "-p", &project.ports.postgresql_port.to_string(),
            "-U", "postgres",
            "-d", "postgres", 
            "-c", &sql_commands
        ])
        .status();

    println!("✅ Banco de dados '{}' pronto para uso!", project.postgresql.db);
}

pub fn stop_postgresql() {
    if !Path::new(PID_FILE).exists() { return; }

    let pid = fs::read_to_string(PID_FILE).unwrap_or_default();
    
    let _ = Command::new("taskkill")
        .args(["/F", "/PID", pid.trim(), "/T"])
        .output();

    let _ = fs::remove_file(PID_FILE);
    println!("✅ Servidor PostgreSQL encerrado.");
}