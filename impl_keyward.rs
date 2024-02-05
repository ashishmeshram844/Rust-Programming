// class
impl Calculator{
    fn  addition(&self) -> i32{
        return self.num1 + self.num2;
    }

    fn  substraction(&self) -> i32{
        return self.num1 - self.num2;
    }

    fn  multiplication(&self) -> i32{
        return self.num1 * self.num2;
    }

    fn  division(&self) -> f32{
        return self.num1 as f32 / self.num2 as f32;
    }
}

// class constructor
struct Calculator {
    num1: i32,
    num2: i32
}


fn main(){
    let calcy = Calculator { num1:50, num2:20 };            // object
    println!("Addition of {} and {} is {}",calcy.num1, calcy.num2, calcy.addition());
    println!("Substraction of {} and {} is {}",calcy.num1, calcy.num2, calcy.substraction());
    println!("Multiplication of {} and {} is {}",calcy.num1, calcy.num2, calcy.multiplication());
    println!("Division of {} and {} is {}",calcy.num1, calcy.num2, calcy.division());
}

