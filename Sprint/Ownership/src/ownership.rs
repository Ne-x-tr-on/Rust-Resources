#[allow(unused)]
pub fn ownership(){
    // Each value has one owner
    let s = String::from("RustCode");


    // Only one owner at a time
    let s1 = String::from("RustCode");
    // let s2 = s1;
    let s2 = s1.clone();
    // println!("{:?}",s1);

    // When the owner goes out of scope, the value is dropped
    {
      let s = String::from("Developer");
    }
}