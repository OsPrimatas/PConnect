use std::fs;
use std::io::{Cursor, stdin};
use std::path::{Path, PathBuf};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    if let Err(e) = run_install() {
        println!("\n❌ ERRO FATAL: {}", e);
    }

    println!("\n-------------------------------------------");
    println!("Pressione Enter para fechar...");
    let mut _input = String::new();
    let _ = stdin().read_line(&mut _input);
}

fn run_install() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PConnect Setup - Iniciando instalação...");

    let home = std::env::var("USERPROFILE")?;
    let pconnect_dir = PathBuf::from(&home).join(".pconnect");
    
    if !pconnect_dir.exists() {
        fs::create_dir_all(&pconnect_dir)?;
    }

    let url = "https://github.com/OsPrimatas/PConnect/releases/latest/download/pconnect_tool.zip";
    println!("📥 Baixando de: {}", url);
    
    let client = reqwest::blocking::Client::builder()
        .user_agent("PConnect-Installer")
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    let response = client.get(url).send()?;
    let bytes = response.bytes()?;
    let content = Cursor::new(bytes);
    
    println!("📦 Extraindo arquivos...");
    let mut archive = zip::ZipArchive::new(content)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = pconnect_dir.join(file.name());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() { fs::create_dir_all(p)?; }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    // --- NOVA LÓGICA DE ORGANIZAÇÃO (Move tudo e limpa) ---
    println!("🔍 Organizando arquivos e limpando subpastas...");
    
    // 1. Identifica a subpasta chata (pconnect_tool)
    let subfolder = pconnect_dir.join("pconnect_tool");
    
    if subfolder.exists() && subfolder.is_dir() {
        let entries = fs::read_dir(&subfolder)?;
        for entry in entries.flatten() {
            let from = entry.path();
            let file_name = from.file_name().unwrap();
            let mut to = pconnect_dir.join(file_name);

            // Se o arquivo for o executável, já renomeamos para o nome padrão
            if file_name.to_string_lossy().starts_with("pconnect_tool") && file_name.to_string_lossy().ends_with(".exe") {
                to = pconnect_dir.join("pconnect.exe");
            }

            // Move o arquivo (exe, toml, etc) para a raiz da .pconnect
            if to.exists() { fs::remove_file(&to)?; }
            fs::rename(&from, &to)?;
            println!("  ✨ Movido: {:?}", to.file_name().unwrap());
        }

        // 2. Remove a subpasta agora que ela está vazia
        fs::remove_dir_all(&subfolder)?;
        println!("🗑️  Pasta temporária removida.");
    }

    let target_exe = pconnect_dir.join("pconnect.exe");
    if !target_exe.exists() {
        return Err("Erro: pconnect.exe não encontrado após a organização!".into());
    }

    // --- CONFIGURAÇÃO E EXECUÇÃO ---
    println!("🔑 Configurando variáveis de ambiente...");
    setup_env_variables(pconnect_dir.to_str().ok_or("Caminho inválido")?)?;

    println!("🛠️  Executando 'pconnect install'...");
    let status = Command::new(&target_exe)
        .arg("install")
        .status()?;

    if status.success() {
        println!("\n✅ SUCESSO! O ambiente do PConnect está configurado.");
    } else {
        println!("\n⚠️  O CLI foi instalado, mas o 'pconnect install' falhou.");
    }

    Ok(())
}

fn setup_env_variables(path_value: &str) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment")?;
    env.set_value("PCONNECT", &path_value)?;
    let current_path: String = env.get_value("Path").unwrap_or_default();
    if !current_path.contains("%PCONNECT%") {
        let new_path = format!("{};%PCONNECT%", current_path);
        env.set_value("Path", &new_path)?;
    }
    Ok(())
}