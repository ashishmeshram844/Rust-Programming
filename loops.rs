fn main(){
    let mut n: i128 = 0;
    loop {
        n = n+1;
        if n == 95{
            continue;
        }
        if n > 100{
            break;
        }
        println!("the value is {}",n);
    }
}