fn main() {
    println!("\n\nSi possono passare tuple come parametri di fuzioni");
    let (w,d) = (30,50);
    let tupla = (w,d);   // oppure direttamente tupla = (30,50);
    println!("{}", area(tupla));

    let rect1 = Rectangle{
        width : dbg!(20),  //per debugging
        height : 30,
    };

    println!("\nGli struct si possono passare a funzini per ref");
    println!("{}", area_2(&rect1));

    println!("\nImplementare dubug consente di ottenere un toString di default:");
    println!("rect1 is {rect1:?}");
    println!("Altro output:");
    println!("rect1 is {rect1:#?}");

    dbg!(&rect1);
    println!("Dbg probabilemente non capito completamente ma wow");
    println!("\nEsempio implementazione metodi");
    println!("{}-{}", rect1.area(), rect1.get_width());
}

fn area(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_2(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
} 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }
}
