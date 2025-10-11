// enum Option<T>{
// Some(T),
// None,
// }

fn divide(denominitor:f64,numerator:f64)-> Option<f64>{
    if denominitor == 0.0 {
        None
    } else {
        Some(numerator/denominitor)
    }
}

fn main(){
    let results = divide(10.0,20.0);
    match results {
        None => println!("Cannot divide by Zero"),
        Some(value) => println!("Value of division:\n{} ",value)
    }
}