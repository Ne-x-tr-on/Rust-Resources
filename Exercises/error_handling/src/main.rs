    // Error Handling
    // Approach 1 -->  Using Options

// enum Option<T>{ //Defining the generic type
//     Some(T), //Rep a value
//     None, //Represents none        
        
//     }

fn divide(numerator:f64,denominator:f64) -> Option<f64>{
    if denominator == 0.0{
     None
    }
     else {
      Some(numerator/denominator)
       }
 } 


    // enum Result<T,E>{
    //     ok<T>,
    //     Err<E>,
    // }

fn main(){
    let result = divide(10.0,7.0);
    match result{
        Some(x) => println!("Results {:.2}",x),
        None => println!("Cannot divide by Zero"),
    }
}