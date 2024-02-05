use std::fs::File;
use std::io::prelude::*;


fn main(){  
    let mut file = File::open("array.rs").expect("failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("vant read the file ...");

    println!("file content is \n\n : {}",content)


}

