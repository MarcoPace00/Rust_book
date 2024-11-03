fn main() {
    println!("\n\nIF-else-else if");
    let number = 10;
    if number < 10{
        println!("Numero minore di 10");
    } else if number == 10{
        println!("Numero uguale a 10");
    }
    else {
        println!("Numero maggiore di 10");
    }

//--------------------------------------------------
    println!("ITERAZIONE SU RANGE");
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("DECOLLO!");

//--------------------------------------------------
    println!("\nOPERATORE TERNARIO");

    let condition = false;
    let isOk = if condition {"Si, tutto ok"} else{"No, codition false"};
    println!("isOk: {}", isOk);

//--------------------------------------------------
    println!("LOOP");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the loop is {result}");let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
//--------------------------------------------------
    
    println!("ITERAZIONE SU ARRAY E FOREACH");
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
//--------------------------------------------------
    let b = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", b[index]);
        index += 1;
    }


}
