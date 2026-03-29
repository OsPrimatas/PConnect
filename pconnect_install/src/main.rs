use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PConnect Setup - Iniciando instalação...");

    // 1. Definir caminhos
    let home = std::env::var("USERPROFILE")?;
    let pconnect_dir = PathBuf::from(&home).join(".pconnect");
    
    if !pconnect_dir.exists() {
        fs::create_dir_all(&pconnect_dir)?;
    }

    // 2. Baixar a ferramenta do GitHub
    let url = "https://github.com/OsPrimatas/PConnect/releases/latest/download/pconnect.zip";
    println!("📥 Baixando a versão mais recente do PConnect...");
    
    let response = reqwest::blocking::get(url)?;
    let content = Cursor::new(response.bytes()?);
    
    println!("📦 Extraindo arquivos em: {}...", pconnect_dir.display());
    let mut archive = zip::ZipArchive::new(content)?;
    archive.extract(&pconnect_dir)?;

    // 3. Configurar Variáveis de Ambiente
    println!("🔑 Configurando variáveis de ambiente...");
    setup_env_variables(pconnect_dir.to_str().unwrap())?;

    // 4. Chamar o 'pconnect install' da ferramenta que acabamos de baixar
    println!("🛠️  Chamando 'pconnect install' para configurar dependências (PHP, Postgres, Bun)...");
    let pconnect_exe = pconnect_dir.join("pconnect.exe");

    let status = Command::new(pconnect_exe)
        .arg("install")
        .status()?;

    if status.success() {
        println!("\n✅ Instalação concluída com sucesso!");
        println!("🔔 Reinicie qualquer terminal aberto para começar a usar o comando 'pconnect'.");
    } else {
        println!("\n❌ Ocorreu um erro ao executar 'pconnect install'.");
    }

    // Aguarda o usuário ler antes de fechar
    println!("\nPressione Enter para sair...");
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input)?;

    Ok(())
}

fn setup_env_variables(path_value: &str) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment")?;

    // Criar a variável PCONNECT
    env.set_value("PCONNECT", &path_value)?;
    println!("✅ Variável %PCONNECT% criada.");

    // Inserir %PCONNECT% no PATH (se não existir)
    let current_path: String = env.get_value("Path")?;
    if !current_path.contains("%PCONNECT%") {
        let new_path = format!("{};%PCONNECT%", current_path);
        env.set_value("Path", &new_path)?;
        println!("✅ %PCONNECT% adicionado ao PATH do usuário.");
    }

    Ok(())
}
