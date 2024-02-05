use std::env;

fn main(){
    let args : Vec<String> = env::args().collect();         // read command line passed arguments

    for argument in args.iter(){
        println!("{}",argument)
    }
}

