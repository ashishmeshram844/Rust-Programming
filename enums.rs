
enum AvailNames {
    Ashish,
    Ram,
    Sham
}

fn main(){
    let ashish:AvailNames = AvailNames::Ashish;
    let ram:AvailNames = AvailNames::Ram;
    let sham:AvailNames = AvailNames::Sham;
    
    match ram {
        AvailNames::Ashish => println!("Ashish is here"),
        AvailNames::Ram => println!("Ram is here"),
        AvailNames::Sham => println!("Sham is here"),
    }

} 