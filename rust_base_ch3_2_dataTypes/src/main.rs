fn main() {
    println!("Gestione della conversione stringa => u32\n\n\n");

    let guess: u32 = "42".parse().expect("Not a number!");
    let guess_2 : u32 = match  "42".parse() {
        Ok(x)  => x,
//        Err(_) => 1,
        Err(_) => {
            println!("Errore nella conversione");
           1
        }, 
    };
    println!("\n\n\nScalar types per rappresentare un singolo valore\nCi sono signed e unsigned\n\n\n");
    println!("Tipi di interi (float analoghi con f al posto di i):");
    println!("i8, i16,i32, i64, i128, isize con segno\n");
    println!("u8, u16,u32, u64, u128, usize SENZA segno\n");

    println!("\nTest divisione intera: 7/8 = {}, 7.0/8.0 = {}", 7/8, 7.0/8.0);

    println!("\n\nIl tipo char sembra supportare pure le emoticon");

    println!("\n\nCompound types per raggruppare piu valori: tuple ed array.");
    
    let tup: (i32, f64, u8) = (500,6.4,1);
    let (x,y,z) = tup;
    println!("\nTuple comode, ad esempio, per assegnamenti multipli-destructuring (riga 24-25).");
    
    println!("\nAccesso agli elementi della tupla con . tipo tup.0 = {}", tup.0);

    println!("\nArray invece sono collezioni di  elementi DELLO STESSO TIPO");
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1,2,3,4,5];
    let b = [3;5];  // a =[3, 3, 3, 3, 3];
    println!("Possono essere creati inizializzati a uno specifico valore.");
    

}
