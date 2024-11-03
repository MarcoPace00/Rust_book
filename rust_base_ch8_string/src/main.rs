fn main() {
    println!("\n\nCreazione furba string da literal");
    let mut s = "Questa è una stringa".to_string();
    s.push_str(", è mutable");
    println!("{s}");
    println!("\nOperatore +");
    let s2 = "tic".to_string() + "-" + "tac" + "-" + "toe";
    println!("{s2}");
    println!("\n\nTradizionale slicing può fare crashare");
    println!("Per iterare sulle lettere usare .char() o .bytes()");
    println!("\n\nConviene imparare metodi libreria std per cose comuni");
    println!("");
    println!("");
}
