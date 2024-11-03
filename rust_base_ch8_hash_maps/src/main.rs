use std::collections::HashMap;

fn main() {
    println!("Creazione Hashmap e ottenimento valori");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name =  "Blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("blue-team score: {}", score);
    println!("\n\nIterazione su mappa");
    for (key, value) in &scores {
        println!("{key} : {value}");        
    }
    println!("\nPer quanto riguarda ownership:");
    println!("se elementi (chiavi o val) implementano copy si fa copia");
    println!("altrimenti ownership passa a hashmap");
    println!("\n\nAggiunta chiave-val SOLO se NON presente");
    scores.entry(String::from("Yellow")).or_insert(100); //non inserisce
    scores.entry(String::from("Green")).or_insert(100); // inserisce
                                                                 
    println!("{scores:?}");
    println!("\n\nAggiornamento valori in base a val precedente");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    println!("\n\nSI PUO PERSONALIZZARE HASH FUNCTION (ch10)\nFARE I 3 ESERCIZI NEL SUMMARY!");
}
