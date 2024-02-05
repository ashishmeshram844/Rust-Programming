struct Person {
    name : String,
    age : u8
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!(" my name is {} and age is {}",self.name,self.age);
    }  
}

impl Person {
    fn call_me(&self){
        println!("CALLED CALL ME");
    }
}

fn main(){
    let ashish = Person { name : String::from("Ashish"), age : 25 };
    println!("{}",ashish.to_string());
    ashish.call_me();
}