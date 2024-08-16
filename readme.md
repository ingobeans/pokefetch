# POKEFETCH

Pokefetch is a simple CLI tool written in Rust that combines the functionality of pokemon-colorscripts and neofetch. It replaces the ASCII art generated by neofetch with a random Pokémon.

# Usage
`pokefetch` - prints out neofetch information with a random pokemon

`pokefetch -p totodile` - pokemon specified to be totodile

`pokefetch -p pikachu -f alola-cap -s` - shiny pikachu with the alola-cap form

`pokefetch -p totodile --disable cpu gpu` - passes the `--disable cpu gpu` args to neofetch to hide cpu & gpu information.

# Install
### Install dependencies
Rust, pokemon-colorscripts and neofetch have to be installed for pokefetch to work.

Install rust:
```bash
sudo pacman -S rustup
rustup default stable
rustup update
```

Install neofetch:
```bash
sudo pacman -Syu neofetch
```
Install pokemon-colorscripts:
```bash
yay -S pokemon-colorscripts-git
```

### Clone the repo
```bash
git clone https://github.com/ingobeans/pokefetch.git
```

### Build

Use cargo to build pokefetch:
```bash
cd pokefetch
cargo build --release
```

The compiled binary will be located in the target/release/ directory.
### Add pokefetch to PATH

To run pokefetch from anywhere, add the binary to your PATH. You can do this by copying it /usr/local/bin:

```bash
sudo cp target/release/pokefetch /usr/local/bin/
```