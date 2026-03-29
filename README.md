# 🚀 PHP-Connects (pconnect)

O **pconnect** é um gerenciador de projetos e CLI escrito em Rust, projetado para orquestrar ambientes de desenvolvimento Fullstack de alto desempenho. Ele automatiza a instalação de binários globais e gerencia instâncias isoladas de banco de dados, integrando o ecossistema Laravel e Vue através do Bun.

---

## 🛠️ Comandos Principais

| Comando                  | Descrição                                                                                                       |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `pconnect install`       | **Setup Global**: Baixa e instala as versões oficiais do PHP, MySQL e Bun na pasta do usuário.                  |
| `pconnect create <nome>` | **Scaffolding**: Cria um novo projeto com o esqueleto do Laravel (Backend) e Vue/Vite (Frontend) já integrados. |
| `pconnect run`           | **Maestro**: Inicia simultaneamente o PHP, o MySQL local (.cache) e o servidor de dev do Vite via Bun.          |
| `pconnect stop`          | **Cleanup**: Encerra todos os processos ativos, libera as portas e remove os arquivos de PID.                   |

---

## 📖 Como Usar

### 1. Preparação do Ambiente

Execute o comando de instalação para baixar os binários necessários. Eles ficarão armazenados em `~/.php-connects` para não poluir suas variáveis de ambiente globais.

```bash
pconnect install
```
