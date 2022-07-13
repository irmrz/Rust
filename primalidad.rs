use std::io;

fn main() {
    println!("\nEste programa verifica si un número es primo.");
    println!("Para salir del programa, escriba 'salir' o 'q'.");

    loop {
        println!("\nIngrese el número que desea verificar:");

        let mut x = String::new();
        io::stdin().read_line(& mut x).expect("Fallo al leer la entrada");

        if x.trim() == "salir" || x.trim() == "q" {
            break;
    }

    let x: u32 = match x.trim().parse() {
        Ok(x) => x,
        Err(_) => continue,
         };

         println!("\n{} es primo? -> {}.",x,es_primo(x))

        }     
}

fn es_primo(n: u32) -> bool{
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}