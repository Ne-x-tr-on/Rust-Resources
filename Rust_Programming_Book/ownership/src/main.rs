fn main(){
    // {
    //     let _s = "hello";
    //     println!("Inner scope say: {_s}");
    // }
    // let _s = "hello";
    // println!("Outer scope say: {_s}");

    // let _x = 5;
    // let _y = x;
    // println!("{x}");

    // let _x = String::from("David");
    // let _y = _x;
    // println!("{_x}")

    // {
    // let mut _x = String::from("David ");
    // let mut _y = _x.clone();
    // _y.push_str("Kamau Manyeki ");
    // _y.push_str("The legend in a drivers seat...You drove ambitions");
    // println!("{_y}");
    // }

// {
//     let s = String::from("hello");
//     takes_ownership(s); 

//     let x = 5; 
//     makes_copy(x); 

    
//     fn takes_ownership(some_string: String) { 
//     println!("{some_string}");
//     } 
    
// fn makes_copy(some_integer: i32) { 
//     println!("{some_integer}");
// }

// }

// {
//     let s1 = String::from("Davd Kamau Manyeki");
//     let (s2, len) = calculate_length(s1);
//     println!("The length of '{s2}' is {len}.");

//   fn calculate_length(data: String) -> (String,usize){
//     let length = data.len();
//     (data,length)
//   }
   
// }
// {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{s1}' is {len}.");
    
//     fn calculate_length(s: &String) -> usize {
//      s.len()
// }

// }

    {
        let mut s = String::from("Hello ");
        change(&mut s);

        fn change(some_string: &mut String){
            some_string.push_str("David");
             println!("{some_string}");
        }
       
    }
    
}

