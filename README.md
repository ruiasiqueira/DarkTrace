# 🕶️ DARKTRACE  
### Advanced Local Network Recon Tool (Rust + Nmap)

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)
![Nmap](https://img.shields.io/badge/Nmap-Required-red?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Active-darkgreen?style=for-the-badge)

> _"You can’t defend what you don’t see."_

**DarkTrace** é uma ferramenta de reconhecimento de rede local desenvolvida em **Rust**, utilizando o poder do **Nmap** para mapear dispositivos ativos, identificar sistemas operacionais e extrair informações de hardware em tempo real.

Projetado para estudos em:

- 🔎 Network Reconnaissance  
- 🧠 Enumeração interna  
- 🛰 Análise de rede  
- 🛡 Segurança ofensiva  

---

# ⚡ Features

- 🔍 Auto-discovery da subnet local  
- 🛰 SYN Scan (`-sS`)  
- 🖥 OS Fingerprinting (`-O`)  
- 🎯 OS Guessing avançado (`--osscan-guess`)  
- 🧾 Enumeração de IP  
- 🔐 Extração de MAC Address  
- 🏷 Identificação de fabricante via OUI  
- 🧠 Parsing automatizado da saída do Nmap  
- ⚙️ CLI leve, rápida e extensível  

---

# 🧠 Como Funciona

O DarkTrace executa as seguintes etapas:

1. Detecta automaticamente sua subnet ativa (ex: `192.168.1.0/24`)
2. Executa um scan SYN furtivo
3. Realiza fingerprinting de sistema operacional
4. Processa a saída usando Regex
5. Exibe os alvos identificados de forma estruturada

Comando executado internamente:

```bash
sudo nmap -O -sS --osscan-guess 192.168.1.0/24
```

---

# 💻 Setup

## 📋 Requisitos

- Linux (recomendado)
- Rust toolchain
- Nmap instalado
- Permissões root/sudo

---

## 🛠 Instalação

Clone o repositório:

```bash
git clone https://github.com/ruiasiqueira/darktrace.git
cd darktrace
```

Compile:

```bash
cargo build
```

Execute:

```bash
sudo cargo run
```

> ⚠️ Root é obrigatório para que o OS detection funcione corretamente.

---

# 🔥 Exemplo de Output

```text
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
```

---

# 🧪 Code Quality

Formatação:

```bash
cargo fmt
```

Análise estática:

```bash
cargo clippy
```

Testes (quando implementados):

```bash
cargo test
```

---

# 📦 Roadmap

- [ ] Exportação JSON  
- [ ] Parsing estruturado via XML  
- [ ] Output colorido  
- [ ] Modo silencioso  
- [ ] Versão assíncrona (Tokio)  
- [ ] Dashboard Web  
- [ ] Docker build  
- [ ] Modularização (`scanner.rs`, `parser.rs`)  

---

# ⚠️ Legal Notice

Esta ferramenta foi desenvolvida exclusivamente para:

- Laboratórios  
- Ambientes de estudo  
- Redes com autorização explícita  

O uso não autorizado pode ser ilegal dependendo da jurisdição.

Você é totalmente responsável pelo uso desta ferramenta.

---

# 🛠 Stack

- 🦀 Rust  
- 🔎 Nmap  
- 📦 Regex crate  

---

# 👾 Authors

**Rui A. Siqueira**  
https://github.com/ruiasiqueira  

**Davi Guerra**  
https://github.com/daviguerra05  

**Kaiky Alvaro**  
https://github.com/kaikyalvaro1708  

---

