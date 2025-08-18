//  enum Result<T,E>{
//         Ok(T),
//         Err(E),
//     }

fn divide(numerator:f64,denominator:f64) -> Optiont<f64> {
    if denominator == 0.0 {
        None
    } else{
            Some(numerator/denominator)
        }
}

fn main(){
    let results = divide(10.0,0.0);
}