pub mod serdecrate;

fn main() {
    let dog1 = Dog{
        name:"Tommy".to_string(),
        year_born: 2025,
    };
    let dog_ser = to_string(&dog1);
    if dog_ser.is_ok{
        println!("{}",dog_ser.ok().unwrap())
    }
}


