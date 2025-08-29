// Functions that takes Ownership
#[allow(unused)]
pub fn owner(){
  let s:String= String::from("Owner");
  // let b:String= String::from("Owner");
  take_return_ownership(s.clone());
  take_ownership(s);
  
}
#[allow(unused)]
pub fn take_ownership(s:String){
  // println!("{:?} has been taken and has not been returned",s);
}

#[allow(unused)]
pub fn take_return_ownership(s:String) -> String{
  s
}