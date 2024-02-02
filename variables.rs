use std::io::stdin;


const VERSION: f32 = 1.2;          // conatant variable define

fn main(){
    println!("Version is {}",VERSION);
    let fname: &str = "Ashish";       //string  // immutable by default
    let lname: &str = "Meshram";      // string
    let mut age: u8 = 25;             //unsigned integer      // mutable
    println!("my name is {} {} and age is {}",fname,lname,age);
    age = 26;                          // change variable value
    println!("now my age is : {}",age);
    let alive: bool = true;             // boolean
    println!("alive values is {}",alive);
    let starts: char = 'A';             // character
    println!("my name starts with {}",starts);


}

