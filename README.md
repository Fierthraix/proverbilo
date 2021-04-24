# Proverbilo
Versio de [ProverbMontrilo](https://github.com/MJWootton/ProverbaroMontrilo) verkita per [Rust](https://www.rust-lang.org/).

Montras hazardajn proverbaojn kaj diraĵojn en terminalo.

### Uzo 
```
$ proverbilo 5
Petolu, diboĉu, sed poste sorton ne riproĉu
Fari aplaŭdon sur la vangon
Karaktero olea
Malsaĝulo en foiro estas bona akiro
Lerteco sorĉon ne bezonas
```
#### Ekprintu proverbon kiam malfermas terminalon:
Eki la progragam ĉiufoje kiam oni ensalutas terminal per aldono de ĉi-tiu linio en la dosiero `~/.bashrc` (aŭ `~/.zshrc`, se uzas oni tion).
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
