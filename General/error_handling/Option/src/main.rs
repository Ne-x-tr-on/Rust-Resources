// enum Option<T>{
//     Some(T),
//     None,
// }

fn divide(numerator:f64,denominator:f64) -> Option<f64>{
    if denominator == 0.0 {
        None
    } else {
        Some(numerator/denominator)
    }
}

fn main(){
    let results = divide(10.0,1.0);
    match results{
        None => println!("Denominator cannot be 0.0 "),
        Some(val) => println!("{}",val)
    }
}