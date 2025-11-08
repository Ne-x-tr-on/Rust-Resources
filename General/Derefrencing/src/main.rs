fn main() {
    println!("Hello, Here we dereference");

    // let no:i32 = 55;
    // println!("Value of params before mutation to zero: {}",no);
    // println!("\n");
    // mutate_no_to_zero(no);

    let mut no :i32 = 50;
    mutate_no_to_zero_deref(&mut no);

}

#[allow(unused)]
fn mutate_no_to_zero(mut param_no:i32){
    param_no = param_no*0;
    println!("Value of params after mutation to zero: {}",param_no);
    
}

fn mutate_no_to_zero_deref(mut param_no: &mut i32){
    println!("Value of params after mutation to zero: {}",param_no);
    *param_no = *param_no*0;
    println!("Value of params after mutation to zero: {}",param_no);
}