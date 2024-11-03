fn main() {
    println!("Hello, world!");

    altra_funzione();
    funzione_con_parametro(5, 'h');
    println!("Funzione che resituisce intero: {}", plus_five(10));
}

fn altra_funzione(){
    println!("Questa è un'altra funzione.");
}

fn funzione_con_parametro(x: i32, label: char){
    println!("Il valore numerico passato a quest'altra funzione è {}",x);
    println!("Il valore char passato a quest'altra funzione è {}",label);
    println!("Il tipo dei parametri DEVE essere specificato");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.

fn plus_five(x: i32) -> i32{
    x + 5
}
