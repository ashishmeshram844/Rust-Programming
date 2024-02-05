fn main(){
    let number : u8 = 4;

    match number {
        1 => {
            println!("ONE")
        },
        2 => println!("TWO"),
        3 => println!("THREE"),
        4 => println!("FOUR"),
        5 => println!("FIVE"),
        6..=10 => println!("Number is between 6 to 10"),        // between range
        _ => println!("NOT MATCHED")
    }
}