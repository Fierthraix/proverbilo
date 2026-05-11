# Proverbilo

[![CI](https://github.com/Fierthraix/proverbilo/actions/workflows/ci.yml/badge.svg)](https://github.com/Fierthraix/proverbilo/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/proverbilo.svg)](https://crates.io/crates/proverbilo)
[![Downloads](https://img.shields.io/crates/d/proverbilo.svg)](https://crates.io/crates/proverbilo)
[![Docs.rs](https://docs.rs/proverbilo/badge.svg)](https://docs.rs/proverbilo)
[![License](https://img.shields.io/crates/l/proverbilo.svg)](Cargo.toml)

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

## Instalado
### [`cargo`](https://doc.rust-lang.org/cargo/)
`cargo install proverbilo`

### Arĉ-Linuko
`yay -S proverbilo`
