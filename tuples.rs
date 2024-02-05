
fn main(){
    let s_tuple = ("name","mobile","address");
    let (a,b,c) = s_tuple;
    println!("{}",s_tuple.0);     // accessed tupled index value
    println!("{:?}",s_tuple);
    println!("{}{}{}",a,b,c);
    
}