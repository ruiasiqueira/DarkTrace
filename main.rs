use std::process::Command;
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔎 Detectando subnet local...");

    let subnet = get_subnet()?;
    println!("📡 Escaneando rede: {}\n", subnet);

    let output = run_nmap(&subnet)?;

    println!("🖥 Dispositivos encontrados:\n");

    parse_nmap_output(&output);

    Ok(())
}

fn get_subnet() -> Result<String, Box<dyn Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ip -4 addr show | grep inet | grep -v 127.0.0.1")
        .output()?;

    let result = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"inet (\d+\.\d+\.\d+)\.\d+")?;

    if let Some(cap) = re.captures(&result) {
        return Ok(format!("{}.0/24", &cap[1]));
    }

    Err("Não foi possível detectar subnet".into())
}

fn run_nmap(subnet: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("sudo")
        .args([
            "nmap",
            "-O",              // OS detection
            "-sS",             // SYN scan
            "--osscan-guess",  // melhora precisão
            subnet,
        ])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_nmap_output(output: &str) {
    let ip_re = Regex::new(r"Nmap scan report for (\d+\.\d+\.\d+\.\d+)").unwrap();
    let os_re = Regex::new(r"OS details: (.+)").unwrap();
    let mac_re = Regex::new(r"MAC Address: ([0-9A-F:]+) \((.+)\)").unwrap();

    let mut current_ip = String::new();

    for line in output.lines() {
        if let Some(cap) = ip_re.captures(line) {
            current_ip = cap[1].to_string();
            println!("IP: {}", current_ip);
        }

        if let Some(cap) = mac_re.captures(line) {
            println!("MAC: {} ({})", &cap[1], &cap[2]);
        }

        if let Some(cap) = os_re.captures(line) {
            println!("SO detectado: {}", &cap[1]);
            println!("----------------------------------------");
        }
    }
}
