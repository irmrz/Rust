// Fibonacci:
// f(0) = 0
// f(1) = 1
// f(n) = f(n-1) + f(n-2)

use std::io;

fn main() {
   println!("Para salir del programa escriba 'salir'");

   loop {
       println!("Ingrese el n-esimo Fibonacci que busca.");

       let mut int = String::new();
       io::stdin().read_line(& mut int).expect("Fallo al leer la entrada");

       if int.trim() == "salir" {
           break;
       }
   
        let int: i32 = match int.trim().parse() {
       Ok(int) => int,
       Err(_) => continue,
        };
        
        println!("\nFibonacci({}) = {}\n",int,fibonacci(int));
    }
}

fn fibonacci (n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } 
        return fibonacci(n-1) + fibonacci(n-2) 
}