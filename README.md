# Compilador
## Colaboradores
João Pedro Bazoli Palma RA:24.123.041-5 <br>


## Dependências
Rust
Cargo
<br>
## Uso

Crie um arquivo Code.txt e preencha ele com o codigo que quer analizar, ou descomente as seguintes linhas no main.rs e personalize o file.write:

``` rust
  {
    use std::io::Write;

    let mut file = File::create("Code.txt").unwrap();
    file.write(b"Hello World").unwrap();
  }
```

No diretório /Compilador<br>
Compile o programa com:
``` bash
cargo build
```
Depois teste ele com:
``` bash
cargo run
```

