# 🕶️ DARKTRACE

Advanced Local Network Recon Tool (Rust + Nmap)

“You can’t defend what you don’t see.”

DarkTrace é uma ferramenta de reconhecimento de rede local desenvolvida em Rust, utilizando o poder do Nmap para mapear dispositivos ativos, identificar sistemas operacionais e extrair informações de hardware em tempo real.

Projetado para estudos em:

🔎 Network Reconnaissance

🧠 Enumeração interna

🛰 Análise de rede

🛡 Segurança ofensiva

⚡ Features

🔍 Auto-discovery da subnet local

🛰 SYN Scan (-sS)

🖥 OS Fingerprinting (-O)

🎯 OS Guessing avançado (--osscan-guess)

🧾 Enumeração de IP

🔐 Extração de MAC Address

🏷 Identificação de fabricante via OUI

🧠 Parsing automatizado da saída do Nmap

⚙️ CLI leve, rápida e extensível

🧠 Como Funciona

O DarkTrace executa as seguintes etapas:

Detecta automaticamente sua subnet ativa (ex: 192.168.1.0/24)

Executa um scan SYN furtivo

Realiza fingerprinting de sistema operacional

Processa a saída usando Regex

Exibe os alvos identificados de forma estruturada

Comando executado internamente:

sudo nmap -O -sS --osscan-guess 192.168.1.0/24

💻 Setup
📋 Requisitos

Linux (recomendado)

Rust toolchain

Nmap instalado

Permissões root/sudo

🛠 Instalação

Clone o repositório:

git clone https://github.com/ruiasiqueira/darktrace.git
cd darktrace


Compile:

cargo build


Execute:

sudo cargo run


⚠️ Root é obrigatório para que o OS detection funcione corretamente.

🔥 Exemplo de Output
[+] Target Found
IP: 192.168.1.1
MAC: AA:BB:CC:DD:EE:FF (TP-Link Technologies)
OS: Linux 3.X
----------------------------------------
[+] Target Found
IP: 192.168.1.15
MAC: 11:22:33:44:55:66 (Intel Corporate)
OS: Windows 10
----------------------------------------

🧪 Code Quality & Checks

Formatação padrão Rust:

cargo fmt


Análise estática:

cargo clippy


Testes automatizados (quando implementados):

cargo test

📦 Roadmap

 Exportação JSON

 Parsing estruturado via XML

 Output colorido

 Modo silencioso

 Versão assíncrona (Tokio)

 Dashboard Web

 Docker build

 Modularização (scanner.rs, parser.rs)

⚠️ Legal Notice

Esta ferramenta foi desenvolvida exclusivamente para:

Laboratórios

Ambientes de estudo

Redes com autorização explícita

O uso não autorizado pode ser ilegal dependendo da jurisdição.

Você é totalmente responsável pelo uso desta ferramenta.

🛠 Stack

🦀 Rust

🔎 Nmap

📦 Regex crate

👾 Authors

Rui A. Siqueira
GitHub: https://github.com/ruiasiqueira

Davi Guerra
GitHub: https://github.com/daviguerra05

Kaiky Alvaro
GitHub: https://github.com/kaikyalvaro1708
