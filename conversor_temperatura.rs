use std::io;

fn main() {
    println!("\nEste programa convierte temperaturas entre grados Fahranheit y Celsius.\n");

    loop {
        println!("Elija como quiere convertir los valores:\n");
        println!(" -0: Convertir de Fahranheit a Celsius\n");
        println!(" -1: Convertir de Celsius a Fahranheit\n");
        println!(" -2: Salir\n");

        let mut comando = String::new();
        match io::stdin().read_line(&mut comando) {
            Ok(_) => {}
            Err(_) => {
                println!("Fallo al leer el comando.");
                continue;
            }
        }

        let comando: i64 = match comando.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Comando inválido.");
                continue;
            }
        };
        match comando {
            0 => fahranheit_a_celsius(),
            1 => celsius_a_fahrenheit(),
            2 => break,
            _ => println!("Comando inválido."),
        }
    }
}

fn fahranheit_a_celsius() {
    println!("\nIngrese la temperatura en Fahrenheit:");

    let mut temperatura = String::new();
    match io::stdin().read_line(&mut temperatura) {
        Ok(_) => {}
        Err(_) => {
            println!("Fallo al leer la temperatura.");
            return;
        }
    }

    let temperatura: f64 = match temperatura.trim().parse(){
        Ok(t) => t,
        Err(_) => {
            println!("Temperatura inválida.");
            return
        }
    };
    let convertido = (temperatura -32.0) / 1.8;
    println!("\n{} Fahranheit = {} Celsius\n", temperatura, convertido);
}    

fn celsius_a_fahrenheit() {
    println!("\nIngrese la temperatura en Celsius:");

    let mut temperatura = String::new();
    match io::stdin().read_line(&mut temperatura) {
        Ok(_) => {}
        Err(_) => {
            println!("Fallo al leer la temperatura.");
            return;
        }
    }

    let temperatura: f64 = match temperatura.trim().parse(){
        Ok(t) => t,
        Err(_) => {
            println!("Temperatura inválida.");
            return
        }
    };
    let convertido = (temperatura * 1.8) + 32.0;
    println!("\n{} Celsius = {} Fahrenheit\n", temperatura, convertido);
}  