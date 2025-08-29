pub mod typecast;
fn main() {
    // println!("Hello, world!");
    typecast::basicmath();

    let results = typecast::add(12,12);
    println!("TypeCasted Result:{:?}",results);
}
