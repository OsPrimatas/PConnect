use std::process::Command;
use std::path::{Path, PathBuf};
use crate::configs::pconnect_cfg::GlobalConfig;

pub fn spawn_shell(global: &GlobalConfig) {
    let home = std::env::var("USERPROFILE").expect("❌ USERPROFILE não encontrado");
    let base_path = PathBuf::from(home).join(".pconnect");

    // 1. Localiza os diretórios de binários usando a lógica de subpastas
    let php_dir = base_path.join(format!("php-{}", global.default_versions.php_version));
    let pg_dir = find_subfolder_bin(&base_path.join(format!("postgres-{}", global.default_versions.postgresql_version)), "bin");
    let bun_dir = find_subfolder_bin(&base_path.join(format!("bun-{}", global.default_versions.bun_version)), "bun.exe");

    // 2. Monta o PATH
    let current_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!(
        "{};{};{};{}",
        php_dir.display(),
        pg_dir.display(),
        bun_dir.display(),
        current_path
    );

    println!("💻 Entrando no pconnect Shell...");
    println!("✅ PHP, Composer, PostgreSQL e Bun prontos para uso.");
    println!("💡 Digite 'exit' para sair.\n");

    let _ = Command::new("powershell.exe")
        .arg("-NoExit")
        .arg("-Command")
        .arg("$host.ui.RawUI.WindowTitle = 'pconnect Shell'")
        .env("PATH", new_path)
        .status();
}

// Função auxiliar para o shell encontrar onde os binários realmente estão
fn find_subfolder_bin(base: &Path, target: &str) -> PathBuf {
    if !base.exists() { return base.to_path_buf(); }
    
    // Se o alvo for uma pasta (como 'bin' no postgres) e ela já existir na raiz
    if target == "bin" && base.join("bin").exists() { return base.join("bin"); }
    // Se o alvo for um arquivo (como 'bun.exe') e ele estiver na raiz
    if target.ends_with(".exe") && base.join(target).exists() { return base.to_path_buf(); }

    // Procura em subpastas (caso pgsql/bin ou bun-windows-x64/...)
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let path = entry.path();
                // Postgres fix
                if target == "bin" && path.join("bin").exists() { return path.join("bin"); }
                // Bun fix
                if target.ends_with(".exe") && path.join(target).exists() { return path; }
            }
        }
    }
    base.to_path_buf()
}