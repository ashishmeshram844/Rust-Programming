struct Colors {
    red : u8,
    green : u8,
    blue : u8
}


fn main(){
    let mut background : Colors  =  Colors {
        red : 255,
        green : 15,
        blue : 154
    };
    background.red = 199;
    println!("{} {}",background.red,background.green);

    // tuple structure
    struct TupColor(u8,u8,u8);
    let tuple_struct : TupColor = TupColor(255,15,141);
    println!("{} {} {}",tuple_struct.0,tuple_struct.1, tuple_struct.2)
}



