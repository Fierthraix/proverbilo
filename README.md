# Proverbilo
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
