use std::io;

fn compara(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    } else if x < y {
        return y;
    } else {
        return -1;
    }
}
fn mostrar(x: i32) -> () {
    println!("{}", x);
}
fn main() {
    let mut x: i32 = 10;
    let mut Y: i32 = 5 + 5;
    let mut z: i32 = compara(x, y);
    while z == 20 {
        z += 2;
    }
    let mut i: i32 = 0;
    while i < 20 {
        let mut num: i32 = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Falha ao ler linha");
            input.trim().parse().unwrap_or_else(|_| 0)
        };
        if num < 10 {
            println!("{}", num);
        } else if num == 10 {
            break;
        } else {
            continue;
        }
        i += 1;
    }
    mostrar(20, 30);
    x = 10 + 35 - (-10 * 3) + 5;
    let mut letra: char = 'a';
    let mut float: f32 = -1;
    if float + 1 == 10 || compara(x, y) == -1 {
        while true {
            i;
            while i > 20 {
                break;
                i += 1;
            }
            break;
        }
    }
}
