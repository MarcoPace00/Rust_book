fn main() {
    println!("String literals immutabili, in generale meglio usare classe String");
    {    
        let mut s = String::from("hello");
        s.push_str(", world!");  // to append si puo fare perche va in heap, non stack!
        println!("{s}");
    }
    println!("La memoria per s viene deallocata quando non è più nello scope");
    println!("\nAssegnamenti di oggetti rendono non validi il primo oggetto!!!");
    println!("\nUSARE CLONE per fare deep-copy!");
    let s1 = String::from("hello");
    let s2 = s1;   //DOPO QUESTO ASSEGNAMENTO S! NON PIU VALIDO
    let s3 = s2.clone();

    take_ownership(s3);
    //println!("{s3}");  // NON COMPILA: 
    //ownership ceduta a take_ownership, usare referenza
    //oppure .clone()
    //println!("{s1}, world!");    // NON COMPILA S1 NON ESISTE PIU

    println!("Con tipi primitivi salvati nello stack il problema non si pone");
    println!("\nOGGETTI-VALORI RESTUTUITI PASSANO OWNERSHIP A CHIAMANTE");

    println!("{}",take_and_give_back(String::from("Chi sono io?")));

}

fn take_ownership(b_string: String) {
    println!("Ho rubato s3: {b_string}");
    println!("USARE PASSAGGIO PER REFERENZA");

}

fn take_and_give_back(a_string: String) -> String{
    a_string.clone()
}
