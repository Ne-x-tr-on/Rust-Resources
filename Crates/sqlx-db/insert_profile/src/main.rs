fn main(){
    let mut name ="Newton".to_string();
    println!("First name {}",name);

    name.push_str(" Kamau");
    println!("Hey {}",name);

    let mut i:i32 = 1;

    // for na in name.split(" "){
    //     println!("Name is {} -> {}",i,na);
    //     i+=1;
    // }

    println!("Putting the values in a vector");

    let namesplit :Vec<&str> = name.split(" ").collect();
    println!("::{}",namesplit[0]);
    println!("Name at index of 0 {}",namesplit[0]);

    for characters in name.chars(){
        println!("{}",characters);
    }
}
    
