// fn main() {
//  let mut x = 5;
//  println!("The value of x is: {x}");
//  x = 6;
//  println!("The value of x is: {x}");



// }


// Shadowing
// fn main() {
//  let x = 5;
//  let x = x + 1;
//  {
//  let _x = x * 2;
//  println!("The value of x in the inner scope");
//  }
//  println!("The value of x is: {x}");
// }


fn main(){
    print_labelled_measurements(5,'h');
}

fn print_labelled_measurements(value:i32,unit_label:char){
    println!("The measurement is: {value}{unit_label}");
}