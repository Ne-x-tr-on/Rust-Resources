fn main(){
    let array: [i32; 10] = [12,23,5,45,1,3,2,6,7,81];
    println!("This is an array {:?} of numbers",array);
    let fruits: [&str; 4] = ["apple", "banana", "cherry", "date"];
    //All elements
    println!("Fruits Array: {:?}",fruits);
    //First Element
    
    println!("First Array: {:?}",fruits[0]);
}