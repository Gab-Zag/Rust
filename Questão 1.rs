use std::io; // inclui a biblioteca std::io

// função main
pub fn main() {
    let mut n1_input = String::new(); // cria n1_input como string mutavel
    let mut n2_input = String::new(); // cria n2_input como string mutavel
    let mut op = String::new(); // // cria op como string mutavel
    let res: i32; // cria res como inteiro 32 bits
    let number1: i32; // cria number1 como inteiro 32 bits
    let number2: i32; // cria number2 como inteiro 32 bits

    io::stdin().read_line(&mut n1_input).expect("failed to read line"); // lê o n1_input
    io::stdin().read_line(&mut n2_input).expect("failed to read line"); // lê o n2_input

    println!("Somar ou multiplicar? (+ ou *)"); // printa se quer somar ou multiplicar
    io::stdin().read_line(&mut op).expect("failed to read line"); // lê o op

    number1 = n1_input.trim().parse().unwrap(); // define que o number1 é igual ao n1_input convertido pra numero
    number2 = n2_input.trim().parse().unwrap(); // define que o number2 é igual ao n2_input convertido pra numero

    if op.trim() == "+" { // verifica se o operador é igual a +
        res = number1 + number2; // define que o resultado é a soma dos dois números
        println!("Soma {} + {} = {}", number1, number2, res) // imprime o resultado
    } else { // se não for igual a +, é multiplicação
        res = number1 * number2; // o resutaldo é a multiplicação dos dois números
        println!("Multiplicacao {} * {} = {}", number1, number2, res) // imprime o resultado
    }
}
