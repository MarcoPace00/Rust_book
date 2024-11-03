pub mod garden; // dichiarazione del modulo garden
use crate::garden::Vegetale;


fn main() {
    println!("Hello, world!");
    let nome_vegetale = String::from("Asparago");
    let veggy = Vegetale::make_vegetale(nome_vegetale);
    veggy.print_hello();
}
