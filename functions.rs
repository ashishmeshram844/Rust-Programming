
fn print_name(){
    println!("my name is ashish");
}

fn add_nums(num1:i64, num2:i64){
    let sum = num1 + num2;
    println!("{} + {}  = {}",num1,num2,sum);
}



fn main(){
    print_name();
    add_nums(5,10);

    fn convert_lower(string: &str){
        let _new_str  = string.to_ascii_lowercase();
        println!("{}",_new_str);
        println!("{}",string)
    }
    fn find_len(string : &str){
        println!("{}",string.len())
    }

    convert_lower("ASHISH");
    find_len("ASHISH");
}
