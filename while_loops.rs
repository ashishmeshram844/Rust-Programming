fn main(){
    let mut number:i32 = 1;
    while number <= 20{
        println!("{}",number);
        number +=1;
    }

    // check odd even number 
    while number <= 40 {
        if number % 2 == 0 {
            println!("{} : even",number);
        }
        else{
            println!("{} : odd",number)
        }
        number +=1
    }
}