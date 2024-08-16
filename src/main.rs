use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=String::from("random"))]
    pokemon_name: String,
}

fn get_line_width(s: &str) -> i32 {
    let mut line_width = 0;
    for c in s.split("") {
        if c == "▀" || c == " " || c == "▄"{
            line_width += 1
        }
    }
    return line_width;
}

fn main() {
    let args = Args::parse();
    
    let mut pokemon = Command::new("pokemon-colorscripts");
    if (args.pokemon_name) == String::from("random"){
        pokemon.arg("-r");
    }else {
        pokemon.arg("-n");
        pokemon.arg(args.pokemon_name);
    }
    pokemon.arg("--no-title");
    let pokemon_output = pokemon.output().expect("pokemon-colorscripts should be installed");
    let pokemon = String::from_utf8_lossy(&pokemon_output.stdout);
    let neofetch = Command::new("neofetch")
        .arg("--backend")
        .arg("off")
        .output()
        .expect("neofetch should be installed");
    let neofetch = String::from_utf8_lossy(&neofetch.stdout);
    let neofetch_lines: Vec<&str> = neofetch.split("\n").collect();
    let pokemon_lines:Vec<&str> = pokemon.split("\n").collect();
    let tab_width = 4;

    let mut max = neofetch_lines.len();
    if pokemon_lines.len() > max {
        max = pokemon_lines.len()
    }
    
    // calculate the pokemon ascii art width
    let mut width = 0;
    for line in pokemon_lines.iter() {
        let line_width = get_line_width(line);
        if line_width > width {
            width = line_width
        }
    }
    
    for i in 0..max {
        let mut spaces = width+tab_width;
        if pokemon_lines.len() > i {
            spaces = width - get_line_width(pokemon_lines[i])+tab_width;
        }
        let mut spaces_string = String::new();
        for _ in 0..spaces {
            spaces_string = spaces_string + " "
        }
        if neofetch_lines.len() > i && pokemon_lines.len() > i {
            println!("{}{spaces_string}{}",pokemon_lines[i], neofetch_lines[i]);
        }else if pokemon_lines.len() > i{
            println!("{}{spaces_string}",pokemon_lines[i]);
        }else {
            println!("{spaces_string}{}",neofetch_lines[i]);
        }
    }
}
