fn main(){
    let mut name = String::from("My name is Aashish");
    println!("{}",name);
    println!("length of given string is {}",name.len());
    println!("length of given string is empty   {}",name.is_empty());

    // split string into one by one character
    for token in name.chars(){
        println!("{}",token);
    }

    // split string from passed character
    for token in name.split(" "){
        println!("{}",token);
    }


    println!("does string contain  'is' : {}",name.contains("is"));            // check string contain


    name.push_str(" here is pushed string ...");            // concate string
    println!("{}",name)
}