fn main() {
    println!("\n\nScript test vettori. let: Vec<i32> = Vec::new() per crearne uno vuoto");
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    v1.push(9);
    println!("Altre pro-move con macro per inizializzare vettori");
    let v2 = vec![1,2,3];  //compilatore deduce tipo da valori con cui si inizializza
    println!("Usare Option per accedere ad elementi del vettore con .get");

    if let Some(value) = v2.get(2) {
        println!("v2 ha come terzo valore {}", value);
    } else {
        println!("v2 non ha un terzo elemento");
    }
    if let Some(value) = v2.get(7) {
        println!("v2 ha come ottavo valore {}", value);
    } else {
        println!("v2 non ha otto valori!!");
    }

    println!("\nIterazione su vettore mut e non mut.\nMut:");
    for i in &mut v1 {
        println!("Valore prima: {i}");
        *i += 50; //dereferencing
        println!("Valore dopo: {i}");
    }
    println!("\nNon mut");
    for i in v2 {
        println!("Val: {i}");
    }
    println!("\n\nPer salvare tipi diversi conviene usare enum");
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("");
    println!("");
}
