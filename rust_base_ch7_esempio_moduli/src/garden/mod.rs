pub struct Vegetale{
    name: String
}

impl Vegetale {
    pub fn print_hello(&self){
        println!("Ciao, io sono {0} il vegetale", self.name);
    }
    pub fn make_vegetale(name: String) -> Vegetale{ //oggetto prende ownership stringa
        Vegetale {
            name,  // non necessario : name, vedi capitolo struct
        }
    }
}
