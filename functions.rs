
fn print_name(){
    println!("my name is ashish");
}

fn add_nums(num1:i64, num2:i64){
    let sum = num1 + num2;
    println!("{} + {}  = {}",num1,num2,sum);
}

fn main(){
    print_name();
    add_nums(5,10)
}
