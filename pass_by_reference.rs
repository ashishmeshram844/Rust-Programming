
struct TupColor(u8,u8,u8);

fn main(){

    let tuple_struct : TupColor = TupColor(255,15,141);
    print_color(&tuple_struct)
}

fn print_color(color : &TupColor){
    println!("{} {} {}",color.0,color.1,color.2)
}


