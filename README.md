# Proverbilo

[![CI](https://github.com/Fierthraix/proverbilo/actions/workflows/ci.yml/badge.svg)](https://github.com/Fierthraix/proverbilo/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Fierthraix/proverbilo?display_name=tag)](https://github.com/Fierthraix/proverbilo/releases)
[![Crates.io](https://img.shields.io/crates/v/proverbilo.svg)](https://crates.io/crates/proverbilo)
[![Downloads](https://img.shields.io/crates/d/proverbilo.svg)](https://crates.io/crates/proverbilo)
[![Docs.rs](https://docs.rs/proverbilo/badge.svg)](https://docs.rs/proverbilo)
[![License](https://img.shields.io/crates/l/proverbilo.svg)](Cargo.toml)
[![AUR](https://img.shields.io/aur/version/proverbilo)](https://aur.archlinux.org/packages/proverbilo)
[![AUR bin](https://img.shields.io/aur/version/proverbilo-bin)](https://aur.archlinux.org/packages/proverbilo-bin)
[![AUR git](https://img.shields.io/aur/version/proverbilo-git)](https://aur.archlinux.org/packages/proverbilo-git)

Versio de [ProverbMontrilo](https://github.com/MJWootton/ProverbaroMontrilo) verkita per [Rust](https://www.rust-lang.org/).

Montras hazardajn proverbaojn kaj diraĵojn en terminalo.

### Uzado 
```bash
$ proverbilo 5
Petolu, diboĉu, sed poste sorton ne riproĉu
Fari aplaŭdon sur la vangon
Karaktero olea
Malsaĝulo en foiro estas bona akiro
Lerteco sorĉon ne bezonas
```
#### Ekprintu proverbon kiam malfermas terminalo:
Ekalvoki la progragam ĉiufoje kiam oni ensalutas terminalon per aldono de ĉi-tiu linio ĉe la fino de la dosiero `~/.bashrc` (aŭ `~/.zshrc`, se uzas oni tion).
```bash
# ~/.bashrc
#...

# Ĉe le fin' de l'dosiero: 
proverbilo
```

## Installation

```bash
cargo install proverbilo
yay -S proverbilo
yay -S proverbilo-bin
yay -S proverbilo-git
brew tap Fierthraix/tap
brew install --cask proverbilo
nix run github:Fierthraix/nur-packages#proverbilo
```

```powershell
scoop bucket add fierthraix https://github.com/Fierthraix/scoop-bucket
scoop install proverbilo
```

```text
deb/rpm/apk/tar/zip: https://github.com/Fierthraix/proverbilo/releases
```
