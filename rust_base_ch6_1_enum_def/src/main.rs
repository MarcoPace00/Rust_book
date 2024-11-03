enum IpAddressKind{
    V4,
    V6,
}
struct IpAddr_0 {
    kind : IpAddressKind,
    address: String,
}


enum IpAddr {   //per evitare struct con enum tipo IpAddressKind e valore
        V4(String),
        V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {
    println!("\n\nEsempi di enum e def funzioni");
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    route(four);
    println!("");
    let home = IpAddr_0 {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{}", home.address);
    println!("\n\nEnum si usano anche per evitare struct come sopra (ridondanti)");
    let home_2 = IpAddr::V4(String::from("127.0.0.1"));
    println!("Mi turba un pochino: sembra che se lo scopo di enum è essere campi di struct puoi dichiarare direttamente l'enum e 'trattarlo' come uno struct... SE SI HANNO 4 TIPI DI MESSAGGIO NON SERVE DICHIARARE 4 STRUCT ENUM");
    println!("\n\nENUM OPTION PER IMPLEMENTARE NAN");
    let some_number = Some(5);  // tipo Option<i32>
    let absent_number: Option<i32> = None; // tipo nan
    println!("Option OBBLIGA a gestire caso nan... usare .is_some()-.is_none");
    println!("https://doc.rust-lang.org/std/option/enum.Option.html");
    println!("\n\nBEST PRACTICE GESTIONE NEL PROSSIMO CODICE");

}


fn route(ip_kind: IpAddressKind) {
    println!("Ok");
}

//enum Option<T> {  //NON SERVE AGGIUNGERLO O IMPORTARLO, c'è gia di default
//   None,
//   Some(T),
//}
