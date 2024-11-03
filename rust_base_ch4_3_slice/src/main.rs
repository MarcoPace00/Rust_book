fn main() {
    println!("\n\nUno Slice è una referenza ad elementi adiacenti di una collezione");
    println!("\nProblema esempio:write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.");
    let mut s = String::from("Hello world");
    let end_first_word = first_word(&s);
    println!("{}", end_first_word);

    println!("\n\nString slices più pratiche.");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}-{}", hello,world);

    println!("\n\n{}", first_word_2(&s));
    s.clear(); // svuota stringa, ora è uguale a ""
    println!("\nAccedere a slices dopo clear della stringa non compila.");
    println!("\n\nCONVIENE FARE FUNZIONI CON INPUT &str NON &String: codice più flessibile e passare stringhe NON genera errori");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //conversione ad array
    println!("\nPrima implementazione:\n1. Converto ad array con .bytes()");
    println!("2. ottengo elementi(ref) array con iterator e enumerate fa wrap dell'elemento con indice numerico");
    println!();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()

}

fn first_word_2(s: &String) -> &str {
    println!("\n\nSeconda implementazione con string slices.");
    println!("Restituisco slice");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
