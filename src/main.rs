use std::process::Command;

fn is_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

fn main() {
    let pokemon = Command::new("pokemon-colorscripts")
        .arg("-r")
        .arg("--no-title")
        .output()
        .expect("pokemon-colorscripts should be installed");
    let pokemon = String::from_utf8_lossy(&pokemon.stdout);
    let neofetch = Command::new("neofetch")
        .arg("--backend")
        .arg("off")
        .output()
        .expect("neofetch should be installed");
    let neofetch = String::from_utf8_lossy(&neofetch.stdout);
    let neofetch_lines: Vec<&str> = neofetch.split("\n").collect();
    let pokemon_lines = pokemon.split("\n");
    for (i, line) in pokemon_lines.into_iter().enumerate() {
        if is_whitespace(line) {
            continue;
        }
        if neofetch_lines.len() > i {
            //println!("{}    - {}",line,neofetch_lines[i]);
            println!("{}    - {}",line, neofetch_lines[i]);
        }else {
            println!("{}    - cool guy",line);
        }
    }
}
