
        // Error Handling
    // Approach 1 -->  Using Options



// enum Option<T>{ //Defining the generic type
//     Some(T), //Rep a value
//     None, //Represents none        
        
//     }
fn divideOption(numerator:f64,denominator:f64) -> Option<f64>{
    if denominator == 0.0{
        None
    } else {
        Some(numerator/denominator)
    }
}


// enum Result<T,E>{
//     ok(T),
//     Err(E),
// }
fn divideResult(numerator:f64,denominator:f64) -> Result<f64,String>{
   if denominator == 0.0{
     Err("Failed to divide by 0".to_string());
   } else{
        ok(numerator/denominator);
   };
}



fn main(){
    let result = divideOption(10.0,2.0);
    match result{
        Some(ans)=> println!("Result: {}",ans),
        None => println!("Cannot divide by Zero"),
    };

    match divideResult(10.0,0.0){
        ok(_result) => println!("Result: {}",_result),
        Err(err) => println!("Error: {}",err),
    }
}