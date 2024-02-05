fn main(){
    let roll_nos : [i32;7] = [1,2,3,4,5,6,8];               // [type of data ;number of elements]
    println!("length of roll_no is {}",roll_nos.len());
    for i in roll_nos.iter(){
        println!("{}",i);
    }

    for (index,i) in roll_nos.iter().enumerate(){
        println!("{} = {}",index,i);
    }
    println!("First value of array is {}",roll_nos[0]);
    println!("last element of array is  : {}",roll_nos[roll_nos.len() - 1]);
}
