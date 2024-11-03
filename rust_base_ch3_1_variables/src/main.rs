fn main() {
    let x = 5;
    println!("The value of x is {} if I change it it'll rise errors", x);
    let mut y = 7;
    y = 8;
    println!("The value of y is {} and I can change it because is mutable", y);
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("Constants cannot be converted to mutables: {}", THREE_HOURS_IN_SECONDS);

    println!("Shadowing can be used for overwring not mutable variables.");
    println!("Current value of x: {}", x);
    {
        let x = x + 1;
        println!("The value of x inside the scope is {}", x);
        
    }
    //const PROVA: u32 =  x;  // NON funziona
    //println!("Test costante da variabile: {}", PROVA);
    //
    println!("\n\nLET MUT CAMBIA VALORE MA NON TIPO \n\n LET PUO CAMBIARE TIPO MA NON VALORE")
}
