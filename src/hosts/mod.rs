use std::fs::{OpenOptions};
use std::io::{Write, Read};

pub fn register_local_domain(project_name: &str) {
    let hosts_path = r"C:\Windows\System32\drivers\etc\hosts";
    let entry = format!("\n127.0.0.1  {}.local", project_name);

    let mut file_content = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(hosts_path)
        .expect("❌ Erro: Execute o CLI como Administrador para configurar o domínio local.");

    file.read_to_string(&mut file_content).ok();

    if !file_content.contains(&format!("{}.local", project_name)) {
        file.write_all(entry.as_bytes()).expect("Falha ao escrever no arquivo hosts");
        println!("✅ Domínio local {}.local registrado!", project_name);
    }
}