fn main(){
    for i in (1..40000).rev(){
        println!("{}",i);
    }
    let rng = 1..400;
    for i in rng {
        println!("{:?}",i);
    }
    let names = ["ashish","ram","sham"];
    for i in names.iter(){
        println!("{}",i);
    }
    println!(" all names : {:?}",names);

    // iterate with index and array element
    let fruits = vec!["mango","apple","banana"];
    for (index,val) in fruits.iter().enumerate() {
        println!("{} : {}",index,val);
    }
}   