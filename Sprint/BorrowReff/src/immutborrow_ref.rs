#[allow(unused)]
pub fn reffn(){

  let n:i32 = 10;
  let q = &n; // q is a reference of n the original data

  // println!("{:?}",q);
  // println!("{:?}",n);
}

#[allow(unused)]
 pub fn borrowfn(){
    let name = String::from("Sprint");
    let len = calc_len(&name);
    // println!("Value:{}\nLength:{}",name,len);
}

#[allow(unused)]
pub fn calc_len(d:&String) -> usize{
    d.len()
}