// Reference and Borrowing
// Safety and perfomance
// Create a reference using "&"

fn main(){
    let mut x:i32 =5;
    let mut r: i32 =x;
        r += 5;
    println!("The values are {} and {}",x,r);
}