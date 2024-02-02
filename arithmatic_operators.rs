fn main(){
    let num1:u16 = 124;
    let num2:u16 = 5;
    println!("Addition is {2} of {0} and {1}",num1,num2,num1+num2);         // positional arguments
    println!("Substraction of {} and {} is {}",num1,num2,num1-num2);
    println!("Multiplication of {} and {} is {}",num1,num2,num1*num2);
    println!("Division of {} and {} is {}",num1,num2,num1/num2);
}