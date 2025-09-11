# Compilador
## Colaboradores
João Pedro Bazoli Palma RA:24.123.041-5 <br>

## Instruções de instalação e uso
Tenha Rust + Cargo na sua maquina (de preferência na versão mais recente)<br>

Para personalizar o arquivo "Code.txt" que é analisado pelo Lexer, basta descomentar essas linhas de codigo na main.rs e personalizar o "Hello World" 
``` rust
  {
    let mut file = File::create("Code.txt").unwrap();
    file.write(b"Hello World").unwrap();
  }
```

No diretório Compilador<br>
Use o comando:
``` bash
cargo build
```
Isso vai compilar o programa <br>

Depois use o comando:
``` bash
cargo run
```
Isso vai roda-lo

