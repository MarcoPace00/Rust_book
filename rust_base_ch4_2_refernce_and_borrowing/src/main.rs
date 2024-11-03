fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("\n\nPer evitare complicazioni da ownership conviene passare referenze a oggetti");
    println!("{s1}-{len}");

    println!("\n\nPassare per referenza è detto 'borrowing'");
    println!("\n\nPER MODIFICARE OGGETTO IN BORROWING USARE MUT (altrimenti non compila)");

    let mut s2 = s1.clone(); // .clone per mantenere attiva s1
    change_string(&mut s2);

    println!("\n{s2}");

    println!("- Si puo avere UNA SOLA mutable reference per valore");
    println!("- Tutto è finalizzato ad evitare data-races");
    println!("- Quindi NO mut e immutable, NO multi mut, SI 1 mut, SI multi immutable");
    println!("- SI multi immutable anche per tipo mutable");
    println!("\n\nECCEZIONE: il compilatore accetta infrangimento regole sopra se vede che gli scopes non si sovrappongono (mut-ref dopo 2 non-mut-ref ok se le non-mut ref non vengono mai usate dopo mut-ref)");

    println!("\n\nDangling ref non compilano 'cannot return refernce to data owned by the current function'");
}

fn calculate_length(s : &String) -> usize {
    s.len()
}
fn change_string(s : &mut String){
    s.push_str(", aggiunta in funzione");
}
