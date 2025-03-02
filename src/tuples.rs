fn main(){
    //tuples
    let human:(String,i32,bool) = ("Alice".to_string(),30,false);
    println!("{:?} is {:?} years old and is {:?}",human.0, human.1, human);

    let human2= ("Alice",30,false);
    println!("{:?} is {:?} years old and is {:?}",human2.0, human2.1, human2);
}