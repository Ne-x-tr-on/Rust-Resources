// Each value in Rust has a variable that's its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.


// // Each value in Rust has a variable that's its owner.
// fn main(){
//     let s1 = String::from("Rust");
//     let len = calc_length(&s1);
//     println!("Length of {} is {}",s1,len);
// }

// fn calc_length(s:&String) -> usize{
//     let act = s.len();
//     act
// }


// // There can only be one owner at a time.
// fn main(){
//     let c =String::from("Cat");
//     let b = c;
//     println!("{}",b)
// }

// When the owner goes out of scope, the value will be dropped.
fn main(){
    let s1 = String::from("Cats");
    let len = calc_length(&s1);
    println!("For {} the length is {}",s1,len);
}

fn printLost(s:&String){
    println!("{}",&s);
}

fn calc_length(s:&String) -> usize{
    s.len()
}
