fn main() {
    println!("\n\nStruct sono concettualmente simili a tuple ma hanno anche metodi");
    
    println!("Per modificare struct bisogna fare istanza mut, NON necessario fare campi mut (e fare alcuni campi mut non compila)");
    let mut user1 = User{  //l'ordine dei campi può cambiare in inizializzazione
        active : true,
        username: String::from("someusername"),
        email: String::from("somemail@gmail.com"),
         sign_in_count : 1,
    };
    println!("\n\nEsempio:");
    println!("{}", user1.username);
    user1.username = String::from("NEWusername");
    println!("{}", user1.username);

    println!("\nFIELD INIT SHORTHAND");
    println!("\n1. Pro-move per inizializzazione: best practice fare funzione e nomi parametri uguali a nomi campi non serve ripeterli");
    let user_2 = build_user(String::from("mail2@gmail.com"),String::from("username_secondo_tizio"));
    println!("{}", user_2.username);

    println!("\n2. Update syntax per copiare campi da altro struct");
    let user_3 = User{
        username : String::from("username_terzo_tizo"),
        ..user_2
    };
    println!("{}", user_3.username);

    println!("\nTUPLE STRUCTS\n\nSono tuple per migliorare leggibilita; si possono definire senza inizializiare come tuple vuote");
    let black = Color(0,0,0);
    let origin = Point(0,0);
    let tbd = ToBeDefined;

    println!("\n\nPer passare referenze a struct serve GESTIRE LIFETIMES, necessari a garantire al compilatore l'esistenza del valore puntato per tutto il ciclo di vita dello struct");
}

struct Color(i32,i32,i32);
struct Point(i32,i32);
struct ToBeDefined;

fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        username,       //non serve specificarli
                        //perchè parametri hanno 
                        //lo stesso nome dei campi
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
