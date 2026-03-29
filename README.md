# 🚀 PConnect

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

### 2. Criando um Novo Ecossistema

Este comando gera a estrutura de pastas, baixa as dependências e configura automaticamente o arquivo `.env` do Laravel para conectar ao banco local

```bash
pconnect create "meu-projeto"
```

### 3. Desenvolvimento Ativo

Entre na pasta criada e suba toda a stack. O **pconnect** configurará o domínio local (ex: meu-projeto.local) e abrirá as portas definidas no seu `.toml`.

```bash
cd meu-projeto
pconnect run
```

### 4. Finalizando a Sessão

Para parar todos os serviços de forma segura:

```bash
pconnect stop
```

### Estrutura de Diretórios Gerada

O **pconnect** segue uma arquitetura opinativa para garantir portabilidade:

```bash
/meu-projeto
├── /backend                    # Laravel Framework (PHP 8.x)
├── /frontend                   # Vue 3 + Vite (Bun)
├── /.cache/mysql               # Dados do MySQL (Exclusivos deste projeto)
├── php_connects.cfg.toml       # Configurações de portas, versões e proxy
└── .pconnect.pid               # Arquivo de controle de processo
```

### ⚙️ Requisitos

- Windows 10/11
- Acesso à Internet (para o primeiro download dos binários)
