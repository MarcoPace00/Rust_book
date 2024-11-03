#[derive(Debug)]
enum State{
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}
impl Coin{
    fn value_in_cents(&self) -> u8{
        match self {
            Coin::Penny => {
                            println!("Lucky penny!");
                            1
                            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                                    println!("Quarter from: {state:?}!");
                                    25
            }, 
        }
    }
}


fn main() {
    println!("\n\nPattern match usato per gestire enum (e quindi anche nan)");
    let moneta_1 = Coin::Penny;
    let moneta_2= Coin::Quarter(State::Alabama);
    println!("{}-{}", moneta_1.value_in_cents(),
                     moneta_2.value_in_cents());

    println!("\nMatch per gestione Option - nan");

    let x : Option<u16>  = Some(3);
    let y : Option<u16>  = None;
    plus_one(x);
    plus_one(y);
    println!("CATCH ALL PATTERNS");

    let dice_roll = 7;
    match dice_roll{
        3 => println!("Chiamata funzione 1"),
        7 => println!("Chiamata funzione 2"),
        _ => (), // per non eseguire codice
        //_ => println!("Funzione default"),
    }
    println!("\n\nProva del costrutto if let");
    test_if_let(x);
    test_if_let(y);

    println!("");
    println!("");
}

fn plus_one(x: Option<u16>) -> Option<u16>{
    match x {
        Some(i) => Some(i+1),
        None => {
            println!("None value!");
            None
        },
    }

}

fn test_if_let(x : Option<u16>) {
    if let Some(max) = x {
        println!("Maximum set to {max}");
    } else {
        println!("None Value");
    }
}
