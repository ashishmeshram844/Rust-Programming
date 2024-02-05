
fn print_name(){
    println!("my name is ashish");
}

fn add_nums(num1:i64, num2:i64){
    let sum = num1 + num2;
    println!("{} + {}  = {}",num1,num2,sum);
}

pub fn is_even(num:i64) -> bool {
    if num % 2 == 0{
        return true;
    }
    return false;
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

    let res = is_even(20);
    println!("{}",res);
}

