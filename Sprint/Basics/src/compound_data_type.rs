#[allow(unused)]
pub fn arrayfn(){
  let mut _cdt: [i32;10] = [1,2,3,4,5,6,7,8,9,10];
  // println!("{:?}",_cdt);
  
  // Simple Vec to Extend the Array
  let mut _cdtvec: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
  _cdtvec.extend(_cdt);
  // println!("{:?}",_cdtvec);
}

#[allow(unused)]
pub fn tupplesfn(){
  let user:(String,bool,i32) = ("Newton".to_string(),true,90);
  // println!("Username:{:?}\nIs_male:{:?}\nAge:{:?}",user.0,user.1,user.2);
}

#[allow(unused)]
pub fn slicesfn(){
  let array:[i32;5]= [1,2,3,4,5];
  let slice:&[i32]= &array[0..2];
  // println!("{:?}",slice);
}

#[allow(unused)]
pub fn str_slicefn(){
  let mut name:String = String::from("Nextron");
  let mut slice = &name[0..2];
  println!("Slice: {}", slice);
  name.push_str("-Newton Kamau"); 
  println!("Full name: {}", name);
}