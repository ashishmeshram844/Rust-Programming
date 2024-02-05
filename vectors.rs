 fn main(){
    let mut my_vec = vec![1,2,3];           // initializing a vector
    println!("{}",my_vec[0]);

    my_vec.push(49);        //add extra item in vector at last position
    println!("{}",my_vec[my_vec.len()-1]);      
    println!("{:?}",my_vec);        //prints while vector

    my_vec.remove(0);           // remove index value
    println!("{:?}",my_vec);


 }


 