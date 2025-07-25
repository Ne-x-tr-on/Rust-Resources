#[derive(Debug)]
enum Coins{
    Penny,
    Nickel,
    Dime,
    Quater,
}


fn main() {
    // println!("Hello, David Kamau!");
// let value = value_in_cents(1);
// println!("You have inserted {value}");



}

  fn value_in_cents(coin:Coins) -> i8 {
        match coin{
            Coins::Penny => 1,
            Coins::Nickel => 5,
            Coins::Dime => 10,
            Coins::Quater (state) => {
                println!("State quater from {:?}",25)
            }

        }

        
    }
