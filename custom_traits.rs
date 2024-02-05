struct Person {
    name : String,
    age : i8
}

// custom traits
trait HasVoiceBox {
    fn speak(&self);
    fn has_speak(&self)->bool;
}


// implement Custom trait in Person class
impl HasVoiceBox for Person {
    fn speak(&self){
        println!("hello my name is {}",self.name)
    }

    fn has_speak(&self) -> bool{
       return self.age > 5
    }
}

fn main(){
    let person = Person {
        name : String::from("Ashish"),
        age : 10
    };
    println!("Can {} speak ? {}", person.name,person.has_speak())

}


