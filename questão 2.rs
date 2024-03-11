use std::io;

fn fill_array_with_multiples(value: i32) -> [i32; 10] {
    let mut result = [0; 10];

    for i in 0..10 {
        result[i] = value * (i as i32);
    }

    result
}

fn main() {
    println!("Digite um número para multiplicar:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    let value: i32 = input.trim().parse().expect("Entrada inválida");

    let array = fill_array_with_multiples(value);

    println!("Array resultante: {:?}", array);
}
